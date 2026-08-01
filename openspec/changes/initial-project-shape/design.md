## Context

A real circuit breaker already exists inside a consumer application's worker runtime.
Its shape: a policy of `failure_threshold` and `open_duration`, states kept in a
`HashMap` keyed by job kind, a three-state machine (closed / open / half-open), an
`admit` check and a `record` outcome call. Its states and transition rules are sound;
its impurity is that the transition logic reads the wall clock internally instead of
receiving time as an argument, and its state machine is entangled with the per-kind
keying that is really a caller concern.

That real instance is treated as evidence a reusable mechanism exists, not as the
literal shape to port. Sigorta's job is to distill the mechanism the evidence points at,
prove it correct and complete on its own, and let any real adoption happen later as a
separate, later decision — this repository's existence does not depend on that adoption
happening.

## Goals / Non-Goals

**Goals:**
- A pure, `Instant`-explicit, single-instance circuit-breaking state machine.
- A closed decision vocabulary that answers exactly one question per call: admit
  normally, admit as a probe, or reject with a wait duration.
- A dogfood example proving the contract holds for a scenario shaped like the real
  evidence, with no dependency on the application that motivated it.

**Non-Goals:**
- No per-key/per-kind collection inside this crate. Managing many breakers is a
  caller concern.
- No trait-based extension point for what counts as success or failure. The caller
  has already judged the outcome before this crate sees it.
- No facade crate, no governance crate, no publish preparation in this change. Those
  are separate, later decisions gated on their own triggers.
- No dependency on, or reference to, any other project in this family by name, in any
  committed file.

## Decisions

### D1 — Explicit time, no ambient clock

Every method that can transition state takes `now: Instant` as an explicit parameter.
The real evidence calls `Instant::now()` internally; that is the one concrete impurity
this design removes. Alternative considered: keep an internal clock read for caller
convenience — rejected, since an ambient read is exactly what a sans-I/O core cannot
do, and the fix is mechanical, not costly.

### D2 — Single-instance ownership

`Sigorta` owns exactly one breaker's state. There is no `HashMap<K, Sigorta>` inside
this crate. Alternative considered: bake in keyed multi-instance storage, matching the
real evidence's own shape directly — rejected, because keying by job kind is a
property of one specific consumer's use case, not of the mechanism itself.

### D3 — Ownership-consuming transitions; only `admit` needs a decision wrapper

Both `admit` and `record` consume `self` by value. `admit` returns a `Decision` whose
variants carry the resulting `Sigorta` forward, rather than a separate
`(Sigorta, Decision)` tuple:

```rust
pub enum Decision {
    Admitted(Sigorta),
    Probing(Sigorta),
    Rejected { core: Sigorta, retry_after: Duration },
}
```

`record` returns a bare `Sigorta`, not a `Decision`. Recording an outcome answers no
admission question — `Rejected` has no meaning for an outcome that already happened —
so wrapping it in `Decision` would be a misfit. Only `admit` needed the wrapper;
`record` only needed ownership to flow through, which a bare return value already
does.

`Probing` is a deliberate departure from the real evidence: there, a half-open trial
admit is indistinguishable from a normal admit to the caller. Here it is a first-class,
observable outcome, because a caller that cannot tell it is on a trial probe cannot
reasonably decide how to treat a subsequent failure differently from a routine one. A
repeated `admit` call while a probe is already outstanding returns `Probing` again,
unchanged, rather than introducing a second timer field to track probe expiry.

Alternative considered: `admit() -> Option<Duration>`, mirroring the real evidence
exactly — rejected, since it would make this crate a renamed copy rather than a
distilled mechanism, and it hides a fact the mechanism already computes internally.

### D4 — `Event` is a closed enum, not a trait

```rust
pub enum Event {
    Success,
    Failure,
}
```

The caller judges success or failure before calling into this crate; this crate only
combines already-judged events into a decision. Alternative considered: a trait so
callers could classify domain errors themselves — rejected. The real evidence shows
exactly one instance of this concern, already resolved as a plain boolean before
reaching the breaker; there is no second, differing instance forcing a richer shape. A
trait here would mean this crate reaching into a judgment it has no basis to make, and
would only invite inconsistent vocabulary across implementations with no offsetting
benefit.

### D5 — Crate layout: contract only, for now

Only `sigorta-contract` is added in this change. No `sigorta` facade crate (added only
once a publish is actually prepared) and no `sigorta-governance` crate (added only once
a concrete governance need exists).

### D6 — Vocabulary: settled as industry-standard

`Closed`, `Open`, `HalfOpen` for state; `Success`/`Failure` for events;
`Admitted`/`Probing`/`Rejected` for decisions. Full reasoning recorded in
`docs/naming.md`: these names are already precise and specific to this mechanism, so
there is no bad-default vocabulary to escape, and the ongoing translation cost of a
themed alternative would not be buying back any protection against semantic drift.

## Risks / Trade-offs

- **Distilling too early, with only one real instance as evidence** → the mechanism
  described here has not been validated against a second, differing real use.
  Mitigation: this change deliberately does not add speculative generality beyond what
  D1–D4 already require; nothing here anticipates a second consumer's shape.
- **`Probing` as a new, observable outcome has no precedent in the real evidence** →
  a caller adapting the real evidence's own call site to this contract will need to
  decide what to do with `Probing` that it previously never saw. Mitigation: this is an
  intentional, documented improvement, not an oversight; it is exercised directly in the
  dogfood example.
