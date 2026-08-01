## 1. Workspace publish metadata

- [x] 1.1 Add `repository`, `keywords`, `categories`, and `rust-version` to
      `[workspace.package]`, and add `tianheng` to `[workspace.dependencies]`.
- [x] 1.2 Set `sigorta-contract`'s `publish` to `true` and add its `repository`,
      `keywords`, `categories`, `rust-version`, and `readme` fields.
- [x] 1.3 Write `crates/sigorta-contract/README.md`.

## 2. Facade crate

- [x] 2.1 Create `crates/sigorta` with a `Cargo.toml` depending only on
      `sigorta-contract`, `publish = true`.
- [x] 2.2 Write `crates/sigorta/src/lib.rs` as a glob re-export of
      `sigorta_contract::*` with crate-level docs and a doc-tested usage example.
- [x] 2.3 Write `crates/sigorta/README.md`.

## 3. Governance crate

- [x] 3.1 Create `crates/sigorta-governance` with a `Cargo.toml` depending only on
      `tianheng`, unpublished.
- [x] 3.2 Write the `Constitution` in `crates/sigorta-governance/src/main.rs`:
      dependency boundaries for `sigorta-contract` (none), `sigorta` (only
      `sigorta-contract`), and `sigorta-governance` (only `tianheng`); and a
      `must_not_call_inline("std::time").ending_with(["now"])` boundary on
      `sigorta-contract`.
- [x] 3.3 Write `crates/sigorta-governance/README.md`.

## 4. Wire into Definition of Done and CI

- [x] 4.1 Update `AGENTS.md`'s Definition of Done to add `cargo doc`, the
      governance check (`cargo run -p sigorta-governance -- check --manifest-path
      Cargo.toml`), and the dogfood example run. (Already present from PR 1,
      written directly under the new name — confirm it matches this change's
      actual crate names.)
- [x] 4.2 Update `.github/workflows/ci.yml` to add doc build, the dogfood example
      run, and a governance job, matching `AGENTS.md`. Drop the now-permanently-dead
      "no real crate yet" conditional, since a real crate exists from here on.

## 5. Verification

- [x] 5.1 Run the full Definition of Done from the workspace root; fix any
      failures.
- [x] 5.2 Confirm `cargo run -p sigorta-governance -- check --manifest-path
      Cargo.toml` passes and actually reacts if a boundary is violated
      (spot-check by temporarily breaking one, observing the failure, then
      reverting).

## 6. Sync

- [x] 6.1 Delete the `openspec/changes/add-facade-and-governance/` directory (no
      capability spec changes to merge into `openspec/specs/`).
