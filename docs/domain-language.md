# Naming Worldview

## Crate name

`Sigorta` (Turkish, "fuse, circuit breaker") is the one foreign-language flavor word
for this project. Unlike some sibling projects, whose crate name is a metaphor for
their mechanism, `sigorta` is not a metaphor — it is the everyday household word for
the physical device this crate models. (This project was originally named `Stoma`,
Greek for "mouth, opening" — a metaphor rather than a literal match — until the bare
`stoma` crate name was found already taken on crates.io by an unrelated crate before
any publish happened. See `BACKLOG.md`'s Settled Decisions for the full account.)

## Terminology

- `Sigorta` — one breaker's configuration and state.
- `Closed` / `Open` / `HalfOpen` — the three states (see below for why these industry-standard
  names were kept rather than replaced with a themed register).
- `Event` — a caller-judged outcome of one admitted attempt (`Success` or `Failure`).
- `Decision` — the result of an admission check: `Admitted`, `Probing`, or `Rejected`.
- `Probing` — an admission granted as the one outstanding trial after a cooldown
  elapses, distinguished from a normal `Admitted` so the caller can treat it cautiously.

## Internal state vocabulary

Sigorta's public state vocabulary is `Closed`, `Open`, and `HalfOpen` — the
industry-standard names for this exact mechanism. This crate does not invent a
themed register for these three states.

### Reasoning

Inventing a vocabulary is a defense against a *bad default*: a generic or
adjacent-domain word set that would quietly pull implementers toward the wrong
mental model. `Closed`/`Open`/`HalfOpen` are not that. They are already precise
and specific to this mechanism, not generic words borrowed from an unrelated
domain — there is no competing, misleading default vocabulary for this
mechanism to escape.

The cost of replacing them is real and ongoing: every future reader, log line,
dashboard, and code review pays a translation tax back to the standard terms
for the lifetime of this crate. That cost only earns its keep when the
replacement is protecting against actual semantic drift. Here it would not be
— it would be decoration layered on top of vocabulary that already does its
job.

`Event` and `Decision` follow the same reasoning: `Success`/`Failure` and
`Admitted`/`Probing`/`Rejected` are plain, descriptive, and already unambiguous
in context. No themed register is introduced for them either.

This is a considered choice, not an omission — a later change should not
introduce themed renames without a concrete, forcing reason recorded in
`BACKLOG.md`.
