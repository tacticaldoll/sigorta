## Context

`rust-version = "1.88"` was set by copying a sibling project's precedent at
`add-facade-and-governance` time, without checking whether this project's own code
needed it. It does not: `cargo +1.85.0 build --workspace` succeeds once the
declared floor is lowered.

## Goals / Non-Goals

**Goals:** declare the MSRV this project's code actually needs, verified by
building against it directly, not copied from a sibling.

**Non-Goals:** none — this is a single metadata correction.

## Decisions

### D1 — 1.85, verified by building against it

`1.85` is edition 2024's own minimum. Verified locally by temporarily removing
`rust-version` and running `cargo +1.85.0 build --workspace`: succeeds. No newer
language or standard-library feature is used anywhere in this workspace.

### D2 — Add an MSRV CI job now, not later

This is not a speculative addition: the exact failure mode (a declared MSRV nobody
actually built against) just happened for real, in a real consumer's CI, one
adoption cycle after this project shipped its first release. That is forcing
pressure, not a hypothetical. The job mirrors the consumer's own `msrv` job
precisely — same toolchain-pin pattern, same governance-crate exclusion rationale
(an unpublished dev tool is not part of the published-crate MSRV contract).

## Risks / Trade-offs

- **The MSRV job adds CI time for a one-crate-family project.** → Accepted: the
  job is cheap (a `cargo check`, not a full test run) and directly prevents a
  failure mode that already happened once.
