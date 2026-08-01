//! Executable architectural governance for the Sigorta workspace.

#![forbid(unsafe_code)]

use std::{env, process::ExitCode};

use tianheng::prelude::*;

const CONTRACT_REASON: &str = "sigorta-contract is the isolated core contract: a sans-I/O, \
single-instance circuit-breaking state machine. It needs no dependency at all, so it may \
depend on nothing.";
const FACADE_REASON: &str = "sigorta is the curated public entrypoint: a pure re-export facade \
with no logic of its own. It must depend on sigorta-contract only, never acquiring a dependency \
the core itself does not have.";
const GOVERNANCE_REASON: &str = "the governance gate must stay independent of the workspace \
graph it judges: it may depend only on tianheng, never on sigorta-contract or any other \
workspace crate under judgment.";
const NO_AMBIENT_CLOCK_REASON: &str = "the sans-I/O core reads no wall clock: every transition \
takes now: Instant as an explicit argument. A call to std::time::Instant::now or \
std::time::SystemTime::now anywhere in sigorta-contract is exactly the impurity this \
repository's real-world evidence had, and this project exists to remove it.";

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
