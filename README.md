# Sigorta

A thin, sans-I/O circuit-breaking core for Rust: given an explicit clock reading and a
caller-judged stream of success/failure events, it adjudicates whether to admit an
attempt, admit it as a trial probe, or reject it with a retry-after duration.

`Sigorta` owns exactly one breaker's state at a time. It does not own a keyed
collection of many breakers, the system clock, or judgment about what counts as
success or failure — see `PROJECT.md` for the full Core Contract and Terminology, and
`docs/naming.md` for why the state names are the industry-standard
`Closed`/`Open`/`HalfOpen` rather than a themed register.

## Crates

- [`sigorta-contract`](crates/sigorta-contract) - the pure core: the state machine, its
  transitions, and its decision vocabulary. Zero dependencies.
- [`sigorta`](crates/sigorta) - the curated public entrypoint. A pure re-export of
  `sigorta-contract`; depend on this one.
- `sigorta-governance` - unpublished. Executable Tianheng architecture governance for
  this workspace.

## Development

This repository uses OpenSpec (`AGENTS.md`) and Conventional Commits with
squash-merged pull requests. See `AGENTS.md` and `docs/development-flow.md` for the
full workflow, and `BACKLOG.md` for settled and deferred decisions.

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo deny check
cargo run -p sigorta-governance -- check --manifest-path Cargo.toml
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
