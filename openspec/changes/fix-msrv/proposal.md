## Why

`rust-version = "1.88"` was copied from a sibling project's precedent without
verifying this project's own code actually needs it. Attempting real adoption into
a consumer application whose own MSRV is `1.85` failed CI's MSRV check outright:
`rustc 1.85.0 is not supported... requires rustc 1.88`. Verified locally (with
`rust-version` temporarily removed): `cargo +1.85.0 build --workspace` succeeds —
nothing in this project's code needs anything past edition 2024's own floor.

## What Changes

- Lower `rust-version` from `1.88` to `1.85` at the workspace level (inherited by
  `sigorta-contract` and `sigorta`).
- Add an `msrv` CI job that builds against the declared floor directly (mirroring
  the consumer application's own `msrv` job, which is what caught this), excluding
  `sigorta-governance` (an unpublished dev tool, not part of the published-crate
  MSRV contract) — so a future MSRV drift like this one fails in this project's own
  CI instead of only in a consumer's.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None. This is a metadata correction with no behavior or requirement change.

## Impact

- Affected code: `Cargo.toml` only.
- No dependency, API, or behavior changes.
- Ships as `0.1.3` (metadata-only, no breaking change).
