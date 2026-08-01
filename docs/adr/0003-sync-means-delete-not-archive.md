# ADR 0003: Sync Means Delete, Not Archive

## Status

Accepted

## Context

ADR 0002 and this template's own `AGENTS.md`/`docs/development-flow.md` described the OpenSpec
lifecycle as ending in "implemented, verified, synced, and archived," with a fifth **Archive**
step that moves a completed change to `openspec/changes/archive/YYYY-MM-DD-<name>/`. This is not
this template's actual intended convention — every project derived from it (this repository's own
sibling starters, and every project built from them) has independently rediscovered and corrected
the same drift: sync should merge verified delta specs into `openspec/specs/` *and delete* the
change directory in the same step. There is no populated archive folder; the deletion itself,
plus the squash-merged pull request that carried it, is the durable record — git history already
keeps the deliberation.

## Decision

The OpenSpec lifecycle is four steps: `explore -> propose -> apply -> sync`. Sync deletes the
completed change directory as part of merging its delta specs — never run `openspec archive`, and
never introduce an `openspec/changes/archive/` folder.

## Consequences

- `AGENTS.md`, `docs/development-flow.md`, and ADR 0002 all describe the four-step lifecycle
  consistently; none references a fifth **Archive** step or an archive folder.
- A future agent or contributor who reaches for `openspec archive` should stop and re-read this
  ADR rather than assume the CLI's default behavior is this project's convention.
- Every project derived from this starter inherits the corrected convention from its first commit,
  instead of needing to rediscover and fix the same drift independently.
