//! A sans-I/O, single-instance circuit-breaking core.
//!
//! [`Sigorta`] owns exactly one breaker's state. Every transition takes an explicit
//! `now: Instant` and returns a [`Decision`] whose variants carry the resulting
//! [`Sigorta`] forward. The core does not read the system clock, hold a keyed
//! collection of many breakers, or classify what counts as success or failure.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::time::{Duration, Instant};

/// The state of one circuit breaker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    /// Admitting every attempt, counting consecutive failures toward `threshold`.
    Closed { failures: u32 },
    /// Rejecting every attempt until `eligible_again_at`.
    Open { eligible_again_at: Instant },
    /// Exactly one trial probe outstanding.
    HalfOpen,
}

/// A caller-judged outcome of one admitted attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    /// The attempt succeeded.
    Success,
    /// The attempt failed.
    Failure,
}

/// The result of an admission check or a recorded [`Event`].
///
/// Every variant carries the resulting [`Sigorta`] forward — there is no separate
/// return value carrying state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub enum Decision {
    /// The attempt is admitted normally.
    Admitted(Sigorta),
    /// The attempt is admitted as the one outstanding trial probe.
    Probing(Sigorta),
    /// The attempt is rejected; retry no sooner than `retry_after`.
    Rejected {
        /// The breaker's state after this decision.
        core: Sigorta,
        /// How long the caller should wait before checking admission again.
        retry_after: Duration,
    },
}

/// One circuit breaker's state and configuration.
///
/// `Sigorta` owns exactly one breaker. Tracking many breakers (for example, one per
/// job kind) is the caller's concern: hold a keyed collection of `Sigorta` values
/// outside this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sigorta {
    failure_threshold: u32,
    open_duration: Duration,
    state: State,
}

impl Sigorta {
    /// Construct a new breaker in the closed state.
    ///
    /// `failure_threshold` is the number of consecutive failures that trips the
    /// breaker. `open_duration` is how long the breaker stays open before admitting
    /// one trial probe.
    #[must_use]
    pub const fn new(failure_threshold: u32, open_duration: Duration) -> Self {
        Self {
            failure_threshold,
            open_duration,
            state: State::Closed { failures: 0 },
        }
    }

    /// Check whether an attempt is admitted at `now`, without recording an outcome.
    pub fn admit(self, now: Instant) -> Decision {
        match self.state {
            State::Closed { .. } => Decision::Admitted(self),
            State::Open { eligible_again_at } => {
                if now >= eligible_again_at {
                    let probing = Self {
                        state: State::HalfOpen,
                        ..self
                    };
                    Decision::Probing(probing)
                } else {
                    Decision::Rejected {
                        retry_after: eligible_again_at - now,
                        core: self,
                    }
                }
            }
            State::HalfOpen => Decision::Probing(self),
        }
    }

    /// Record the outcome of an admitted attempt at `now`.
    ///
    /// Calling this while the breaker is open is a caller precondition violation —
    /// no attempt could have been admitted to produce an outcome — and returns the
    /// breaker unchanged.
    #[must_use]
    pub fn record(self, event: Event, now: Instant) -> Self {
        match self.state {
            State::Closed { failures } => match event {
                Event::Success => Self {
                    state: State::Closed { failures: 0 },
                    ..self
                },
                Event::Failure => {
                    let failures = failures + 1;
                    if failures >= self.failure_threshold {
                        Self {
                            state: State::Open {
                                eligible_again_at: now + self.open_duration,
                            },
                            ..self
                        }
                    } else {
                        Self {
                            state: State::Closed { failures },
                            ..self
                        }
                    }
                }
            },
            State::Open { .. } => self,
            State::HalfOpen => match event {
                Event::Success => Self {
                    state: State::Closed { failures: 0 },
                    ..self
                },
                Event::Failure => Self {
                    state: State::Open {
                        eligible_again_at: now + self.open_duration,
                    },
                    ..self
                },
            },
        }
    }

    /// Whether this breaker is currently closed (admitting normally).
    #[must_use]
    pub const fn is_closed(&self) -> bool {
        matches!(self.state, State::Closed { .. })
    }

    /// Whether this breaker is currently open (rejecting).
    #[must_use]
    pub const fn is_open(&self) -> bool {
        matches!(self.state, State::Open { .. })
    }

    /// Whether this breaker currently has one outstanding trial probe.
    #[must_use]
    pub const fn is_half_open(&self) -> bool {
        matches!(self.state, State::HalfOpen)
    }
}
