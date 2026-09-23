# sigorta-governance

Executable architectural governance for the [Sigorta](https://github.com/tacticaldoll/sigorta)
workspace: a [Tianheng](https://crates.io/crates/tianheng) constitution that mechanically
enforces the normal-dependency boundaries between `sigorta-contract`, `sigorta`, and this crate
itself, and that `sigorta-contract`'s library makes no inline `std::time` `now` call. A clock read
a source scan cannot see, such as one through a method on a value like `Instant::elapsed`, stays
review-governed; the accepted boundary reasons in `AGENTS.sigorta-law.md` state each perimeter.

Unpublished — this gate has no reason to exist outside this workspace's own CI.

```bash
cargo run -p sigorta-governance -- check --manifest-path Cargo.toml
```

The accepted constitution is projected into `AGENTS.sigorta-law.md` at the repository root, a
generated file that `law_projection_is_fresh` byte-checks. Regenerate it after a deliberate,
reviewed law change:

```bash
BLESS=1 cargo test -p sigorta-governance law_projection_is_fresh
```
