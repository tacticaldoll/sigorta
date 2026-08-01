## Why

Attempting a real adoption (replacing a consumer application's hand-rolled circuit
breaker with `sigorta`) surfaced a real behavior gap: that hand-rolled evidence bounds
how long a single outstanding trial probe may go unreported before a fresh probe is
issued, so a probe holder that dies (or never calls back) cannot wedge a kind in
`HalfOpen` forever. `Sigorta`'s original distillation dropped this as unnecessary
complexity — an admission check during an outstanding probe just returns `Probing`
again, unconditionally, with no way out if the probe never resolves. That was a real
gap in the original distillation, not a stylistic simplification that survived contact
with the evidence it was drawn from.

## What Changes

- `HalfOpen` gains a bound: once a probe has been outstanding longer than the
  breaker's own `open_duration` (the evidence reuses the same duration for both the
  cooldown and the probe window — there is no separate config value), the next
  admission check treats it as stale and issues a fresh probe instead of returning
  the same indefinite `Probing` forever.
- No public API signature changes. `Sigorta::new`'s parameters are unchanged; `Event`
  and `Decision` are unchanged. This is a corrected internal transition rule, not a
  new capability surface.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `circuit-breaking`: the "Half-open state admits exactly one trial probe"
  requirement's "Admission during an outstanding probe" scenario is corrected — it
  previously specified admitting the same probe unconditionally forever; it now
  specifies a bound after which a fresh probe replaces the stale one.

## Impact

- Affected code: `crates/sigorta-contract/src/lib.rs` (the `HalfOpen` variant and
  `admit`'s `HalfOpen` arm), its tests, and its dogfood example.
- No dependency changes.
- This is a behavior correction, not a breaking API change — ships as `0.1.1`.
