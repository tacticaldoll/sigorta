## MODIFIED Requirements

### Requirement: Half-open state admits exactly one trial probe, bounded
While in the half-open state, the core SHALL treat a recorded `Success` as closing the
breaker and resetting its failure count, and SHALL treat a recorded `Failure` as
reopening the breaker for a fresh cooldown. A single probe SHALL NOT remain
outstanding indefinitely: once it has been outstanding longer than the configured
open duration, the next admission check SHALL replace it with a fresh probe rather
than extending the same one forever.

#### Scenario: A successful probe closes the breaker
- **WHEN** a `Sigorta` in the half-open state records a `Success` event
- **THEN** the resulting `Sigorta` SHALL transition to closed with a failure count of
  zero

#### Scenario: A failed probe reopens the breaker
- **WHEN** a `Sigorta` in the half-open state records a `Failure` event
- **THEN** the resulting `Sigorta` SHALL transition to open, recording a new
  eligible-again moment as `now` plus the configured open duration

#### Scenario: Admission during an outstanding probe still within its window
- **WHEN** a `Sigorta` in the half-open state processes an admission check with `now`
  before the outstanding probe's bound
- **THEN** the decision SHALL be `Probing` again, carrying the state unchanged, since
  exactly one trial probe remains outstanding until its outcome is recorded or its
  bound passes

#### Scenario: A stale outstanding probe is replaced with a fresh one
- **WHEN** a `Sigorta` in the half-open state processes an admission check with `now`
  at or after the outstanding probe's bound, with no outcome having been recorded for
  it
- **THEN** the decision SHALL be `Probing` carrying a state whose bound is renewed as
  `now` plus the configured open duration, admitting the caller as a fresh probe
