//! Sigorta: a thin, sans-I/O circuit-breaking core you compose.
//!
//! This crate is the curated public entrypoint. It re-exports the whole public API of
//! the Sigorta workspace so a consumer can depend on one crate:
//!
//! - the breaker itself — [`Sigorta`] and [`Sigorta::new`];
//! - a caller-judged outcome — [`Event`];
//! - the result of an admission check — [`Decision`].
//!
//! It carries no logic of its own: every item here is a re-export. Sigorta's whole
//! public surface is compose-level, so the facade withholds nothing — there is no
//! advanced kernel to reach through [`sigorta_contract`] directly.
//!
//! # The contract
//!
//! `Sigorta` owns exactly one breaker's state and its transition rules. It does not
//! own a keyed collection of many breakers, the system clock, or judgment about what
//! counts as success or failure — see the crate-level docs of [`sigorta_contract`] and
//! the workspace root `PROJECT.md` for the full Core Contract.
//!
//! # Tracking a breaker
//!
//! ```
//! use std::time::{Duration, Instant};
//!
//! use sigorta::{Decision, Event, Sigorta};
//!
//! let breaker = Sigorta::new(2, Duration::from_secs(10));
//! let now = Instant::now();
//!
//! let Decision::Admitted(breaker) = breaker.admit(now) else {
//!     panic!("a fresh breaker admits");
//! };
//! let breaker = breaker.record(Event::Success, now);
//! assert!(breaker.is_closed());
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

// A glob re-export makes "the facade withholds nothing" structurally true: the
// facade's surface *is* `sigorta-contract`'s public surface, enforced by the compiler
// in both directions. A new public item in the core appears here automatically; none
// can be silently dropped or left behind.
pub use sigorta_contract::*;
