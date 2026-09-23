# Sigorta Tianheng Law Projection

This file is generated from `constitution()` in `crates/sigorta-governance/src/main.rs`.
The Rust declaration is authoritative; do not edit the projection by hand.
Regenerate it with `BLESS=1 cargo test -p sigorta-governance law_projection_is_fresh`.

# Constitution: sigorta

## Static boundaries

### `sigorta-contract` (crate)

> sigorta-contract is the isolated core contract: a sans-I/O, single-instance circuit-breaking state machine. It needs no dependency at all, so it may depend on nothing.

- **rule**: restrict dependencies to (only: )
- **kind**: crate · **severity**: enforce

### `sigorta` (crate)

> sigorta is the curated public entrypoint: a pure re-export facade with no logic of its own. It must depend on sigorta-contract only, never acquiring a dependency the core itself does not have.

- **rule**: restrict dependencies to (only: sigorta-contract)
- **kind**: crate · **severity**: enforce

### `sigorta-governance` (crate)

> the governance gate must stay independent of the workspace graph it judges: it may depend only on tianheng, never on sigorta-contract or any other workspace crate under judgment.

- **rule**: restrict dependencies to (only: tianheng)
- **kind**: crate · **severity**: enforce

### `sigorta-contract::crate` (module)

> the sans-I/O core reads no wall clock: every transition takes now: Instant as an explicit argument. A call to std::time::Instant::now or std::time::SystemTime::now anywhere in sigorta-contract is exactly the impurity this repository's real-world evidence had, and this project exists to remove it.

- **rule**: inline symbol path confined to module (confined_prefix: std::time; ending_with: now)
- **kind**: module · **severity**: enforce · **crate**: sigorta-contract
