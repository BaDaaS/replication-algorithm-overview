# 0034: ZAB -- ZooKeeper Atomic Broadcast

## Historical context

Junqueira, Reed, and Serafini published "Zab: High-performance
broadcast for primary-backup systems" at DSN 2011, formalising
the protocol that underpins Apache ZooKeeper (released 2008).
ZAB is structurally similar to Multi-Paxos with a strong
emphasis on *primary order*: each leader's writes preserve
the order in which the primary received them, even across
view changes.

ZooKeeper became the canonical coordination service for
distributed systems in the 2010s. Hadoop, Kafka, HBase,
Solr, and many others depend on it. ZAB's design emphasis on
order-preserving recovery comes directly from coordination-
service requirements (e.g. lock-acquisition order matters).

## System and threat model

- **Network.** Asynchronous, FIFO point-to-point.
- **Failures.** Crash-recovery with stable storage; `f < n / 2`.
- **Goal.** Linearisable atomic broadcast, with primary
  ordering preserved across views.

## Theory

### Phases

ZAB has four phases per view:

1. *Discovery.* Newly elected leader collects each follower's
   last accepted epoch and zxid (transaction id).
2. *Synchronisation.* Leader picks the longest-prefix log
   among the followers; each follower truncates or extends
   its log to match.
3. *Broadcast.* Leader broadcasts new transactions via
   PROPOSE; followers ACK; leader COMMITs after `f + 1` ACKs.
4. *Recovery.* On leader failure, restart from Discovery.

### Key data: zxid

A zxid is `(epoch, counter)`. Epoch is the leader's term;
counter increments per transaction within the term. Zxids
provide:

- Total order on transactions.
- Easy log comparison (compare zxids lexicographically).

### Theorem (ZAB correctness)

ZAB satisfies linearisable atomic broadcast under
crash-recovery and `f < n / 2`. Junqueira-Reed-Serafini 2011
gives the proof.

The *primary-order property*: if the leader broadcasts `op_1`
before `op_2`, all honest replicas deliver them in this order.
Even across view changes, this property holds because the new
leader's synchronisation phase preserves the previous
leader's order.

### Comparison to Paxos and Raft

| Aspect             | Multi-Paxos     | Raft               | ZAB                      |
| ------------------ | --------------- | ------------------ | ------------------------ |
| Term/view          | ballot          | term               | epoch                    |
| Election           | implicit        | explicit RV/Vote   | leader-election protocol |
| Per-tx id          | (ballot, slot)  | (term, index)      | (epoch, counter) = zxid  |
| Synchronisation    | Phase 1         | log catch-up       | explicit Sync phase      |
| Order preservation | yes             | yes                | yes (FIFO emphasised)    |

## Practice

### ZooKeeper deployment

ZooKeeper is the reference ZAB implementation. Production
deployments:

- *3 or 5 servers.* Tolerates 1 or 2 failures respectively.
- *Read amplification.* Reads served by any replica without
  consensus; writes through ZAB.
- *Watch mechanism.* Clients subscribe to data-node changes;
  ZAB delivers notifications in order.

### Why FIFO ordering matters

Coordination workloads (locks, leader election, group
membership) depend on FIFO order:

- *Lock requests.* The first request in zxid order acquires
  the lock.
- *Group membership.* Joins and leaves serialise.
- *Configuration distribution.* Updates apply in order.

Paxos and Raft also preserve FIFO; ZAB makes it a first-class
property.

### Replacement by Raft

ZooKeeper has been challenged by Raft-based alternatives
(etcd, Consul). Both protocol families have similar guarantees;
the choice is operational. ZAB remains in use where ZooKeeper
is already deployed.

## Formalisation aspects

```text
structure ZabState where
  epoch        : Nat
  history      : List Zxid
  voted_for    : Option NodeId
  state        : ZabPhase  -- Discovery | Sync | Broadcast | Recovery

theorem zab_primary_order
    (op_1 op_2 : Operation) (zxid_1 zxid_2 : Zxid) :
    LeaderProposed op_1 zxid_1 -> LeaderProposed op_2 zxid_2 ->
    zxid_1 < zxid_2 ->
    forall (replica : NodeId), IsHonest replica ->
    Delivered op_1 replica < Delivered op_2 replica := by
  -- Synchronisation phase preserves primary order across
  -- views.
  sorry
```

ZAB's Coq formalisation by Skrzypczak et al. (2018) covers
the core safety properties.

## Verifiability and circuit encoding

**Tag: `friendly`.**

Per-commit ZAB proof: BLS-aggregated `f + 1` ACK cert
~10^6 constraints. Same as Raft and Multi-Paxos.

Synchronisation phase adds: per-replica zxid comparison
(~constraints), longest-prefix selection (~constraints).
~10^4 per Sync phase.

## Known attacks and limitations

- *Fan-out.* All writes go through the leader. Bandwidth-
  limited beyond a certain scale.
- *Snapshot transfer.* Leader-to-follower snapshot transfer
  is bandwidth-intensive; ZooKeeper has tunable parameters.

## Implementation notes

The crate provides a minimal ZAB simulator with:

- Three nodes; one starts as leader.
- Pre-loaded operations.
- PROPOSE/ACK/COMMIT flow.

Tests verify replicas reach the same log.

## References

- Junqueira, Reed, Serafini, "Zab: High-performance broadcast
  for primary-backup systems", DSN 2011.
- Hunt et al., "ZooKeeper: Wait-free Coordination for
  Internet-scale Systems", USENIX ATC 2010.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
