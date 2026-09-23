# Sigorta Tianheng Law Projection

This file is generated from `constitution()` in `crates/sigorta-governance/src/main.rs`.
The Rust declaration is authoritative; do not edit the projection by hand.
Regenerate it with `BLESS=1 cargo test -p sigorta-governance law_projection_is_fresh`.

# Constitution: sigorta

## Static boundaries

### `sigorta-contract` (crate)

> sigorta-contract is the isolated core contract: a sans-I/O, single-instance circuit-breaking state machine. It needs no dependency at all, so it declares no normal dependency.

- **rule**: restrict dependencies to (only: )
- **kind**: crate · **severity**: enforce

### `sigorta` (crate)

> sigorta is the curated public entrypoint: its normal dependencies are sigorta-contract alone, never a normal dependency the core itself does not have. That the facade holds no logic of its own is review-governed, not observed here.

- **rule**: restrict dependencies to (only: sigorta-contract)
- **kind**: crate · **severity**: enforce

### `sigorta-governance` (crate)

> the governance gate must stay independent of the workspace graph it judges: its normal dependencies are tianheng alone, never sigorta-contract or any other workspace crate under judgment.

- **rule**: restrict dependencies to (only: tianheng)
- **kind**: crate · **severity**: enforce

### `sigorta-contract::crate` (module)

> sigorta-contract's library makes no inline `std::time` `now` call, such as std::time::Instant::now or std::time::SystemTime::now, in any of its modules: every transition takes now: Instant as an explicit argument, because an ambient clock read is exactly the impurity this repository's real-world evidence had, and this project exists to remove it. Coverage is partial by nature (a clock read through a method on a value, such as `Instant::elapsed`, or a `now` path taken as a value rather than called, is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::time; ending_with: now)
- **kind**: module · **severity**: enforce · **crate**: sigorta-contract
