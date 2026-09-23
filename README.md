# Sigorta

A thin, sans-I/O circuit-breaking core for Rust: given an explicit clock reading and a
caller-judged stream of success/failure events, it adjudicates whether to admit an
attempt, admit it as a trial probe, or reject it with a retry-after duration.

## Scope

`Sigorta` owns exactly one breaker's state at a time. It does not own a keyed
collection of many breakers, the system clock, or judgment about what counts as
success or failure — see `PROJECT.md` for the full Core Contract, and
`docs/domain-language.md` for the Terminology and for why the state names are the
industry-standard `Closed`/`Open`/`HalfOpen` rather than a themed register.

## Architecture

- [`sigorta-contract`](crates/sigorta-contract) - the pure core: the state machine, its
  transitions, and its decision vocabulary. Zero dependencies.
- [`sigorta`](crates/sigorta) - the curated public entrypoint. A pure re-export of
  `sigorta-contract`; depend on this one.
- `sigorta-governance` - unpublished. Executable Tianheng architecture governance for
  this workspace.

## Contributing

This repository uses OpenSpec (`AGENTS.md`) and Conventional Commits with
squash-merged pull requests. See `AGENTS.md` and `docs/development-flow.md` for the
full workflow, including the Definition of Done, and `BACKLOG.md` for settled and deferred
decisions.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
