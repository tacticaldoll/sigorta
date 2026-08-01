# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Do not accumulate an `[Unreleased]` section between releases. Write each version's entry as part
of its own release-preparation pull request (see `AGENTS.md`'s Release Finalization section),
cross-checked against the actual commit history rather than written from memory. Every `## [X.Y.Z]`
version heading must have a matching `[X.Y.Z]: <url>` footer link — `scripts/changelog-guard.sh`
checks this mechanically.

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

[0.1.0]: https://github.com/tacticaldoll/sigorta/releases/tag/v0.1.0
