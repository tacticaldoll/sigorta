# AGENTS.md

Meta-guideline for any AI coding agent working in this repository. Read this first,
then let `openspec/specs/` and active change specs be the source of durable
architecture truth.

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
6. **Sibling-blind**: this repository names no other product in its own governed
   prose (`PROJECT.md`, `AGENTS.md`, `BACKLOG.md`, specs, or code comments). Which
   products it might one day be composed with is a consumer's knowledge, not this
   repository's.

## Document Authority

- `openspec/specs/` is shipped architecture truth.
- `openspec/changes/` contains active proposed truth until it is synced.
- `PROJECT.md` states product vision, positioning, and non-goals.
- `BACKLOG.md` records settled and deferred decisions and candidate patterns, not
  mandatory phases.
- `AGENTS.md` is operating protocol for agents and contributors.

Decision provenance lives in git — the commit body and pull request that made a
change record its rationale. Forward-looking or reversed decisions are noted in
`BACKLOG.md`. There is no separate architecture-decision-record file class; the
living documents above are the single source of truth for current state, and git is
the source of truth for why it changed.

If these documents conflict, fix the conflict through an OpenSpec change before
implementing feature code.

## Adversarial Review Stance

When reading proposals or reviewing code, actively challenge the design:

- **Propose phase**: Does the change make Sigorta heavier than a single-instance,
  sans-I/O state machine requires? Does it smuggle keyed storage, an ambient clock
  read, or a second responsibility into the core? Does it add openness (a trait, a
  growing enum) without a real, forcing instance behind it?
- **Apply phase**: Does the implementation leak an ambient clock read, a keyed
  collection, or a judgment about what counts as success or failure into the core?
  Does a new public decision type carry `#[must_use]`, so silently discarding it
  cannot compile clean?

Reject or redesign changes that pull Sigorta toward a broader runtime.

## This Project Uses OpenSpec

The source of truth lives in `openspec/`, which is version-controlled and
agent-agnostic.

- `openspec/specs/` - the living specification of what the system currently is.
- `openspec/changes/` - active change proposals as delta specs.

Per-agent command files such as `.codex/`, `.claude/`, and editor-specific shims are
per-clone generated files and are not committed. After cloning, generate your own
with:

```bash
openspec init --tools codex
# or: openspec init --tools claude,cursor,github-copilot
```

## Workflow

Follow this lifecycle:

```text
explore -> propose -> apply -> sync
```

1. **Explore**: think and investigate only. Do not write feature code outside of a
   change.
2. **Propose**: create a change with `proposal.md`, `design.md`, `tasks.md`, and delta
   specs. Commit as `docs(<change>): propose <summary>`.
3. **Apply**: implement tasks one at a time, checking each off in `tasks.md` only
   after verification. Commit coherent compiling milestones as `feat(...)` or
   `fix(...)`.
4. **Sync**: merge verified delta specs into `openspec/specs/`, then delete the
   completed change directory in the same commit. There is no
   `openspec/changes/archive/` folder — see `BACKLOG.md`'s Settled Decisions. Never
   run `openspec archive`. Commit as `docs(specs): sync <change>`.

A single change's propose, apply, and sync commits stay on one branch and land as one
squash-merged pull request — a change is not submitted for review until it is fully
synced.

## OpenSpec CLI

If your agent has no OpenSpec slash commands, use the CLI:

```bash
openspec list [--json] [--specs]
openspec new change "<name>"
openspec status --change "<name>" --json
openspec instructions <artifact> --change "<name>"
```

## Rules

- Before implementing anything, read the relevant files in `openspec/specs/` and the
  active change's artifacts.
- Do not write feature code without an active change proposal that contains tasks.
- Keep changes minimal and scoped to the task being implemented.
- Treat `openspec/specs/` as the truth. Reflect requirement changes there via the
  sync step, not by editing code silently.
- Keep project-specific contract, terms, and priorities in `PROJECT.md`.

## Language

- Write OpenSpec artifacts, code comments, and commit messages in English.
- Converse with users in the language they use.

## Commit And Integration Governance

### Branch Commits

- Use Conventional Commits: `type(scope): summary`.
- Write the subject in English, lowercase imperative mood, at no more than 72 characters.
- Use the body to record motivation, important decisions, constraints, and verification when that context exists.
- Do not append pull request or issue numbers to the subject or body.
- Development branches may contain multiple coherent commits because the pull request is squash-merged.

### Pull Requests

- Branch from `main` and open every change directly against `main`.
- Make the pull request title the intended squash commit subject.
- Give every pull request a non-empty body that explains why the change is needed, what changed, consequential decisions or tradeoffs, and verification.
- Rebase the branch onto the current `main` before final verification.
- Do not introduce a release integration branch between a change and `main`.

### Squash Merges

- Squash-merge every verified pull request into `main`.
- Make the squash commit subject exactly the approved pull request title.
- Give every squash commit a non-empty body distilled from the approved pull request body.
- Do not append a pull request number, issue number, or URL to the squash subject or body.
- Every content-changing commit on `main`, including release preparation, must come from a squash-merged pull request.
- Keep `main` releasable after every merge.

### Attribution

- Do not include AI, agent, model, tool, automation, or generation attribution in commits, pull requests, tags, changelogs, or release notes.
- A `Co-authored-by` trailer is allowed only for a real human contributor.

### Release Finalization

- Prepare release content (including the `CHANGELOG.md` entry) in a pull request whose squash subject is exactly `chore(release): prepare X.Y.Z`.
- Give the release preparation squash commit a non-empty body describing scope, compatibility, metadata changes, and verification.
- Run the complete Definition of Done after that commit reaches `main`.
- Finalize with annotated tag `vX.Y.Z` on that commit, with message exactly `release: X.Y.Z`. Push the tag alone — no accompanying GitHub Release object, matching this project's tag-only convention.
- Push the tag without another commit. Release branches and empty release commits are not part of the flow.

### Changelog

- Follow [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and [Semantic Versioning](https://semver.org/spec/v2.0.0.html) in `CHANGELOG.md`.
- Do not accumulate an `[Unreleased]` section between releases — write each version's entry as part of its own release-preparation pull request, cross-checked against the actual commit history.
- Every `## [X.Y.Z]` version heading must have a matching `[X.Y.Z]: <url>` footer link pointing at `.../releases/tag/vX.Y.Z`. `scripts/changelog-guard.sh` checks this mechanically and runs as part of the Definition of Done.

## Definition Of Done

Run these from the workspace root before checking off a task or syncing specs:

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
```

Before the first real crate exists, the Rust commands are not yet meaningful. The
first project-specific OpenSpec change should add the real crate layout and make the
Definition of Done runnable from the workspace root. `./scripts/changelog-guard.sh`
and `cargo deny check` are runnable immediately and always run.

If a command cannot run in the current environment, report that explicitly.
