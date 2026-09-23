# Project Contract

Keep this short and concrete; it is the orientation layer for humans and AI agents.

## Vision

Sigorta is a thin, sans-I/O circuit-breaking core. Given an explicit clock reading and a
caller-judged stream of success/failure events, it adjudicates whether to admit an
attempt, admit it as a trial probe, or reject it with a retry-after duration.

## Product Positioning

Sigorta owns exactly one breaker's state at a time; it does not own a keyed collection of many
breakers, the system clock, or judgment about what counts as success or failure.

## Core Contract

The behavior that must be protected first:

- **No ambient clock reads.** Every transition takes `now: Instant` explicitly.
- **Single-instance ownership.** A `Sigorta` value is exactly one breaker.
  Multi-instance management is entirely the caller's concern, never expressed inside
  this crate.
- **Ownership flows through the result.** `admit` consumes `self` and returns a
  `Decision` whose variants carry the resulting `Sigorta` forward; `record` consumes
  `self` and returns the resulting `Sigorta` directly.
- **Closed judgment vocabulary.** `Event` is a plain, closed enum (`Success`,
  `Failure`) supplied by the caller — never a trait the core uses to classify an
  outcome itself.

## Terminology

`docs/domain-language.md` is the canonical vocabulary, including the terms `Sigorta`, `Closed`,
`Open`, `HalfOpen`, `Event`, `Decision`, and `Probing`.

## Non-Goals

This repository is intentionally narrow:

- Sigorta is not a worker runtime, a retry framework, or an orchestration platform.
- Keyed multi-instance management, the system clock, and integration with any real caller are
  the caller's concern, never the core's identity.

## First Project Change

`initial-project-shape` establishes the `sigorta-contract` crate and its
circuit-breaking contract. See `openspec/specs/circuit-breaking/spec.md` for the full
specification.

## Change Prioritization

When comparing possible changes, prefer the one that protects the core contract
earliest:

1. Correctness, data integrity, lifecycle safety, and security foundations.
2. Specified feature completeness for concepts already declared in OpenSpec.
3. Operator and developer ergonomics.
4. Scale-out, integrations, and optional platform features.

Do not add scale-out or integration scope merely because a correctness change
enables it. Keep enabling contract changes separate and small.

## References

- `openspec/specs/circuit-breaking/spec.md` — the full circuit-breaking specification.
- `docs/domain-language.md` — the state vocabulary and naming decisions.
- `BACKLOG.md` — settled and deferred decisions.
- `AGENTS.sigorta-law.md` — the generated projection of the accepted constitution.
