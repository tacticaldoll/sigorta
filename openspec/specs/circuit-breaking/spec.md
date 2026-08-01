## Purpose

A sans-I/O, single-instance circuit-breaking core that adjudicates whether to admit an
attempt, admit it as a trial probe, or reject it with a retry-after duration, given an
explicit clock reading and a caller-supplied stream of success/failure events.

## Requirements

### Requirement: Explicit clock input
The core SHALL require an explicit `now: Instant` argument for every state transition
and SHALL NOT read the system clock internally.

#### Scenario: Admission check uses caller-supplied time
- **WHEN** a caller checks admission by supplying `now`
- **THEN** the decision SHALL be computed only from `now` and the current state, never
  from an internally read clock value

### Requirement: Single-instance state ownership
A `Sigorta` value SHALL represent exactly one breaker's state. The core SHALL NOT
provide any keyed collection of multiple breaker states.

#### Scenario: Multiple keys are the caller's concern
- **WHEN** a caller needs to track breakers for multiple distinct keys
- **THEN** the caller SHALL maintain that keyed collection outside the core, holding one
  `Sigorta` value per key

### Requirement: Closed state admits and accumulates failures
While in the closed state, the core SHALL admit every admission check and SHALL track
consecutive failures toward a configured threshold when outcomes are recorded.

#### Scenario: Closed state admits normally
- **WHEN** a `Sigorta` in the closed state processes an admission check
- **THEN** the decision SHALL be `Admitted` carrying the unchanged state

#### Scenario: A recorded success resets the failure count
- **WHEN** a `Sigorta` in the closed state records a `Success` event
- **THEN** the resulting `Sigorta` SHALL remain closed with a failure count of zero

#### Scenario: Reaching the failure threshold trips the breaker
- **WHEN** a `Sigorta` in the closed state records a `Failure` event that brings the
  consecutive failure count to the configured threshold
- **THEN** the resulting `Sigorta` SHALL transition to open, recording the moment it
  becomes eligible to test again as `now` plus the configured open duration

### Requirement: Open state rejects until cooldown elapses
While in the open state, the core SHALL reject every admission check with a computed
retry-after duration until the configured cooldown elapses, then SHALL transition to a
trial probe on the next admission check.

#### Scenario: Open state rejects before cooldown elapses
- **WHEN** a `Sigorta` in the open state processes an admission check with `now` before
  the recorded eligible-again moment
- **THEN** the decision SHALL be `Rejected` with `retry_after` equal to the remaining
  duration until that moment

#### Scenario: Cooldown elapsed transitions to a trial probe
- **WHEN** a `Sigorta` in the open state processes an admission check with `now` at or
  after the recorded eligible-again moment
- **THEN** the decision SHALL be `Probing` carrying a state transitioned to half-open

### Requirement: Half-open state admits exactly one trial probe
While in the half-open state, the core SHALL treat a recorded `Success` as closing the
breaker and resetting its failure count, and SHALL treat a recorded `Failure` as
reopening the breaker for a fresh cooldown.

#### Scenario: A successful probe closes the breaker
- **WHEN** a `Sigorta` in the half-open state records a `Success` event
- **THEN** the resulting `Sigorta` SHALL transition to closed with a failure count of
  zero

#### Scenario: A failed probe reopens the breaker
- **WHEN** a `Sigorta` in the half-open state records a `Failure` event
- **THEN** the resulting `Sigorta` SHALL transition to open, recording a new
  eligible-again moment as `now` plus the configured open duration

#### Scenario: Admission during an outstanding probe
- **WHEN** a `Sigorta` in the half-open state processes an admission check before its
  outstanding probe's outcome is recorded
- **THEN** the decision SHALL be `Probing` again, carrying the state unchanged, since
  exactly one trial probe remains outstanding until its outcome is recorded

### Requirement: Admission checks consume ownership and the decision carries the new state
The admission-check method SHALL consume the current `Sigorta` value by value and
SHALL return a `Decision` whose variant carries the resulting `Sigorta` forward, rather
than returning it as a separate value.

#### Scenario: Decision variants carry state
- **WHEN** the admission-check method is called
- **THEN** the returned `Decision` SHALL be one of `Admitted(Sigorta)`,
  `Probing(Sigorta)`, or `Rejected { core: Sigorta, retry_after: Duration }`, with no
  separate return value carrying state

### Requirement: Recording consumes ownership and returns the new state directly
The recording method SHALL consume the current `Sigorta` value by value and SHALL
return the resulting `Sigorta` directly, without an admission-decision wrapper —
recording an outcome answers no admission question.

#### Scenario: Recording returns the breaker, not a decision
- **WHEN** the recording method is called
- **THEN** it SHALL return a `Sigorta` value, not a `Decision`

#### Scenario: Recording while open is a caller precondition violation
- **WHEN** the recording method is called on a `Sigorta` in the open state
- **THEN** it SHALL return the breaker unchanged, since no attempt could have been
  admitted to produce an outcome in that state

### Requirement: Closed event vocabulary
The core SHALL accept outcomes only as a closed, two-variant `Event` enum (`Success`,
`Failure`) supplied by the caller, and SHALL NOT expose a trait for classifying an
outcome.

#### Scenario: Caller supplies an already-judged event
- **WHEN** a caller calls the recording method
- **THEN** it SHALL accept only an `Event` value and SHALL NOT accept or inspect any
  domain-specific error type
