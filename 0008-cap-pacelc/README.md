# 0008: CAP and PACELC

## Historical context

Brewer's PODC 2000 keynote [`brewer2000`] formulated the
distributed-systems folklore that "you cannot have all of
Consistency, Availability, and Partition tolerance at the same
time". Gilbert and Lynch (SIGACT News 2002) [`gl2002cap`]
formalised it as a theorem in the asynchronous model with linear
consistency.

In 2012, Abadi proposed PACELC [`abadi2012pacelc`] as a refinement
acknowledging that even *without* partitions, distributed systems
trade *latency* (L) against *consistency* (C). The mnemonic: in
case of *Partition*, choose *Availability* or *Consistency*;
else (*Else*), choose *Latency* or *Consistency*.

CAP-PACELC is folklore-precise rather than mathematically deep.
The Gilbert-Lynch theorem has a one-page proof; the value is in
the conceptual scaffolding it gives engineers when picking a
data store.

## System and threat model

- **Network.** Asynchronous; partitions can occur (the network
  may split into two components for an arbitrary period).
- **Failures.** Channels may be partitioned; processes may be
  in either partition.
- **Operations.** Read and write to a shared register.
- **Goal.** A protocol that guarantees the data store is
  *Consistent* (linearisable: every read returns the latest
  acknowledged write), *Available* (every operation eventually
  responds), and *Partition-tolerant* (correct under partitions).

## Theory

### Definition (Linearisability)

An execution is *linearisable* if there exists a serialisation
of the operations such that:

- The serial order respects the real-time order of operations
  (if `op_1` finishes before `op_2` starts, `op_1` precedes
  `op_2` in the serial order).
- Each read returns the latest write preceding it in the serial
  order.

### Definition (Availability)

A system is *available* if every operation issued at any non-
crashed process eventually completes.

### Definition (Partition tolerance)

A system is *partition-tolerant* if it remains correct (linearisable
and available, with the modifications below) under arbitrary
network partitions.

### Theorem (Gilbert-Lynch 2002)

It is impossible for a distributed system to simultaneously
provide all three of Consistency (linearisability), Availability,
and Partition tolerance.

*Proof.* Consider two processes `p_1, p_2` connected by a
channel that may partition. Initial register value is `0`.

- *Phase A:* `p_1` writes `1` and the partition is in place.
  `p_1`'s write must complete (availability). `p_2` cannot have
  observed the write (partitioned).
- *Phase B:* `p_2` reads. Availability requires `p_2` to respond.
  Linearisability requires `p_2` to return `1` (the latest
  write) or otherwise some value not yet written.

If `p_2` returns `0`, the schedule
`write(1) -> read() = 0` is non-linearisable. If `p_2` blocks
until the partition heals, availability is violated.

Hence one of CAP fails. QED.

### Caveats and refinements

- *Linearisability is strict.* CAP rules out *strong*
  consistency under partitions. Weaker consistency models
  (causal, session, eventual) are achievable with availability
  and partition-tolerance. CRDT-based systems exemplify this.
- *Partial partitions.* In partial partitions (one-way drops,
  asymmetric routes), the trade-off is more subtle.
- *Cloud reality.* Most production "CP" systems (Spanner, etcd)
  drop availability during partitions; most "AP" systems
  (Cassandra, DynamoDB) drop consistency.

### PACELC

For a system to be useful, the trade-off must be characterised
both in partition mode (P/A vs P/C) and in normal mode (E/L vs
E/C):

- **PA/EL.** Available and low-latency at all times. No strong
  consistency. Examples: Cassandra default, Riak, DynamoDB
  eventually-consistent reads.
- **PA/EC.** Available under partitions, consistent in
  normal operation. Examples: MongoDB with primary node and
  multi-region failovers.
- **PC/EL.** Consistent under partitions, low-latency in normal
  operation. Examples: Spanner with TrueTime (but at the cost
  of clock-skew handling).
- **PC/EC.** Consistent always. Examples: classical RDBMS with
  synchronous replication; etcd, ZooKeeper.

Abadi's classification is operationally useful. It refines CAP
by acknowledging that even outside partitions, distributed
systems pay a latency tax for consistency.

### Connection to FLP

CAP rules out CAP under partitions. FLP rules out deterministic
consensus under asynchrony. Both concern the *tension between
liveness (availability / termination) and safety (consistency /
agreement)* in adverse network conditions. CAP is the
linearisability-restricted, partition-specific instance of the
FLP family.

## Practice

### Where systems sit

| System         | CAP        | PACELC    |
| -------------- | ---------- | --------- |
| etcd, ZooKeeper| CP         | PC/EC     |
| Spanner        | CP         | PC/EC*    |
| CockroachDB    | CP         | PC/EC     |
| DynamoDB       | AP         | PA/EL     |
| Cassandra      | AP         | PA/EL     |
| MongoDB        | configurable | varies  |
| Aurora MySQL   | CP-ish     | PC/EL*    |
| CometBFT       | CP         | PC/EC     |
| Aptos / Sui    | CP         | PC/EC     |
| Bitcoin        | AP         | PA/EL     |
| Ethereum       | partially  | PC/EC at finality, PA/EL pre-finality |

(*) Spanner and Aurora are sometimes described as bypassing CAP
because their network is engineered for very low partition
probability. They formally remain CP; the engineering reduces
the cost of partition mode.

### Engineering implications

- A new system designed for "global availability with no
  consistency" is implicitly choosing AP. Eventual consistency
  protocols (CRDTs, gossip) are the right toolkit.
- A new system requiring strong consistency under partitions
  (e.g. for financial transactions) must accept downtime under
  partition; failover automation handles it operationally.
- Blockchains live in an interesting CAP middle ground:
  permissionless chains are AP at the longest-chain level (any
  partition resolves by accepting the longer chain); finality
  gadgets (Casper FFG, GRANDPA) provide CP at a higher level
  with explicit liveness assumptions (`f < n / 3` honest
  validators in the larger partition).

## Formalisation aspects

```text
def IsLinearisable
    (run : Sequence Operation) (history : Sequence Event) : Prop :=
  exists (serial : Sequence Operation),
    ConsistentWithRealtimeOrder serial history /\
    EveryReadReflectsLatestWrite serial

def IsAvailable
    (Sys : DistributedSystem) (sched : Schedule) : Prop :=
  forall (op : Operation) (issued_at : Time),
    exists (resp_at : Time), resp_at < infinity /\
      Responds Sys op issued_at resp_at sched

theorem cap_impossibility :
    forall (Sys : DistributedSystem),
      (Linearisable Sys /\ Available Sys /\ PartitionTolerant Sys)
      = false := by
  -- Two processes, partition between them, one write then one
  -- read. Determinism on the read side gives the contradiction.
  sorry
```

The `Linearisable` predicate is the most subtle to formalise; it
is a *history* property quantifying over a permutation of
operations. Mathlib's `List.Permutations` and the
`Order.Embedding` infrastructure suffice. Lamport's TLA+ model of
linearisability (Herlihy-Wing 1990, formalised in TLA+ 2003) is
a good Lean reference point.

## Verifiability and circuit encoding

**Tag: `na`** for CAP and PACELC themselves; the relevance is
indirect.

A verifiable replication algorithm typically lives in the
`PC/EC` corner: it is a strongly consistent protocol with a
SNARK proof of state transitions. Under partition, it loses
availability (the partition's smaller side cannot produce a new
proof; the larger side can). PACELC's `EC` corner means the
system also pays a latency tax in normal operation, which the
SNARK proving cost compounds: each block adds `~10^8` to `~10^9`
constraints to the prover budget.

The trade-off is therefore *amplified*, not avoided, by adding
verifiability. This is one of the open engineering tensions in
verifiable replication: SNARK-based finality gadgets (Mina,
zk-rollups) inherit the latency hit of the consistency-favouring
choice and add the proving latency on top.

## Known attacks and limitations

- *Partition-tolerance is a property of the network, not the
  protocol.* CAP is sometimes misread as "you can choose two
  of three"; the right reading is "if partitions can occur,
  you must give up one of A and C".
- *PACELC's E case.* Many production systems implement multiple
  consistency levels (strong vs eventual); the PACELC label
  applies per-operation rather than per-system.
- *Empirical CAP.* Real systems sit on a spectrum. The labels
  are useful but should not be taken as binary.

## Implementation notes

The crate provides a tiny two-process register:
`OnePartitionAdversary { partition_at, heal_at }` partitions the
network from `partition_at` to `heal_at` (drops all messages
both ways). Tests demonstrate:

- *AP variant.* The register responds immediately to reads even
  under partition, returning a stale value.
- *CP variant.* The register blocks reads under partition,
  returning only after the partition heals.

The two variants are implemented as different processes
sharing the same wire protocol, so the test compares them on
identical schedules.

## References

- Brewer, "Towards Robust Distributed Systems", PODC 2000.
  [`brewer2000`].
- Gilbert and Lynch, "Brewer's Conjecture and the Feasibility
  of Consistent, Available, Partition-Tolerant Web Services",
  SIGACT News 2002. [`gl2002cap`].
- Abadi, "Consistency Tradeoffs in Modern Distributed Database
  System Design", IEEE Computer 2012. [`abadi2012pacelc`].
- Herlihy and Wing, "Linearizability: A Correctness Condition
  for Concurrent Objects", TOPLAS 1990.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
