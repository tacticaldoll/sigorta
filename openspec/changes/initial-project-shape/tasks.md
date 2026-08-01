## 1. Crate scaffolding

- [x] 1.1 Add `crates/sigorta-contract` to the workspace `Cargo.toml` members.
- [x] 1.2 Create `crates/sigorta-contract` as a dependency-free library crate with
      `#![forbid(unsafe_code)]` and `#![warn(missing_docs)]`.

## 2. Core contract implementation

- [x] 2.1 Implement the `Closed`/`Open`/`HalfOpen` state representation, holding a
      failure count in `Closed` and an eligible-again `Instant` in `Open` and
      `HalfOpen`.
- [x] 2.2 Implement `Sigorta::new` with a configurable failure threshold and open
      duration.
- [x] 2.3 Implement the closed-state admission and failure-accumulation requirement
      (spec: "Closed state admits and accumulates failures").
- [x] 2.4 Implement the open-state rejection and cooldown-expiry requirement (spec:
      "Open state rejects until cooldown elapses").
- [x] 2.5 Implement the half-open trial-probe requirement (spec: "Half-open state
      admits exactly one trial probe"), including re-admitting the outstanding probe
      on a repeated admission check before its outcome is recorded.
- [x] 2.6 Implement the `Event` enum (`Success`, `Failure`) and the `Decision` enum
      (`Admitted`, `Probing`, `Rejected { core, retry_after }`), with `admit`
      consuming `self` by value and returning `Decision`, and `record` consuming
      `self` by value and returning `Sigorta` directly (spec: "Admission checks
      consume ownership...", "Recording consumes ownership...", "Closed event
      vocabulary").
- [x] 2.7 Ensure no method in the crate reads the system clock; every method that
      needs time takes `now: Instant` explicitly (spec: "Explicit clock input").

## 3. Verification

- [x] 3.1 Add unit tests covering every scenario in
      `specs/circuit-breaking/spec.md`, including threshold-boundary and
      cooldown-boundary cases.
- [x] 3.2 Add a dependency-free dogfood example under
      `crates/sigorta-contract/examples/` that exercises a scenario shaped like the
      real-world evidence (a keyed collection of `Sigorta` values, admission checks,
      and recorded outcomes) entirely within this crate's own types, with no
      dependency on any other project.
- [x] 3.3 Run `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`,
      `cargo fmt --all --check`, `cargo deny check`, and `./scripts/changelog-guard.sh`
      from the workspace root; fix any failures.

## 4. Project metadata

- [x] 4.1 Replace `PROJECT.md`'s placeholder Vision, Core Contract, and Terminology
      sections with Sigorta's own, using the names settled in `docs/naming.md`.
- [x] 4.2 ~~Add a `CHANGELOG.md` entry for this change.~~ Not applicable:
      `CHANGELOG.md` itself states version entries are written only as part of a
      release-preparation pull request, never accumulated per feature change. This
      change is not a release.

## 5. Sync

- [x] 5.1 Move `specs/circuit-breaking/spec.md` into `openspec/specs/circuit-breaking/spec.md`.
- [x] 5.2 Delete the `openspec/changes/initial-project-shape/` directory in the same
      commit.
