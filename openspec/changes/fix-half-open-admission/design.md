## Context

The real-world evidence's `HalfOpen` branch of `admit` is:

```rust
BreakerState::HalfOpen { probe_expires } => {
    if now < *probe_expires {
        Some(*probe_expires - now)  // reject: the probe is still outstanding
    } else {
        *state = BreakerState::HalfOpen { probe_expires: cooldown_end(now) };
        None  // admit: issue a fresh probe
    }
}
```

A caller during an outstanding, unexpired probe is **rejected** — the same shape as
being rejected by `Open`, just bounded by the probe's own deadline instead of the
cooldown's. Only once that deadline passes with no recorded outcome does the next
caller get admitted, as a fresh probe.

This project's `0.1.1` release correctly added the *bound* (`probe_expires` and its
renewal), but inverted which side of that comparison admits and which rejects:
`0.1.1`'s `admit` returned `Probing` (admitting) for every check within the window,
and only changed behavior — still admitting, just with a renewed bound — once the
window passed. It never actually rejected anyone. This was wrong from
`initial-project-shape` onward, not a regression `0.1.1` introduced: `0.1.1` bounded
the wrong branch's behavior instead of fixing the missing rejection.

Real integration testing against the evidence (adopting the published crate into
the application it was distilled from) is what caught this — the evidence's own
existing test suite failed immediately once the two were wired together.

## Goals / Non-Goals

**Goals:**
- Match the evidence exactly: reject a caller during an outstanding, unexpired
  probe; admit a fresh probe once the window has passed.

**Non-Goals:**
- Any other behavioral change. `Closed` and `Open`'s branches, `record`, `Event`,
  and `Decision`'s shape are all already correct and untouched.

## Decisions

### D1 — Reject within the window, admit past it

```rust
State::HalfOpen { probe_expires } => {
    if now < probe_expires {
        Decision::Rejected {
            retry_after: probe_expires - now,
            core: self,
        }
    } else {
        let renewed = Self {
            state: State::HalfOpen { probe_expires: now + self.open_duration },
            ..self
        };
        Decision::Probing(renewed)
    }
}
```

This mirrors `Open`'s own branch shape almost exactly (reject-with-remaining-time
vs. admit-with-a-renewed-bound), which is itself a signal this is the right
condition — the two branches now read as structurally parallel, where `0.1.1`'s
version did not.

## Risks / Trade-offs

- **A second behavior-changing release in two version bumps.** → Both `0.1.1` and
  this fix are corrections toward the same evidence, not new design decisions;
  documented plainly in `CHANGELOG.md` as what changed and why, rather than framed
  as a feature.
- **Any consumer that adopted `0.1.0` or `0.1.1` and depended on every half-open
  check being admitted will see a behavior change.** → No real consumer has adopted
  either release yet (`BACKLOG.md`'s Settled Decisions already records this project
  publishes ahead of a real consumer); this is corrected before the first real
  adoption completes, not after.
