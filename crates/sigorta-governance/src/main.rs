//! Executable architectural governance for the Sigorta workspace.

#![forbid(unsafe_code)]

use std::{env, process::ExitCode};

use tianheng::prelude::*;

const CONTRACT_REASON: &str = "sigorta-contract is the isolated core contract: a sans-I/O, \
single-instance circuit-breaking state machine. It needs no dependency at all, so it declares no \
normal dependency.";
const FACADE_REASON: &str = "sigorta is the curated public entrypoint: its normal dependencies \
are sigorta-contract alone, never a normal dependency the core itself does not have. That the \
facade holds no logic of its own is review-governed, not observed here.";
const GOVERNANCE_REASON: &str = "the governance gate must stay independent of the workspace \
graph it judges: its normal dependencies are tianheng alone, never sigorta-contract or any other \
workspace crate under judgment.";
const NO_AMBIENT_CLOCK_REASON: &str = "sigorta-contract's library makes no inline `std::time` \
`now` call, such as std::time::Instant::now or std::time::SystemTime::now, in any of its \
modules: every transition takes now: Instant as an explicit argument, because an ambient clock \
read is exactly the impurity this repository's real-world evidence had, and this project exists \
to remove it. Coverage is partial by nature (a clock read through a method on a value, such as \
`Instant::elapsed`, or a `now` path taken as a value rather than called, is invisible to a \
source scan), so this tooth complements review rather than replacing it.";

fn constitution() -> Constitution {
    Constitution::new("sigorta")
        .boundary(
            CrateBoundary::crate_("sigorta-contract")
                .restrict_dependencies_to(Vec::<&str>::new())
                .because(CONTRACT_REASON),
        )
        .boundary(
            CrateBoundary::crate_("sigorta")
                .restrict_dependencies_to(["sigorta-contract"])
                .because(FACADE_REASON),
        )
        .boundary(
            CrateBoundary::crate_("sigorta-governance")
                .restrict_dependencies_to(["tianheng"])
                .because(GOVERNANCE_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("sigorta-contract")
                .module("crate")
                .must_not_call_inline("std::time")
                .ending_with(["now"])
                .because(NO_AMBIENT_CLOCK_REASON),
        )
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    tianheng::run(&constitution(), args)
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    use super::*;

    const LAW_PROJECTION_PREAMBLE: &str = "\
# Sigorta Tianheng Law Projection

This file is generated from `constitution()` in `crates/sigorta-governance/src/main.rs`.
The Rust declaration is authoritative; do not edit the projection by hand.
Regenerate it with `BLESS=1 cargo test -p sigorta-governance law_projection_is_fresh`.

";

    fn workspace_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    #[test]
    fn current_workspace_satisfies_constitution() {
        GovernanceTest::for_constitution(constitution())
            .with_manifest_dir(workspace_root())
            .assert_clean();
    }

    #[test]
    fn every_workspace_crate_is_covered() {
        GovernanceTest::for_constitution(constitution())
            .with_manifest_dir(workspace_root())
            .assert_all_workspace_members_covered();
    }

    #[test]
    fn law_projection_is_fresh() {
        GovernanceTest::for_constitution(constitution())
            .with_manifest_dir(workspace_root())
            .assert_projection_fresh_with_preamble(
                "AGENTS.sigorta-law.md",
                LAW_PROJECTION_PREAMBLE,
            );
    }

    #[test]
    fn clean_scratch_workspace_stays_clean() {
        let workspace = TempWorkspace::new("sigorta-governance-clean");

        let outcome = workspace.outcome(&[]);
        assert!(
            matches!(outcome, Outcome::Clean(_)),
            "the clean scratch workspace must raise no violation: {outcome:?}"
        );
    }

    #[test]
    fn core_dependency_is_rejected() {
        let workspace = TempWorkspace::new("sigorta-governance-core-dependency");
        workspace.write_package("leak", "", "");
        workspace.write_package(
            "sigorta-contract",
            "[dependencies]\nleak = { path = \"../leak\" }\n",
            CLEAN_CORE,
        );

        let report = workspace.violations(&["leak"]);
        assert!(
            report.violations.iter().any(|violation| {
                violation.target() == "sigorta-contract"
                    && violation.rule == "restrict dependencies to"
                    && violation.finding == "leak"
            }),
            "expected the core dependency boundary to fire: {report:?}"
        );
    }

    #[test]
    fn facade_dependency_beyond_the_core_is_rejected() {
        let workspace = TempWorkspace::new("sigorta-governance-facade-dependency");
        workspace.write_package("leak", "", "");
        workspace.write_package(
            "sigorta",
            "[dependencies]\nsigorta-contract = { path = \"../sigorta-contract\" }\nleak = { path = \"../leak\" }\n",
            "",
        );

        let report = workspace.violations(&["leak"]);
        assert!(
            report.violations.iter().any(|violation| {
                violation.target() == "sigorta"
                    && violation.rule == "restrict dependencies to"
                    && violation.finding == "leak"
            }),
            "expected the facade dependency boundary to fire: {report:?}"
        );
    }

    #[test]
    fn governance_dependency_beyond_tianheng_is_rejected() {
        let workspace = TempWorkspace::new("sigorta-governance-gate-dependency");
        workspace.write_package(
            "sigorta-governance",
            "[dependencies]\ntianheng = { path = \"../tianheng\" }\nsigorta-contract = { path = \"../sigorta-contract\" }\n",
            "",
        );

        let report = workspace.violations(&[]);
        assert!(
            report.violations.iter().any(|violation| {
                violation.target() == "sigorta-governance"
                    && violation.rule == "restrict dependencies to"
                    && violation.finding == "sigorta-contract"
            }),
            "expected the governance dependency boundary to fire: {report:?}"
        );
    }

    #[test]
    fn core_ambient_clock_at_the_root_is_rejected() {
        let workspace = TempWorkspace::new("sigorta-governance-core-clock-root");
        workspace.write_package(
            "sigorta-contract",
            "",
            "pub fn leak() -> std::time::Instant {\n    std::time::Instant::now()\n}\n",
        );

        let report = workspace.violations(&[]);
        assert!(
            report.violations.iter().any(|violation| {
                violation.target() == "std::time"
                    && violation.rule == "inline symbol path confined to module"
                    && violation.finding == "std::time::Instant::now in crate"
            }),
            "expected the core ambient-clock boundary to fire: {report:?}"
        );
    }

    #[test]
    fn core_ambient_clock_in_a_submodule_is_rejected() {
        let workspace = TempWorkspace::new("sigorta-governance-core-clock-submodule");
        workspace.write_package(
            "sigorta-contract",
            "",
            "pub mod inner {\n    use std::time::SystemTime;\n\n    pub fn leak() -> SystemTime {\n        SystemTime::now()\n    }\n}\n",
        );

        let report = workspace.violations(&[]);
        assert!(
            report.violations.iter().any(|violation| {
                violation.target() == "std::time"
                    && violation.rule == "inline symbol path confined to module"
                    && violation.finding == "std::time::SystemTime::now in crate"
            }),
            "expected the core ambient-clock boundary to fire: {report:?}"
        );
    }

    /// A clean core source: it takes the clock reading as an argument, as the real core does.
    const CLEAN_CORE: &str =
        "pub fn decide(now: std::time::Instant) -> std::time::Instant {\n    now\n}\n";

    /// A scratch workspace carrying clean `sigorta-contract`, `sigorta`, and `sigorta-governance`
    /// crates (plus a stand-in `tianheng`), so every boundary has a real target and a violating
    /// fixture differs from the clean one only in what its test overwrites.
    struct TempWorkspace {
        path: PathBuf,
    }

    impl TempWorkspace {
        fn new(name: &str) -> Self {
            let path = env::temp_dir().join(format!("{name}-{}", std::process::id()));
            if path.exists() {
                fs::remove_dir_all(&path).expect("stale temporary workspace should be removable");
            }
            fs::create_dir_all(&path).expect("temporary workspace should be creatable");
            let workspace = Self { path };
            workspace.write_package("tianheng", "", "");
            workspace.write_package(
                "sigorta-governance",
                "[dependencies]\ntianheng = { path = \"../tianheng\" }\n",
                "",
            );
            workspace.write_package("sigorta-contract", "", CLEAN_CORE);
            workspace.write_package(
                "sigorta",
                "[dependencies]\nsigorta-contract = { path = \"../sigorta-contract\" }\n",
                "",
            );
            workspace
        }

        fn outcome(&self, extra_members: &[&str]) -> Outcome {
            let entries = [
                "tianheng",
                "sigorta-governance",
                "sigorta-contract",
                "sigorta",
            ]
            .iter()
            .chain(extra_members)
            .map(|member| format!("    \"{member}\","))
            .collect::<Vec<_>>()
            .join("\n");
            fs::write(
                self.path.join("Cargo.toml"),
                format!("[workspace]\nresolver = \"2\"\nmembers = [\n{entries}\n]\n"),
            )
            .expect("workspace manifest should be writable");

            tianheng::check_constitution(&constitution(), &self.path.join("Cargo.toml"))
        }

        fn violations(&self, extra_members: &[&str]) -> Report {
            match self.outcome(extra_members) {
                Outcome::Violations(report) => report,
                other => panic!("expected violations, got {other:?}"),
            }
        }

        fn write_package(&self, name: &str, dependencies: &str, source: &str) {
            let package = self.path.join(name);
            fs::create_dir_all(package.join("src")).expect("package source dir should be writable");
            fs::write(
                package.join("Cargo.toml"),
                format!(
                    "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n{dependencies}"
                ),
            )
            .expect("package manifest should be writable");
            fs::write(package.join("src/lib.rs"), source)
                .expect("package source should be writable");
        }
    }

    impl Drop for TempWorkspace {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}
