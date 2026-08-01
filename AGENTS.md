# AGENTS.md

Meta-guideline for any AI coding agent working in this repository. Read this
first.

## This Project Uses OpenSpec

The source of truth lives in `openspec/`, which is version-controlled and
agent-agnostic.

- `openspec/specs/` - the living specification of what the system currently is.
- `openspec/changes/` - active change proposals as delta specs.

Per-agent command files such as `.codex/`, `.claude/`, and editor-specific shims
are per-clone generated files and are not committed. After cloning, generate
your own with:

```bash
openspec init --tools codex
# or: openspec init --tools claude,cursor,github-copilot
```

## Workflow

Follow this lifecycle:

```text
explore -> propose -> apply -> sync
```

1. **Explore**: think and investigate only. Do not write feature code outside of
   a change.
2. **Propose**: create a change with `proposal.md`, `design.md`, `tasks.md`, and
   delta specs.
3. **Apply**: implement tasks one at a time, checking each off in `tasks.md`
   only after verification.
4. **Sync**: merge verified delta specs into `openspec/specs/`, then delete the
   completed change directory. There is no `openspec/changes/archive/` folder —
   see `docs/adr/0003-sync-means-delete-not-archive.md`. Never run
   `openspec archive`.

## OpenSpec CLI

If your agent has no OpenSpec slash commands, use the CLI:

```bash
openspec list [--json] [--specs]
openspec new change "<name>"
openspec status --change "<name>" --json
openspec instructions <artifact> --change "<name>"
```

## Rules

- Before implementing anything, read the relevant files in `openspec/specs/` and
  the active change's artifacts.
- Do not write feature code without an active change proposal that contains
  tasks.
- Keep changes minimal and scoped to the task being implemented.
- Treat `openspec/specs/` as the truth. Reflect requirement changes there via
  the sync step, not by editing code silently.
- Keep project-specific contract, terms, and priorities in `PROJECT.md`.

## Language

- Write OpenSpec artifacts, ADRs, code comments, and commit messages in English.
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
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all --check
./scripts/changelog-guard.sh
```

Before the first real crate exists, the Rust commands are not yet
meaningful. The first project-specific OpenSpec change should add the real
crate layout and make the Definition of Done runnable from the workspace
root. `./scripts/changelog-guard.sh` is language-independent and always runs.

If a command cannot run in the current environment, report that explicitly.
