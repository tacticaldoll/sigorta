## Why

`sigorta-contract` exists, is tested, and is dogfooded, but a crates.io publish needs a
curated facade crate and a mechanically enforced architecture boundary, matching how
this family's other published core-plus-facade projects are structured. This project's
owner has deliberately chosen to pursue publish now, before a real bridge consumer
exists — see `BACKLOG.md`'s Settled Decisions for that departure recorded as a
considered choice.

## What Changes

- Add a `sigorta` facade crate: a pure re-export of `sigorta-contract`'s public
  surface, with no logic of its own.
- Add a `sigorta-governance` crate: a Tianheng constitution that mechanically enforces
  this repository's architectural axioms — the dependency boundaries between
  `sigorta-contract`, `sigorta`, and `sigorta-governance` itself, and that
  `sigorta-contract` never reads the ambient clock (`std::time::*::now`).
- Add workspace-level publish metadata (`repository`, `keywords`, `categories`,
  `rust-version`) and per-crate `README.md` files, so `sigorta-contract` and `sigorta`
  are ready to publish.
- Wire the governance check, `cargo doc`, and the dogfood example into `AGENTS.md`'s
  Definition of Done and into CI.

This change does not run `cargo publish`. Publishing itself is a separate,
release-preparation pull request, per `AGENTS.md`'s Release Finalization convention.

## Capabilities

### New Capabilities

None. This change adds packaging, a public re-export surface, and mechanical
architecture governance — it does not change `circuit-breaking`'s requirements.

### Modified Capabilities

None.

## Impact

- Affected code: adds `crates/sigorta/`, `crates/sigorta-governance/`.
- Affected docs: `AGENTS.md` (Definition of Done), `.github/workflows/ci.yml`,
  `BACKLOG.md` (already recorded), per-crate `README.md` files, workspace
  `Cargo.toml`, root `README.md`.
- Dependencies: `sigorta-governance` depends on `tianheng` (published, `0.3.0`).
  Neither `sigorta-contract` nor `sigorta` gains a dependency.
- No external consumer is touched by this change.
