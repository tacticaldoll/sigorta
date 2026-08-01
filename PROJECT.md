# Project Contract

Keep this short and concrete; it is the orientation layer for humans and AI agents.

## Purpose

Sigorta is a thin, sans-I/O circuit-breaking core. Given an explicit clock reading and a
caller-judged stream of success/failure events, it adjudicates whether to admit an
attempt, admit it as a trial probe, or reject it with a retry-after duration. It owns
exactly one breaker's state at a time; it does not own a keyed collection of many
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

- `Sigorta` — one breaker's configuration and state.
- `Closed` / `Open` / `HalfOpen` — the three states (see `docs/naming.md` for why these
  industry-standard names were kept rather than replaced with a themed register).
- `Event` — a caller-judged outcome of one admitted attempt (`Success` or `Failure`).
- `Decision` — the result of an admission check: `Admitted`, `Probing`, or `Rejected`.
- `Probing` — an admission granted as the one outstanding trial after a cooldown
  elapses, distinguished from a normal `Admitted` so the caller can treat it cautiously.

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
