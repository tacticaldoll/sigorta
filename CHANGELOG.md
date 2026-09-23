# Changelog

This file is a ledger of released versions. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Normal development is recorded in OpenSpec changes, pull requests, and `BACKLOG.md`, not in an
`[Unreleased]` section. Each version's entry is written in its own release-preparation pull
request (see `AGENTS.md`'s Release Finalization).

## [0.1.3] - 2026-08-01

Metadata correction, no code changes.

### Fixed

- The declared MSRV was `1.88`, copied from a sibling project's precedent without
  verification. Lowered to `1.85` (edition 2024's own floor) — nothing in this
  workspace needs anything newer. Caught by a real consumer's CI (its own MSRV is
  `1.85`) during adoption.
- Added an `msrv` CI job so a future drift like this fails here directly, instead
  of only in a consumer.

## [0.1.2] - 2026-08-01

Correctness fix, no public API changes. Adopting `sigorta` into the real consumer
application it was distilled from surfaced a genuine bug, not a missing feature.

### Fixed

- A half-open trial probe that is still outstanding and within its window now
  correctly rejects every other admission check (with `retry_after` equal to the
  remaining time until the probe's bound), instead of admitting every check as
  `Probing`. Only once that window passes with no recorded outcome is a fresh probe
  admitted. This inverted condition dated back to the initial release, not a `0.1.1`
  regression — `0.1.1` added the bound correctly but never actually rejected anyone
  within it.

## [0.1.1] - 2026-08-01

Behavior correction, no public API changes. Attempting a real adoption (replacing a
consumer application's hand-rolled circuit breaker with `sigorta`) surfaced a real
gap the original distillation had dropped as unnecessary complexity.

### Fixed

- A single outstanding half-open trial probe no longer waits forever for its outcome
  to be recorded. Once it has been outstanding longer than the configured open
  duration, the next admission check now replaces it with a fresh probe rather than
  extending the same one indefinitely — matching the real-world evidence this
  project was distilled from, which bounds a probe's lifetime the same way and
  reuses the same duration for both the cooldown and the probe window.

## [0.1.0] - 2026-08-01

Initial release: a sans-I/O, single-instance circuit-breaking core, evidenced by a
real hand-rolled breaker in a consumer application's worker runtime rather than
copied from it. Published as a Tier 2 spike, proven by dogfooding and this
workspace's own governance and conformance gates, before any real bridge consumer
adopts it — see `BACKLOG.md`'s Settled Decisions for that deliberate departure from
this family's usual Tier 1 definition, recorded as a considered choice.

### Added

- `sigorta-contract`: `Sigorta`, a `Closed`/`Open`/`HalfOpen` state machine. Every
  transition takes an explicit `now: Instant`; `admit` returns a `Decision`
  (`Admitted`/`Probing`/`Rejected`) whose variants carry the resulting `Sigorta`
  forward; `record` consumes `self` and returns the resulting `Sigorta` directly.
  `Event` is a closed `Success`/`Failure` enum, not a trait.
- `sigorta`: the curated public entrypoint, a pure re-export of `sigorta-contract`.
- `sigorta-governance` (unpublished): a Tianheng constitution enforcing this
  workspace's dependency boundaries and that `sigorta-contract` never reads the
  ambient clock.
- `docs/naming.md`: records the vocabulary decision (industry-standard
  `Closed`/`Open`/`HalfOpen` over a themed register) and this project's own naming
  history (originally `Stoma`, renamed to `Sigorta` before any publish, after the
  bare `stoma` crate name was found already taken on crates.io).
- 9 unit tests covering every scenario in `openspec/specs/circuit-breaking/spec.md`,
  and a dependency-free dogfood example (`keyed_by_job_kind.rs`).

[0.1.3]: https://github.com/tacticaldoll/sigorta/releases/tag/v0.1.3
[0.1.2]: https://github.com/tacticaldoll/sigorta/releases/tag/v0.1.2
[0.1.1]: https://github.com/tacticaldoll/sigorta/releases/tag/v0.1.1
[0.1.0]: https://github.com/tacticaldoll/sigorta/releases/tag/v0.1.0
