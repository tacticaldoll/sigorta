## 1. Core fix

- [ ] 1.1 Correct `admit`'s `HalfOpen` arm: reject (with `retry_after` equal to the
      remaining time until `probe_expires`) while `now` is before the bound; admit
      a fresh, renewed probe once `now` reaches or passes it.

## 2. Verification

- [ ] 2.1 Fix `admission_during_an_outstanding_probe_still_within_window_probes_again`:
      rename and correct its assertion to expect `Rejected`, not `Probing`.
- [ ] 2.2 Confirm `a_stale_outstanding_probe_is_replaced_with_a_fresh_one` still
      holds correctly against the corrected logic.
- [ ] 2.3 Run the full Definition of Done.

## 3. Sync

- [ ] 3.1 Apply the `MODIFIED Requirements` delta to
      `openspec/specs/circuit-breaking/spec.md`.
- [ ] 3.2 Delete the `openspec/changes/fix-half-open-admission/` directory in the
      same commit.

## 4. Release

- [ ] 4.1 Prepare and publish `0.1.2` in a separate `chore(release)` pull request.
