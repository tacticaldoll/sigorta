## Why

Adopting `sigorta` into the real consumer application this project was distilled
from surfaced a genuine correctness bug, not a missing feature: `admit`'s half-open
branch admits *every* admission check while a probe is outstanding, rather than
admitting only the probe itself and rejecting further callers until it resolves or
its window expires. This inverts the evidence's real behavior. The evidence's own
`HalfOpen` branch rejects a caller while the probe is still within its window, and
only admits again once that window has passed without a recorded outcome — this
project's original distillation got the condition backwards from the first change
that introduced `HalfOpen` at all.

## What Changes

- `admit`'s half-open branch is corrected: while `now` is before the outstanding
  probe's bound, the decision is now `Rejected` with `retry_after` equal to the
  remaining time until that bound — not `Probing` again. Once `now` reaches or
  passes the bound with no outcome recorded, the decision is `Probing`, admitting a
  fresh probe with a renewed bound — this half was already correct.
- No public API signature changes. `Sigorta::new`, `Event`, and `Decision`'s shape
  are unchanged.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `circuit-breaking`: the "Half-open state admits exactly one trial probe, bounded"
  requirement's "still within its window" scenario is corrected from admitting every
  check to rejecting all but the original probe.

## Impact

- Affected code: `crates/sigorta-contract/src/lib.rs` (`admit`'s `HalfOpen` arm), its
  tests.
- No dependency changes.
- This is a correctness fix, not a breaking API change — ships as `0.1.2`.
