//! Covers every scenario in `openspec/changes/initial-project-shape/specs/circuit-breaking/spec.md`.

use std::time::{Duration, Instant};

use sigorta_contract::{Decision, Event, Sigorta};

fn threshold_2_cooldown_10s() -> (Sigorta, Instant) {
    (Sigorta::new(2, Duration::from_secs(10)), Instant::now())
}

#[test]
fn closed_state_admits_normally() {
    let (breaker, t0) = threshold_2_cooldown_10s();

    match breaker.admit(t0) {
        Decision::Admitted(after) => assert!(after.is_closed()),
        other => panic!("expected Admitted, got {other:?}"),
    }
}

#[test]
fn recorded_success_resets_failure_count() {
    let (breaker, t0) = threshold_2_cooldown_10s();

    let breaker = breaker.record(Event::Failure, t0);
    assert!(breaker.is_closed());
    let breaker = breaker.record(Event::Success, t0);
    assert!(breaker.is_closed());

    // A single subsequent failure must not trip a breaker whose count was reset.
    let breaker = breaker.record(Event::Failure, t0);
    assert!(breaker.is_closed());
}

#[test]
fn reaching_failure_threshold_trips_the_breaker() {
    let (breaker, t0) = threshold_2_cooldown_10s();

    let breaker = breaker.record(Event::Failure, t0);
    assert!(
        breaker.is_closed(),
        "one failure must not trip a threshold-2 breaker"
    );

    let breaker = breaker.record(Event::Failure, t0);
    assert!(
        breaker.is_open(),
        "second consecutive failure must trip the breaker"
    );

    match breaker.admit(t0 + Duration::from_secs(9)) {
        Decision::Rejected { retry_after, .. } => {
            assert_eq!(retry_after, Duration::from_secs(1));
        }
        other => panic!("expected Rejected, got {other:?}"),
    }
}

#[test]
fn open_state_rejects_before_cooldown_elapses() {
    let (breaker, t0) = threshold_2_cooldown_10s();
    let breaker = breaker
        .record(Event::Failure, t0)
        .record(Event::Failure, t0);
    assert!(breaker.is_open());

    match breaker.admit(t0 + Duration::from_secs(5)) {
        Decision::Rejected { retry_after, core } => {
            assert_eq!(retry_after, Duration::from_secs(5));
            assert!(core.is_open());
        }
        other => panic!("expected Rejected, got {other:?}"),
    }
}

#[test]
fn cooldown_elapsed_transitions_to_a_trial_probe() {
    let (breaker, t0) = threshold_2_cooldown_10s();
    let breaker = breaker
        .record(Event::Failure, t0)
        .record(Event::Failure, t0);
    assert!(breaker.is_open());

    match breaker.admit(t0 + Duration::from_secs(10)) {
        Decision::Probing(after) => assert!(after.is_half_open()),
        other => panic!("expected Probing at the exact eligible-again moment, got {other:?}"),
    }

    let breaker = breaker.admit(t0 + Duration::from_secs(10));
    let Decision::Probing(breaker) = breaker else {
        panic!("expected Probing");
    };

    match breaker.admit(t0 + Duration::from_secs(20)) {
        Decision::Probing(after) => assert!(after.is_half_open()),
        other => panic!("expected Probing once cooldown has elapsed, got {other:?}"),
    }
}

#[test]
fn successful_probe_closes_the_breaker() {
    let (breaker, t0) = threshold_2_cooldown_10s();
    let breaker = breaker
        .record(Event::Failure, t0)
        .record(Event::Failure, t0);
    let Decision::Probing(breaker) = breaker.admit(t0 + Duration::from_secs(10)) else {
        panic!("expected Probing");
    };

    let breaker = breaker.record(Event::Success, t0 + Duration::from_secs(10));
    assert!(breaker.is_closed());

    match breaker.admit(t0 + Duration::from_secs(10)) {
        Decision::Admitted(_) => {}
        other => panic!("expected Admitted after a closed reset, got {other:?}"),
    }
}

#[test]
fn failed_probe_reopens_the_breaker() {
    let (breaker, t0) = threshold_2_cooldown_10s();
    let breaker = breaker
        .record(Event::Failure, t0)
        .record(Event::Failure, t0);
    let Decision::Probing(breaker) = breaker.admit(t0 + Duration::from_secs(10)) else {
        panic!("expected Probing");
    };

    let probe_failed_at = t0 + Duration::from_secs(10);
    let breaker = breaker.record(Event::Failure, probe_failed_at);
    assert!(breaker.is_open());

    match breaker.admit(probe_failed_at + Duration::from_secs(9)) {
        Decision::Rejected { retry_after, .. } => {
            assert_eq!(retry_after, Duration::from_secs(1));
        }
        other => panic!("expected a fresh cooldown, got {other:?}"),
    }
}

#[test]
fn admission_during_an_outstanding_probe_probes_again() {
    let (breaker, t0) = threshold_2_cooldown_10s();
    let breaker = breaker
        .record(Event::Failure, t0)
        .record(Event::Failure, t0);
    let Decision::Probing(breaker) = breaker.admit(t0 + Duration::from_secs(10)) else {
        panic!("expected Probing");
    };
    assert!(breaker.is_half_open());

    match breaker.admit(t0 + Duration::from_secs(11)) {
        Decision::Probing(after) => assert!(after.is_half_open()),
        other => panic!("expected Probing again, got {other:?}"),
    }
}

#[test]
fn recording_while_open_is_a_no_op() {
    let (breaker, t0) = threshold_2_cooldown_10s();
    let breaker = breaker
        .record(Event::Failure, t0)
        .record(Event::Failure, t0);
    assert!(breaker.is_open());

    let breaker = breaker.record(Event::Success, t0);
    assert!(
        breaker.is_open(),
        "recording while open must not change state"
    );
}
