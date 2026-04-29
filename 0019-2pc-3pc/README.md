# 0019: Two-Phase and Three-Phase Commit

## Historical context

Two-phase commit (2PC) was the standard atomic-commitment
protocol of the early relational database era. Jim Gray's 1978
chapter "Notes on Database Operating Systems" formalised it and
identified its blocking property under coordinator failure.
Three-phase commit (3PC), credited to Skeen 1981, addresses the
blocking by adding a "pre-commit" round at the cost of more
messages and an additional synchrony assumption.

The protocols are atomic-commitment, not consensus: they
coordinate a single yes/no decision among a set of
participants under a coordinator. They are pedagogical
ancestors of Paxos (which generalises atomic commitment to
consensus on arbitrary values).

## System and threat model

- **Network.** Pair-wise reliable channels.
- **Failures.** Crash-stop. Both 2PC and 3PC assume no
  Byzantine faults.
- **Synchrony.** 2PC assumes asynchrony (it works correctly
  even with arbitrary delays, but blocks under coordinator
  failure). 3PC assumes synchrony (timeouts are the basis of
  its non-blocking property).
- **Coordinator.** A distinguished process that orchestrates
  the protocol.

## Theory

### Two-Phase Commit

```
phase 1 (voting):
  coordinator -> all: PREPARE(transaction)
  each participant -> coordinator: VOTE_YES or VOTE_NO

phase 2 (decision):
  if all votes YES:
    coordinator -> all: COMMIT
  else:
    coordinator -> all: ABORT
  each participant: act on COMMIT/ABORT
```

*Properties.*

- *Agreement.* All participants reach the same decision.
- *Validity.* The decision is COMMIT only if all participants
  voted YES.
- *Termination.* If the coordinator and all participants are
  alive, the protocol terminates.

*Failure mode (blocking).* If the coordinator crashes after
sending VOTE_YES to participants but before sending the
decision, participants in PREPARED state cannot infer the
decision: the coordinator might have committed (if other
participants voted YES) or might not have decided yet. They
must wait for the coordinator to recover or for an external
arbitrator.

### Three-Phase Commit

```
phase 1 (voting):
  coordinator -> all: CAN_COMMIT
  each participant -> coordinator: YES or NO

phase 2 (pre-commit):
  if all YES:
    coordinator -> all: PRE_COMMIT
    each participant -> coordinator: ACK
  else:
    coordinator -> all: ABORT, terminate

phase 3 (commit):
  coordinator -> all: COMMIT
  each participant: commit
```

The PRE_COMMIT round is the difference. It guarantees that, if
the coordinator crashes after PRE_COMMIT but before COMMIT, the
participants can elect a new coordinator and decide based on
the PRE_COMMIT state (they all know commit is the eventual
outcome).

*Non-blocking property.* Under synchrony (timeouts work), 3PC
is *non-blocking*: surviving participants can always make
progress without waiting for the coordinator to recover.

*Synchrony assumption.* 3PC's non-blocking depends on detecting
coordinator failure via timeout. Under asynchrony, the
detection is unreliable (a slow but alive coordinator looks
crashed), and 3PC can violate safety (participants commit while
the coordinator was about to abort).

### Comparison

| Property            | 2PC          | 3PC                  |
| ------------------- | ------------ | -------------------- |
| Synchrony required  | no           | yes                  |
| Blocking            | yes          | no (under sync)      |
| Round complexity    | 2            | 3                    |
| Message complexity  | `O(n)`       | `O(n)`               |
| Coordinator failure | blocks       | participants recover |
| Safe under async    | yes          | no                   |

### Connection to Paxos

Lamport's "Paxos Commit" 2004 generalises 2PC to a Paxos-based
atomic commit that survives coordinator failure without
blocking and without a synchrony assumption. The construction:
run a Paxos instance per participant's vote, plus one for the
overall decision. Cost: more messages (`O(n^2)` per commit
instance) but neither blocking nor synchrony-dependent.

Practical implementations (CockroachDB, Spanner) use Paxos
Commit for distributed transactions.

## Practice

### Where 2PC and 3PC show up

- *Classical RDBMS.* Oracle, MySQL XA, PostgreSQL prepared
  transactions all implement 2PC. Operators handle blocking via
  manual recovery.
- *Microservices saga pattern.* Sagas are an alternative to
  2PC: instead of distributed atomic commit, each step has a
  compensating action. No blocking, but no atomicity in the
  classical sense.
- *Distributed databases.* CockroachDB and Spanner use Paxos
  Commit, not 2PC. Cassandra uses lightweight transactions
  (Paxos-based).

### Why 2PC persists

Despite its blocking property, 2PC is widely deployed because:

- Operationally, blocking is rare and recovery is well-
  understood.
- The protocol is simple to implement and reason about.
- The synchrony assumption of 3PC is dubious in real networks.

3PC is rarely used in production for these reasons.

## Formalisation aspects

```text
inductive Decision where
  | commit
  | abort

structure TwoPcState where
  coordinator     : NodeId
  participants    : Finset NodeId
  votes           : Map NodeId (Option Bool)
  decision        : Option Decision
  prepared        : Set NodeId

theorem two_pc_agreement :
    forall (S : TwoPcState),
    forall (p1 p2 : NodeId), p1 ∈ S.participants -> p2 ∈ S.participants ->
    p1.decision = some d1 -> p2.decision = some d2 -> d1 = d2 := by
  -- agreement: all participants follow the coordinator's
  -- broadcast, which is a single value.
  sorry

theorem two_pc_blocking_under_async :
    exists (sched : AsyncSchedule),
      coordinator_crashes_after_prepare_phase /\
      participants_in_prepared_state_cannot_progress := by
  sorry
```

The blocking theorem captures the operational pain of 2PC; it
is a *non-existence* result analogous to FLP.

## Verifiability and circuit encoding

**Tag: `partial`.**

A 2PC commit log is SNARK-friendly: each step (PREPARE, VOTE,
COMMIT) is a signed message; verifying a complete commit log
requires verifying `O(n)` signatures. With BLS aggregation,
this collapses to one pairing per phase.

3PC adds one phase, so one extra pairing in circuit. The
synchrony assumption is *not* circuit-encodable; the verifier
treats it as an oracle input.

Production atomic-commit verifiability is rare because most
verifiable systems use Paxos-Commit-style consensus instead of
2PC/3PC. zk-rollup sequencers' atomic batches are similar in
spirit (one batch = one commit, verified by the L1 contract).

## Known attacks and limitations

- *2PC blocking.* The fundamental limitation. Operators handle
  via manual recovery.
- *3PC asynchrony.* Under asynchrony, 3PC violates safety.
  Production deployments avoid 3PC for this reason.
- *Coordinator centralisation.* Both protocols rely on a
  designated coordinator; failure modes concentrate there.
  Paxos-Commit eliminates this.

## Implementation notes

The crate provides a 2PC simulator with three node types
(coordinator, participant). Tests:

- Happy path: all participants vote YES, all commit.
- One participant votes NO: all abort.
- Coordinator crash after PREPARE: participants block.

The crashed-coordinator test demonstrates the blocking
property by showing that participants remain in PREPARED state
indefinitely after the coordinator stops sending decisions.

## References

- Gray, "Notes on Database Operating Systems", LNCS 60, 1978.
- Skeen, "Nonblocking Commit Protocols", SIGMOD 1981.
- Lamport, "Paxos Commit", TOCS 2006.

See also [`HISTORY.md`](../HISTORY.md), section "Pre-1980".
