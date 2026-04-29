# 0037: Spanner-style Replication with TrueTime

## Historical context

Corbett et al. published "Spanner: Google's Globally
Distributed Database" at OSDI 2012. Spanner is a planet-scale
database supporting external consistency via *TrueTime*, an
API that exposes the *uncertainty* in the local clock as a
bounded interval `[earliest, latest]`. Combined with Multi-
Paxos per replica group, TrueTime gives Spanner globally
linearisable transactions without per-write cross-region
coordination.

The TrueTime idea: instead of pretending clocks are
synchronised, expose the uncertainty. A transaction commits
at a timestamp `t_commit`; the protocol *waits* until
`TrueTime.now().earliest > t_commit` before responding to the
client, ensuring `t_commit` is in the past for every observer.

## System and threat model

- **Network.** Asynchronous; partially synchronous in normal
  operation.
- **Failures.** Crash-recovery; per-replica-group `f < n / 2`.
- **Clocks.** Each server has a clock with bounded
  uncertainty `epsilon` (typically <7ms in Google's
  deployment). TrueTime exposes `[t - epsilon, t + epsilon]`.
- **Goal.** Globally externally-consistent transactions.

## Theory

### TrueTime API

```
TT.now() : (earliest, latest)  -- both u64 timestamps
TT.after(t) : Bool             -- t < TT.now().earliest
TT.before(t) : Bool            -- t > TT.now().latest
```

The "uncertainty" `epsilon = (latest - earliest) / 2` is
typically <10ms.

### Commit-wait protocol

```
on transaction commit at coordinator:
  pick t_commit > TT.now().latest  // future timestamp
  run Paxos to commit the transaction at t_commit
  wait until TT.after(t_commit)    // commit-wait
  respond to client
```

The commit-wait ensures that, when the client sees the ack,
*every observer's clock has passed t_commit*. This gives
external consistency: any later transaction will have a
larger timestamp.

### External consistency

External (or strict) serialisability: the order of committed
transactions matches the real-time order of their commits.

Spanner's TrueTime + commit-wait achieves this.

### Theorem (Spanner external consistency)

Under TrueTime's bounded-uncertainty hypothesis and Multi-Paxos
correctness, Spanner's transaction protocol is externally
consistent.

*Proof.* Corbett et al. 2012, Section 4.2.2. Commit-wait
ensures `t_commit < real-time clock value` at all replicas
when the client sees the ack. Subsequent transactions pick a
larger `t_commit`.

### Cost

- *Commit-wait latency.* `~epsilon = ~7ms` typical. Tunable
  by clock-quality investment (atomic clocks, GPS).
- *Read-only transactions.* Don't need commit-wait; can use
  TrueTime to pick a snapshot timestamp.

## Practice

### Production

- *Spanner / Cloud Spanner.* Production inside Google;
  available as a managed service externally.
- *CockroachDB.* Inspired by Spanner but uses NTP-bounded
  clocks rather than atomic-clock infrastructure. Achieves
  serialisability via uncertainty intervals and retries.
- *YugabyteDB.* Similar approach.
- *FoundationDB.* Different design (centralised sequencer)
  but addresses similar problems.

### TrueTime infrastructure

Google deploys atomic clocks and GPS receivers in each data
centre. Time is propagated via local time servers. The
uncertainty `epsilon` is monitored continuously and exposed
to applications.

### Why uncertainty matters

Real clocks drift. Without explicit uncertainty:

- *Underestimating drift.* Risk wrong-order commits.
- *Overestimating drift.* Pay unnecessary commit-wait.

TrueTime's value: tight, monitored, bounded uncertainty.

## Formalisation aspects

```text
structure TrueTime where
  now      : Time × Time  -- (earliest, latest)
  after_lt :
    forall t : Time, after t = true -> t < now.earliest

theorem spanner_external_consistency
    (TT : TrueTime)
    (paxos : MultiPaxos) :
    forall (T_1 T_2 : Transaction),
    CommittedAt TT T_1 t_1 ->
    StartedAfterCommit TT T_2 T_1 ->
    CommittedAt TT T_2 t_2 ->
    t_1 < t_2 := by
  -- commit-wait + TrueTime soundness
  sorry
```

## Verifiability and circuit encoding

**Tag: `partial`.**

Spanner's verifiability is challenged by TrueTime: clock-
based external consistency depends on TT's guarantees, which
are operational rather than cryptographic.

Verifiable counterpart: replace TrueTime with a *verifiable
delay function* (VDF) for timestamping, or with a signed-
heartbeat-based bounded-uncertainty oracle. Each commit's
timestamp is then provable in circuit.

Per commit: ~10^6 (BLS) + ~10^4 (timestamp witness) =
~10^6 constraints. The VDF approach adds ~200k constraints
per timestamp.

## Known attacks and limitations

- *Clock-failure dependence.* TrueTime's `epsilon` bound is
  operational. If clock infrastructure fails, the bound is
  violated, breaking external consistency.
- *Commit-wait latency.* Adds `~epsilon` to every committed
  transaction. Trade-off vs clock-quality cost.
- *Geographic deployment.* Cross-region replicas pay the
  speed-of-light latency.

## Implementation notes

The crate provides a TrueTime simulation:

- `TrueTime::now() -> (earliest, latest)` based on a
  configurable `epsilon`.
- A simulated transaction with commit-wait.

Tests verify external consistency: transaction `T_2` started
after `T_1` commits has `t_2 > t_1`.

## References

- Corbett et al., "Spanner: Google's Globally Distributed
  Database", OSDI 2012.
- Brewer, "Spanner, TrueTime and the CAP Theorem", Google
  white paper, 2017.

See also [`HISTORY.md`](../HISTORY.md), section "2009 to 2014".
