# sigorta-governance

Executable architectural governance for the [Sigorta](https://github.com/tacticaldoll/sigorta)
workspace: a [Tianheng](https://crates.io/crates/tianheng) constitution that mechanically
enforces the dependency boundaries between `sigorta-contract`, `sigorta`, and this crate itself,
and that `sigorta-contract` never reads the ambient clock.

Unpublished — this gate has no reason to exist outside this workspace's own CI.

```bash
cargo run -p sigorta-governance -- check --manifest-path Cargo.toml
```
