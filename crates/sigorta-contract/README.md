# sigorta-contract

The isolated core contract for Sigorta: a sans-I/O, single-instance circuit-breaking
state machine. Given an explicit clock reading and a caller-judged stream of
success/failure events, it adjudicates whether to admit an attempt, admit it as a
trial probe, or reject it with a retry-after duration.

`Sigorta` owns exactly one breaker's state. Tracking many breakers (for example, one
per job kind) is the caller's concern — see `examples/keyed_by_job_kind.rs` for a
dependency-free demonstration.

Zero dependencies; `sigorta-governance` enforces that it declares no normal dependency. See the
workspace root `PROJECT.md` for the full Core Contract, and `docs/domain-language.md` for the
Terminology and for why the state names are the industry-standard `Closed`/`Open`/`HalfOpen` rather
than a themed register.

Part of [Sigorta](https://github.com/tacticaldoll/sigorta).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/sigorta/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/tacticaldoll/sigorta/blob/main/LICENSE-MIT), at your option.
