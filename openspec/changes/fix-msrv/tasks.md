## 1. Fix

- [x] 1.1 Lower `rust-version` to `1.85` in `[workspace.package]`.
- [x] 1.2 Verify `cargo +1.85.0 build --workspace` succeeds.
- [x] 1.3 Add an `msrv` CI job (`dtolnay/rust-toolchain@1.85.0`,
      `cargo check --workspace --exclude sigorta-governance --all-targets`),
      matching the consumer application's own job.

## 2. Sync

- [ ] 2.1 No capability spec changes to sync.
- [ ] 2.2 Delete the `openspec/changes/fix-msrv/` directory in the same commit.

## 3. Release

- [ ] 3.1 Prepare and publish `0.1.3` in a separate `chore(release)` pull request.
