## Context

The real-world evidence this project was distilled from bounds a `HalfOpen` probe's
lifetime: `probe_expires`, computed from the same `open_duration` used for the
`Open` cooldown, so a probe that never reports back (its holder crashed, or simply
never called back) does not wedge the kind in `HalfOpen` indefinitely — the next
admission check past that bound issues a fresh probe instead.

The original distillation (`initial-project-shape`) dropped this, reasoning that a
repeated admission check during an outstanding probe could just return `Probing`
again, unconditionally — framed as avoiding "a second timer field" for the sake of
simplicity. Attempting the real adoption this project exists to eventually support
showed that reasoning was wrong: this is not incidental complexity, it is the
mechanism that keeps the breaker from getting stuck. The evidence already carried it;
the first distillation pass just missed it.

## Goals / Non-Goals

**Goals:**
- `HalfOpen` bounds how long a single probe may stay outstanding, reusing
  `open_duration` — no new constructor parameter, matching the evidence's own choice
  to reuse one duration for both purposes rather than adding a second config value.
- An admission check past that bound issues a fresh probe (a new bound extended from
  `now`), rather than only being resolved by an eventual `record` call.

**Non-Goals:**
- A separate, independently configurable probe-timeout duration distinct from
  `open_duration`. The evidence does not have one; inventing one now would be
  speculative generality with no forcing instance behind it.
- Any change to `Sigorta::new`'s signature, `Event`, or `Decision`'s shape. This is a
  transition-rule correction inside `HalfOpen`, not a new capability.
- Distinguishing, in `Decision`, a fresh probe issued after a stale one from the
  original probe. The evidence does not expose this distinction to its own callers
  either — both cases are simply "admitted, treat cautiously."

## Decisions

### D1 — Reuse `open_duration`, not a new parameter

`HalfOpen`'s bound is computed as `now + open_duration` at the moment a probe is
issued (both the first time, transitioning from `Open`, and again if a check finds
the previous bound already passed). This mirrors the evidence's own `cooldown_end`
helper being shared by both the `Open.until` and `HalfOpen.probe_expires`
computations verbatim — the evidence deliberately did not carry two separate
duration values, and this project should not invent one where the mechanism it was
drawn from never needed it.

Alternative considered: add a distinct `probe_timeout: Duration` constructor
parameter, letting a caller tune the two independently — rejected. Nothing in the
evidence, or in any other real use, calls for tuning them separately; adding the
parameter now would be exactly the kind of unforced openness this project's own
`AGENTS.md` rejects elsewhere (the `Event`-as-trait decision, `docs/naming.md`'s
vocabulary decision).

### D2 — This is "user obligation" is the wrong frame; this is mechanism

Whether a probe is "too old" is not a domain judgment call the way "did this attempt
succeed" is — it is a pure comparison of `now` against a stored deadline, computed
identically to how `Open → Probing` already works. It belongs inside the core for
the same reason the cooldown-elapsed check already does, not externalized to the
caller as an injected duration or a trait.

## Risks / Trade-offs

- **Silent behavior change** → callers already depending on `0.1.0`'s indefinite
  `HalfOpen` wait get different behavior on upgrade. Mitigation: this is a bug fix
  matching the evidence's own real, load-bearing behavior, not a design preference
  change; it ships as `0.1.1` with the corrected scenario documented in
  `CHANGELOG.md` and the synced spec.
