# AGENTS.md

Meta-guideline for AI coding agents and contributors working in this repository. Read this first,
then let `openspec/specs/` and active change specs be the source of durable architecture truth.

## Sigorta In One Sentence

Sigorta is a thin, sans-I/O circuit-breaking core: a single-instance state machine that
adjudicates admission from an explicit clock reading and caller-judged outcomes.

This repository is intentionally narrow. Sigorta is not a worker runtime, a retry
framework, or an orchestration platform. Keyed multi-instance management, the system
clock, and integration with any real caller are the caller's concern, never the
core's identity.

## Architectural Axioms

Before proposing or writing code, protect these axioms:

1. **The core stays thin**: `sigorta-contract` owns exactly one breaker's state and its
   transition rules. It does not own a keyed collection of many breakers, the system
   clock, retries, or any backend-specific behavior.
2. **Time is explicit**: every transition takes `now: Instant` as an explicit
   argument. No method anywhere in the core reads the ambient clock.
3. **Ownership flows through decisions**: a transition consumes the current state by
   value; the resulting decision's own variants carry the new state forward, never a
   separate return value.
4. **Judgment stays outside**: the core accepts only an already-judged, closed
   outcome vocabulary from the caller. It does not classify errors, and it exposes no
   trait for a caller to inject that judgment.
5. **Vocabulary is governance**: state and decision names are settled deliberately,
   recorded in `BACKLOG.md`, never introduced piecemeal or defaulted silently.

## Lineage

```text
   tianheng  +  〔sans-I/O · OpenSpec · vocabulary-as-governance · least-commitment〕
                    │  inherited discipline — provenance, not coupling
                    ▼
             ●  sigorta

   siblings: ▢ ▢ ▢   intentionally blank — this repo is sibling-blind. Which
                     products compose together is a consumer app's knowledge, never
                     a component's; naming a sibling here would leak that knowledge
                     and rot when the roster changes.
   note: skeleton from tacticaldoll/rust-family-template.
```

Sigorta shares a **discipline** with its lineage, not code: its own crates, specs, constitution, and
release cadence. It does not import, track, or depend on any sibling product, and its governed
prose (`PROJECT.md`, `AGENTS.md`, `BACKLOG.md`, specs, and code comments) names none.

## Document Authority

- `openspec/specs/` is shipped architecture truth.
- `openspec/changes/` contains active proposed truth until it is synced.
- `PROJECT.md` states product vision, positioning, and non-goals.
- `docs/domain-language.md` is the canonical vocabulary.
- `BACKLOG.md` records settled and deferred decisions, open design questions, and candidate
  patterns, not mandatory phases.
- `AGENTS.md` is operating protocol for agents and contributors.
- `AGENTS.sigorta-law.md` is the generated, freshness-gated projection of the accepted Rust
  constitution in `crates/sigorta-governance`. The constitution is authoritative; read the
  projection after this file, regenerate it with its documented command, and never edit it by
  hand.
- Other files under `docs/` elaborate one topic each and yield to the documents above.

Decision provenance lives in git — the commit body and pull request that made a change record its
rationale. Forward-looking or reversed decisions are noted in `BACKLOG.md`. There is no separate
architecture-decision-record file class; the living documents above are the single source of
truth for current state, and git is the source of truth for why it changed.

If these documents conflict, fix the conflict through an OpenSpec change before implementing
feature code.

## Adversarial Review Stance

Every change passes an adversarial review at BOTH the propose and apply phases before it is
committed. Actively challenge the design:

- **Propose phase**: Does the change make Sigorta heavier than a single-instance,
  sans-I/O state machine requires? Does it smuggle keyed storage, an ambient clock
  read, or a second responsibility into the core? Does it add openness (a trait, a
  growing enum) without a real, forcing instance behind it?
- **Apply phase**: Does the implementation leak an ambient clock read, a keyed
  collection, or a judgment about what counts as success or failure into the core?
  Does a new public decision type carry `#[must_use]`, so silently discarding it
  cannot compile clean?

Reject or redesign changes that pull Sigorta toward a broader runtime.

## Governance and Conformance

Sigorta separates the *judgment* from the *check on its projection*.

- **Governance is judgment, and lives in prose** — `openspec/specs/`, this file, `PROJECT.md`,
  and `BACKLOG.md`. Intent and meaning are decided here and stay review-governed.
- **Code is the projection of a judgment onto the structural plane** — a `pub use` set, an absent
  `async fn`, a dependency edge, a missing trait bound.
- **Conformance verifies the projection still matches the judgment.** It is a family: Tianheng
  (structure, dependencies, source scans), `rustc` (type facts), and tests (behavior). They bite
  the projection, never the judgment itself.

Tianheng's accepted constitution projects into `AGENTS.sigorta-law.md`; a freshness test byte-checks
that generated context against the live declaration, so accepted law is visible without a second
hand-maintained authority. A green gate means "no visible violation", not proof: a judgment that
casts no structural shadow stays prose, and a source scan cannot see what a macro expands to.

Before turning a judgment into a Tianheng tooth, it must pass four gates — casting a shadow is
necessary, not sufficient:

1. **Shadow** — does the judgment project into a syntactically decidable structural fact? (No →
   it stays prose and review.)
2. **Faithful** — is that fact a faithful proxy, not a gameable one? (Lines of code are not
   thinness; a proxy invites Goodhart.)
3. **Stable** — is the judgment stable? A tooth on a moving projection is a recurring maintenance
   tax and a second copy of the truth; prefer a test.
4. **Sync** — is the extra `prose ⟷ tooth` coupling worth it? The tooth is itself a *second
   projection* of the judgment, and nothing mechanically checks it matches the prose — only
   review does. The regress terminates in a human.

Fail any gate and the honest home is prose, review, or a test — never a faked tooth. A tooth
complements review; it never replaces it. Where an accepted boundary does hold a claim, its
reason is the single statement of that rule, and prose that merely restated it may be retired.

Repair code toward a violated reason; never weaken a law, baseline new drift, or change severity
merely to make a check green. A deliberate law change requires explicit authority, focused
violating and clean reaction proofs, projection regeneration, and adversarial review.

## OpenSpec Workflow

`openspec/` is the version-controlled, agent-neutral source of truth: `openspec/specs/` is the
living specification of what the system is, and `openspec/changes/` holds active change proposals
as delta specs. Per-agent command files (`.claude/`, `.codex/`, editor shims) are generated per
clone and never committed; generate your own with `openspec init --tools <tool>`.

The lifecycle is:

```text
explore -> propose -> apply -> sync
```

1. **Explore**: investigate and shape intent. Read the relevant `openspec/specs/` first. Do not
   write feature code outside a change.
2. **Propose**: `openspec new change "<change>"`, then write `proposal.md`, `design.md`,
   `tasks.md`, and delta specs with success, failure, and edge scenarios. Commit as
   `docs(<change>): propose <summary>`.
3. **Apply**: implement against the active delta specs, one task at a time, and check a task off
   only after the Definition of Done passes. Keep changes minimal and scoped; never bundle
   unrelated work. Commit coherent compiling milestones as `feat(...)` or `fix(...)`.
4. **Sync**: merge verified delta specs into `openspec/specs/` (agent-driven — the CLI has no sync
   command), then `git rm -r openspec/changes/<change>/`. There is no archive: the change's
   content now lives in `openspec/specs/` and git history. Never run `openspec archive`. Commit
   as `docs(specs): sync <change>`.

Requirement changes reach `openspec/specs/` through sync, never through silent code edits.
Without agent slash commands, use the CLI:

```bash
openspec list [--json] [--specs]
openspec new change "<change>"
openspec status --change "<change>" --json
openspec instructions <artifact> --change "<change>"
```

## Language

- Write OpenSpec artifacts, `BACKLOG.md` entries, code comments, and commit messages in English.
- Converse with users in the language they use.
- Wrap Markdown prose near 100 columns; tables and code blocks are exempt.

## Commit And Integration Governance

### Branch Commits

- Use Conventional Commits: `type(scope): summary`.
- Write the subject in English, lowercase imperative mood, at no more than 72 characters.
- Use the body to record motivation, important decisions, constraints, and verification when that
  context exists. Do not merely enumerate changed files.
- Do not append pull request or issue numbers to the subject or body.
- Development branches may contain multiple coherent commits because the pull request is
  squash-merged.

### Pull Requests

- Branch from `main` and open every change directly against `main`.
- Make the pull request title the intended squash commit subject.
- Give every pull request a non-empty body that explains why the change is needed, what changed,
  consequential decisions or tradeoffs, and verification.
- Rebase the branch onto the current `main` before final verification.
- Do not introduce a release integration branch between a change and `main`.

### Squash Merges

- Squash-merge every verified pull request into `main`.
- Make the squash commit subject exactly the approved pull request title. Hosting tools append
  the pull request number by default; remove it.
- Give every squash commit a non-empty, self-describing body distilled from the approved pull
  request body: preserve durable rationale, decisions, constraints, and verification; omit
  transient checklists and generated commit lists.
- Do not append a pull request number, issue number, or URL to the squash subject or body.
- Every content-changing commit on `main`, including release preparation, must come from a
  squash-merged pull request.
- Keep `main` releasable after every merge.

### Attribution

- Do not include AI, agent, model, tool, automation, or generation attribution in commits, pull
  requests, tags, changelogs, or release notes.
- Prohibited forms include AI `Co-authored-by` trailers, `generated by`, `written with`, model or
  agent names used as signatures, and tool signatures.
- A `Co-authored-by` trailer is allowed only for a real human contributor.

### Changelog

- `CHANGELOG.md` follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and
  [Semantic Versioning](https://semver.org/spec/v2.0.0.html), and is a strict release ledger: it
  has no `[Unreleased]` section. Unreleased work is recorded in OpenSpec changes, pull requests,
  and `BACKLOG.md`.
- Write each version's entry in its own release-preparation pull request, cross-checked against
  the commit history since the previous release.
- Every `## [X.Y.Z] - YYYY-MM-DD` heading has a matching `[X.Y.Z]: <url>` footer link to
  `.../releases/tag/vX.Y.Z`. `scripts/changelog-guard.sh` checks this and runs in the Definition
  of Done.

### Release Finalization

- Prepare release content in a pull request whose squash subject is exactly
  `chore(release): prepare X.Y.Z`.
- Sweep crate-level README files and other non-governed prose for stale version markers or
  disposition language that `BACKLOG.md` has since resolved, superseded, or placed downstream.
- Give the release preparation squash commit a non-empty body describing scope, compatibility,
  metadata changes, and verification.
- Run the complete Definition of Done after that commit reaches `main`.
- Publish crates in dependency order, waiting for each to appear in the crates.io index before
  publishing its dependents. If an upload's result is uncertain, query crates.io for the exact
  version before retrying — a published version cannot be overwritten.
- Finalize with annotated tag `vX.Y.Z` on that commit, with message exactly `release: X.Y.Z`.
- Push the tag without another commit. Release branches and empty release commits are not part
  of the flow.

## Release Tag Convention

Sigorta's releases are tag-only: push the annotated `vX.Y.Z` tag alone, with no accompanying
GitHub Release object.

## Pull Request Timing

A change's pull request opens only once the change is fully synced, never as a propose-only
pull request; `BACKLOG.md`'s Settled Decisions records this rule.

## Definition Of Done

Run these from the workspace root before checking off implementation tasks or syncing specs. This
is the single source for the gate list — `README.md` and `docs/development-flow.md` point here
rather than restating it. If a command cannot run in the current environment, report that
explicitly.

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo deny check
cargo run -p sigorta-governance -- check --manifest-path Cargo.toml
cargo run --example keyed_by_job_kind -p sigorta-contract
./scripts/changelog-guard.sh
cargo +1.88 build --workspace
cargo +1.85 check --workspace --exclude sigorta-governance --all-targets
```

CI (`.github/workflows/ci.yml`) runs the same gates on push and pull request. Rust style lives in
these checks: rustfmt formats, clippy denies warnings, rustdoc denies documentation warnings,
cargo-deny owns resolved supply-chain policy, and `sigorta-governance` owns Tianheng architecture
boundaries. The crates exist, so every command is meaningful from the workspace root.

Two gates are Sigorta's own. `keyed_by_job_kind` is the dependency-free dogfood example: it shows
a caller keeping many breakers while the core owns exactly one. The `+1.85` check holds the
published crates' own floor: the workspace builds on 1.88, but `sigorta-contract` and `sigorta`
declare `rust-version = "1.85"` (edition 2024's own floor), and a consumer on 1.85 must be able to
build them. `sigorta-governance` is unpublished and outside that floor.
