//! A worker-runtime-shaped bridge without any dependency on a worker runtime.
//!
//! Demonstrates the caller-owned wrapper this crate deliberately does not provide:
//! a keyed collection of one `Sigorta` per job kind, admission checks gating whether a
//! job attempt runs, and recorded outcomes driving the breaker through every state,
//! including the `Probing` trial after a cooldown elapses.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use sigorta_contract::{Decision, Event, Sigorta};

fn main() {
    let mut breakers: HashMap<&str, Sigorta> = HashMap::new();
    let policy = || Sigorta::new(2, Duration::from_secs(10));

    let t0 = Instant::now();

    // A healthy job kind: admitted, succeeds, stays closed.
    let breaker = breakers.remove("send-email").unwrap_or_else(policy);
    let Decision::Admitted(breaker) = breaker.admit(t0) else {
        panic!("a fresh breaker must admit");
    };
    let breaker = breaker.record(Event::Success, t0);
    assert!(breaker.is_closed());
    breakers.insert("send-email", breaker);

    // A failing job kind: two consecutive failures trip it.
    let breaker = breakers.remove("charge-card").unwrap_or_else(policy);
    let Decision::Admitted(breaker) = breaker.admit(t0) else {
        panic!("a fresh breaker must admit");
    };
    let breaker = breaker.record(Event::Failure, t0);
    assert!(breaker.is_closed(), "one failure of two must not trip it");

    let Decision::Admitted(breaker) = breaker.admit(t0) else {
        panic!("still closed, must admit");
    };
    let breaker = breaker.record(Event::Failure, t0);
    assert!(breaker.is_open(), "second consecutive failure must trip it");
    breakers.insert("charge-card", breaker);

    // The tripped kind rejects immediately after tripping.
    let breaker = breakers.remove("charge-card").unwrap();
    let (breaker, retry_after) = match breaker.admit(t0 + Duration::from_secs(1)) {
        Decision::Rejected { core, retry_after } => (core, retry_after),
        other => panic!("expected Rejected, got {other:?}"),
    };
    assert_eq!(retry_after, Duration::from_secs(9));
    breakers.insert("charge-card", breaker);

    // Once the cooldown elapses, the next admission check is a trial probe, not a
    // normal admit — the caller can tell the difference and treat it cautiously.
    let breaker = breakers.remove("charge-card").unwrap();
    let breaker = match breaker.admit(t0 + Duration::from_secs(10)) {
        Decision::Probing(core) => core,
        other => panic!("expected Probing, got {other:?}"),
    };

    // The trial succeeds: the breaker closes and forgets its failure history.
    let breaker = breaker.record(Event::Success, t0 + Duration::from_secs(10));
    assert!(breaker.is_closed());
    breakers.insert("charge-card", breaker);

    println!("all breakers settled: {} kinds tracked", breakers.len());
}
