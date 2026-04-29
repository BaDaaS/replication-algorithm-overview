# 0023: Paxos Made Simple and Multi-Paxos

## Historical context

Lamport's 2001 SIGACT News paper "Paxos Made Simple"
re-presented the 1998 Synod protocol in a more accessible
form, then sketched the *Multi-Paxos* extension that runs an
unbounded sequence of Synod instances to give full SMR.

Multi-Paxos is the form of Paxos actually deployed in
production. The key engineering insight: Phase 1 (prepare/
promise) is a *leadership claim*; once a leader is established,
each subsequent decision only needs Phase 2. Phase 1 amortises
across many decisions.

## System and threat model

Same as Synod (module 0022): asynchronous, crash-recovery
with stable storage, `f < n / 2`, authenticated channels.

## Theory

### Sequence of instances

Multi-Paxos runs one Synod instance per *log slot* `i = 0, 1,
2, ...`. Each slot decides on one operation. The replicated
log is the concatenation of decisions.

### Leader-based amortisation

```
on becoming leader (executing Phase 1 once):
  proposer broadcasts Prepare(b) for all future slots
  collects f + 1 promises
  for each slot already accepted by some acceptor, propose
    that value (continues the log)

per-decision (Phase 2 only):
  client -> leader: REQUEST(op)
  leader: assigns next slot i; proposes (b, i, op)
  leader -> all: Accept(b, i, op)
  acceptors: respond Accepted(b, i, op)
  leader: commits on f + 1 Accepteds
```

Phase 1 happens once per leader. Phase 2 happens per decision.

### Theorem (Multi-Paxos correctness)

Multi-Paxos satisfies the SMR safety theorem (module 0000)
under crash-recovery with stable storage and `f < n / 2`. The
proof is a per-slot application of the Synod safety theorem.

### Leader election

Multi-Paxos requires a *distinguished proposer* for liveness.
The election can be:

- *Static.* One pre-chosen process is the leader; others wait.
- *Failure-detector-based.* The leader is the eventually-stable
  output of an Omega detector.
- *Randomised.* Each process backs off randomly before
  proposing.

In production, leader election is timeout-driven (Raft's
explicit timeout, etcd's heartbeat-loss detection).

### Pipelining

A leader can issue Phase 2 for slots `i, i+1, i+2, ...`
without waiting for `i` to commit. If commits return out of
order, the leader applies them in order to the state machine.

Pipelining gives Multi-Paxos's typical throughput: bottle-
necked on the leader's CPU and network, not on round-trip
latency.

## Practice

### Real Multi-Paxos deployments

- *Google Chubby* (Burrows 2006). Lock service.
- *Spanner* (Corbett 2012). Multi-Paxos per replica group; one
  group per shard.
- *PaxosStore* (WeChat). Distributed key-value store using
  Multi-Paxos.
- *MongoDB's replica sets.* Adopt a Paxos-inspired primary-
  backup with explicit elections.

### Multi-Paxos vs Raft

Raft (module 0028) is structurally Multi-Paxos with explicit
leader election and log-replication state machines. Production
defaults have shifted toward Raft for pedagogy, but the
underlying correctness is the same.

### Engineering notes

- *Snapshotting.* The log grows unboundedly; production
  truncates by snapshotting state every `k` slots.
- *Log compaction.* Snapshots replace early log slots; new
  followers catch up via the snapshot.
- *Membership change.* Reconfiguration is a special operation
  in the SMR (Lamport's "Reconfigurable Paxos" 2009).

## Formalisation aspects

```text
structure MultiPaxosState where
  view             : Nat
  leader           : NodeId
  log              : Vec (Option (Ballot × Operation))
  commit_index     : Nat

theorem multi_paxos_smr_safety
    (n f : Nat) (h : 2 * f + 1 = n) :
    forall (sched : AsyncSchedule),
    forall (slot : Nat) (op_a op_b : Operation),
      Decided slot op_a sched -> Decided slot op_b sched ->
      op_a = op_b := by
  -- per-slot Synod safety + log monotonicity
  sorry
```

The per-slot reduction makes Multi-Paxos a "many parallel
Synods plus a leader". The safety proof composes per-slot
Synod proofs. cslib's `LTS` framework can model the leader's
state directly.

## Verifiability and circuit encoding

**Tag: `friendly`.**

Multi-Paxos's verifiability profile (per commit, with leader-
amortised Phase 1):

- Phase 2 quorum cert: BLS aggregate, ~10^6 constraints.
- Slot consistency: ~constraints.

Total: ~10^6 constraints per decision (one BLS pairing).
Compare to Synod's 2 * 10^6: amortised Phase 1 saves a
pairing per slot.

This is why production verifiable BFT (HotStuff family,
Aptos) uses Multi-Paxos-like amortisation: per-slot proof
cost is dominated by one quorum cert.

## Known attacks and limitations

- *Leader bottleneck.* Throughput limited by leader's
  resources. Mitigations: parallel proposers (Mencius), out-of-
  order commits (EPaxos).
- *Leader election cost.* Each leader change triggers a
  Phase 1; under churn, this is expensive. Stable leadership
  amortises better.
- *Stable storage cost.* Each acceptor must fsync `(promised,
  accepted)` per Phase 2. Production batches.

## Implementation notes

The crate provides a Multi-Paxos with:

- A static leader (NodeId(0)).
- Pre-loaded ops at the leader.
- Phase 1 executed once at start; Phase 2 per op.

Test verifies all replicas reach the same log of operations.

## References

- Lamport, "Paxos Made Simple", SIGACT News 2001.
- Lamport, "Reconfigurable Paxos", 2009.
- Burrows, "The Chubby Lock Service for Loosely-Coupled
  Distributed Systems", OSDI 2006.
- Corbett et al., "Spanner: Google's Globally-Distributed
  Database", OSDI 2012.

See also [`HISTORY.md`](../HISTORY.md), section "1986 to 1999"
and "2000 to 2008".
