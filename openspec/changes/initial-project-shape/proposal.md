## Why

Sigorta exists to prove that a reusable circuit-breaking mechanism can stand as an
independent, sans-I/O core: a real, hand-rolled circuit breaker already lives inside a
consumer application's worker runtime, entangled with a keyed collection and an ambient
clock read. That instance is evidence a reusable mechanism exists, not the shape to copy
verbatim. This change gives the mechanism its first home: a pure, single-instance state
machine proven correct on its own, before any real consumer depends on it.

## What Changes

- Replace `PROJECT.md`'s placeholder vision, core contract, and terminology with
  Sigorta's own: a sans-I/O circuit-breaking core.
- Add the real Rust crate layout: a single `sigorta-contract` pure core crate. No
  facade crate and no governance crate yet — both are added later, only once a
  publish or a governance need actually materializes.
- Define the pure state/event/decision contract, using the industry-standard
  vocabulary settled in `docs/naming.md` (`Closed`/`Open`/`HalfOpen`,
  `Success`/`Failure`, `Admitted`/`Probing`/`Rejected`):
  - A `Sigorta` value owns exactly one breaker's state at a time. No internal keyed
    collection — multi-instance management (e.g. one breaker per job kind) is entirely
    the caller's concern, expressed outside this crate.
  - Every transition takes `now: Instant` as an explicit argument. No ambient clock
    read anywhere in the crate.
  - A transition consumes the current `Sigorta` by value and returns a `Decision`
    whose variants carry the new state forward, rather than a separate
    `(state, decision)` pair.
  - `Event` is a plain, closed, two-variant enum (`Success`, `Failure`) supplied by the
    caller — not a trait. The caller has already judged the outcome; this crate only
    combines that judgment into a decision.
- Add a dogfood example that exercises the contract against a scenario shaped like the
  real evidence, with zero dependency on the application that motivated it — proving the
  contract stands on its own.
- Make `cargo build`, `cargo test`, `cargo clippy`, `cargo fmt`, and `cargo deny check`
  meaningful from the workspace root.

## Capabilities

### New Capabilities

- `circuit-breaking`: a sans-I/O core that adjudicates whether to admit an attempt,
  admit it as a trial probe, or reject it with a retry-after duration, given an explicit
  clock reading and a caller-supplied stream of success/failure events.

### Modified Capabilities

None — this is the first specified capability in this repository.

## Impact

- Affected code: entire repository (currently governance-only). Adds
  `crates/sigorta-contract/`.
- Affected docs: `PROJECT.md` (placeholder replaced), `CHANGELOG.md` (entry added).
- Dependencies: none added — the contract crate stays dependency-free.
- No external consumer is touched or depended upon by this change.
