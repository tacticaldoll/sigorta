## Context

`sigorta-contract` is a complete, tested, dogfooded Tier 2 spike. This change prepares
it for a crates.io publish by adding the two crates this family's other published
core-plus-facade projects add at that point: a curated facade and a Tianheng
governance crate — modeled directly on `cadw`'s equivalent crates, the closest sibling
precedent for a project at this exact scale.

## Goals / Non-Goals

**Goals:**
- A facade crate (`sigorta`) that re-exports `sigorta-contract`'s full public surface
  and nothing else.
- A governance crate (`sigorta-governance`) that mechanically enforces the dependency
  boundaries between the three crates, and that `sigorta-contract` never reads the
  ambient clock — the single sans-I/O property this repository has most explicitly
  committed to in `AGENTS.md`.
- Publish-ready metadata on `sigorta-contract` and `sigorta`.

**Non-Goals:**
- Running `cargo publish` itself. That is a separate release-preparation pull request.
- Any change to `circuit-breaking`'s requirements or public API shape.
- A prose stale-vocabulary governance check (as `cadw-governance` has). That exists to
  guard against a discarded working name resurfacing after a rename; this project's
  rename (Stoma to Sigorta) happened before any publish, and the old name lives only
  in `BACKLOG.md`'s historical record, not in any active prose file — adding that check
  now would be governance surface for a concern that does not exist.
- An I/O boundary reaction (`std::io`/`std::fs`/`std::net`/`std::process`) as a
  separate rule: `sigorta-contract` has zero dependencies and is a pure, synchronous
  state machine over primitive types, so there is no code path through which I/O could
  enter undetected.

## Decisions

### D1 — Facade is a glob re-export, nothing else

`sigorta`'s `src/lib.rs` is `pub use sigorta_contract::*;` plus crate-level docs. A
glob re-export makes "the facade withholds nothing" structurally true: a new public
item in the core appears here automatically, and the compiler — not a reviewer —
catches any item silently left behind. This matches `cadw`'s (and `suunta`'s and
`shaahid`'s) own facades exactly.

### D2 — Governance boundaries enforce exactly what AGENTS.md already claims

Four boundaries, each tied to a specific claim already made in prose:

1. `sigorta-contract` may depend on nothing (`AGENTS.md`: "The core stays thin").
2. `sigorta` may depend only on `sigorta-contract` (the facade is pure re-export).
3. `sigorta-governance` may depend only on `tianheng` (the gate stays independent of
   the graph it judges).
4. `sigorta-contract` must never call `std::time::*::now` (`AGENTS.md`: "Time is
   explicit... No method anywhere in the core reads the ambient clock"), narrowed with
   `.ending_with(["now"])` so the rule reacts on `Instant::now()`/`SystemTime::now()`
   calls specifically, not on ordinary use of `Duration`/`Instant` as types.

### D3 — No serde boundary, unlike some siblings

Some sibling cores mechanically forbid acquiring `serde::Serialize`/`Deserialize`.
`sigorta-contract` has no such prose claim to enforce (`AGENTS.md` never mentions
serialization), so adding the reaction now would be guarding a claim nobody made.

## Risks / Trade-offs

- **Publishing before a real consumer** → already recorded as a deliberate, considered
  choice in `BACKLOG.md`, not repeated here as a design risk to mitigate.
- **A facade adds a second published crate to keep in sync with the core** →
  mitigated structurally by D1: the glob re-export cannot drift from the core's public
  surface without a compile error.
