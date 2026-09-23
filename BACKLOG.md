# Backlog & Deferred Decisions

Records the plan, settled decisions, and deferred decisions so the repo can drive its
own development. Shipped truth lives in `openspec/specs/`; active proposed truth in
`openspec/changes/`.

## Settled Decisions

- **No standalone ADR practice; `docs/adr/` is dissolved.** The `rust-openspec-starter`
  template this repository was created from ships a three-file ADR skeleton, but that
  is not how this project records decisions: this file, plus git history, is the sole
  provenance trail — no separate architecture-decision-record file class. The
  template's three ADRs had already drifted into duplicating content that belongs in
  `AGENTS.md`: ADR 0003's content ("sync means delete, not archive") is already stated
  in `AGENTS.md`'s OpenSpec Workflow section; ADR 0002's Decision ("use OpenSpec as the source
  of truth") is already stated more fully by `AGENTS.md`'s OpenSpec Workflow
  section — its Context is the one part worth preserving here: chat history and
  agent-specific command shims are not a reliable source of truth for AI-assisted
  development, which is why this project's actual behavior lives in `openspec/specs/`
  instead. ADR 0001 (the decision to keep ADRs at all) is superseded outright by this
  entry. All three files were removed, and the references to them in `AGENTS.md`,
  `README.md`, and `docs/development-flow.md` were repointed here.
- **This repository is sibling-blind.** It names no other product in its own governed
  prose. Which products it might one day be composed with is a consumer's knowledge,
  not this repository's.
- **A change is not opened as a pull request until it is fully synced.** Propose,
  apply, and sync commits for one OpenSpec change stay on a single branch; the pull
  request that carries them is opened once the change's specs are already synced into
  `openspec/specs/` and its `openspec/changes/<name>/` directory is already deleted.
  There is no separate "propose-only" pull request.
- **Publish is pursued before a real bridge consumer exists — a deliberate departure
  from this family's usual Tier 1 definition, made knowingly.** Elsewhere in this
  family, a crate stays unpublished until a real consumer adopts it and a publish
  happens together with that adoption — dogfooding alone has never been treated as
  sufficient grounds to publish. This project's owner considered that convention and
  chose to publish `sigorta-contract` (and its facade) on the strength of its dogfood
  proof and its own conformance/governance gates, without waiting for a real bridge
  consumer first. This is recorded here so the deviation reads as a considered choice,
  not an oversight — a later reviewer should not assume a real consumer exists just
  because a release does.
- **This project was originally named Stoma (Greek, "mouth, opening"), then renamed to
  Sigorta before any publish happened.** `stoma`, `stoma-contract`, `stoma-governance`,
  and the `tacticaldoll/stoma` repository were fully built — governance, the
  circuit-breaking contract, tests, a dogfood example, a facade, and Tianheng
  governance — but `stoma` (the bare facade name) was found to already be taken by an
  unrelated crate on crates.io before any `cargo publish` was run. Rather than settle
  for a facade name that breaks this family's convention of the facade's crate name
  matching the project's own identity, the whole project was renamed and rebuilt
  before any external identity (a crates.io release) existed to break. `sigorta`
  (Turkish, "fuse, circuit breaker") is a strictly more literal fit than `stoma` was —
  `stoma`'s "mouth, opening" was a metaphor for the mechanism; `sigorta` is the
  everyday household word for the physical device this crate models. The old
  `tacticaldoll/stoma` repository was deleted; nothing published there, so no
  deprecation notice or yanked release is needed anywhere.

## Current Baseline

`sigorta-contract` (the pure core) exists, tested, and dogfooded — see
`openspec/specs/circuit-breaking/spec.md`. See
`openspec/changes/add-facade-and-governance/` for the active change adding the
`sigorta` facade, the `sigorta-governance` crate, and the metadata a crates.io
publish needs.
