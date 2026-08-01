## 1. Core implementation

- [ ] 1.1 Add a `probe_expires: Instant` field to `State::HalfOpen`.
- [ ] 1.2 When `Open` transitions to `HalfOpen` (cooldown elapsed), set
      `probe_expires` to `now + open_duration`.
- [ ] 1.3 In `admit`'s `HalfOpen` arm: if `now` is before `probe_expires`, return
      `Probing` with the state unchanged (as today); if `now` is at or after
      `probe_expires`, return `Probing` with a renewed `probe_expires` of
      `now + open_duration`.

## 2. Verification

- [ ] 2.1 Update `admission_during_an_outstanding_probe_probes_again` (or split it)
      to cover both the still-within-window and the stale-and-renewed cases.
- [ ] 2.2 Run the full Definition of Done.

## 3. Sync

- [ ] 3.1 Apply the `MODIFIED Requirements` delta to
      `openspec/specs/circuit-breaking/spec.md`.
- [ ] 3.2 Delete the `openspec/changes/add-probe-timeout/` directory in the same
      commit.

## 4. Release

- [ ] 4.1 Add a `CHANGELOG.md` `[0.1.1]` entry in a separate
      `chore(release): prepare 0.1.1` pull request, per `AGENTS.md`'s Release
      Finalization convention — not part of this change's own commits.
