# 0021: Viewstamped Replication Revisited (2012)

## Historical context

Liskov and Cowling published "Viewstamped Replication
Revisited" in 2012 (MIT-CSAIL-TR-2012-021). The paper
modernises the original 1988 VR exposition (module 0020) with
clearer state diagrams, explicit recovery and reconfiguration
protocols, and contemporary systems vocabulary. It also
introduces the *VRR* variant that explicitly separates the
SMR layer from the application layer.

The 2012 version is the canonical modern reference. It
clarifies several points that the 1988 paper left implicit:

- The exact set of fields that must be persisted on stable
  storage (`view_num`, `op_num`, `commit_num`, last accepted
  view-change vote).
- The recovery protocol's interaction with view changes.
- A reconfiguration protocol for changing the replica set
  (modelled as a special operation in the SMR).

## System and threat model

Same as VR (module 0020): asynchronous network, crash-recovery
with stable storage, `f < n / 2`. The 2012 paper formalises
adaptive client behaviour and reply consistency.

## Theory

### Differences from VR 1988

| Aspect              | VR 1988            | VR Revisited 2012     |
| ------------------- | ------------------ | --------------------- |
| Exposition          | dense, implicit    | explicit state machine|
| Persistence         | informal           | listed fields         |
| Recovery            | sketch only        | full protocol         |
| Reconfiguration     | absent             | special operation     |
| Client retries      | implicit           | explicit dedup table  |

### Reconfiguration

The 2012 paper adds a *reconfiguration* operation that lets
the replica set change. It is modelled as a special SMR
operation:

```
client -> primary: REQUEST(reconfigure(new_replica_set))
primary: orders the reconfiguration like any other operation
on commit:
  the new replica set takes over starting view v + 1
  old replicas transfer state to new replicas
  new replicas catch up via the standard recovery protocol
```

Reconfiguration is the missing piece for production: real
systems must add and remove replicas as hardware fails, scales,
or rotates.

### State diagram

```
+--------+   primary fails   +-------------+
| Normal | ----------------> | ViewChange  |
+--------+                   +-------------+
   ^                              |
   | new view installed           |
   +------------------------------+
   ^
   |
+----------+
| Recovery |
+----------+
   ^
   | crash + restart
   |
[external]
```

### Theorem (VR Revisited correctness)

Under crash-recovery with stable storage and `f < n / 2`, the
2012 VR satisfies linearisability and liveness (under partial
synchrony). The proof is a refinement of the 1988 argument
with explicit handling of the reconfiguration operation.

## Practice

### What to persist (canonical list)

Per Liskov-Cowling 2012, Section 4.1:

- `view_num`
- `op_num`
- `commit_num`
- `log[op_num - 1] = (view, op)`  the latest accepted op
- `last_view_change_voted_for`

These are the analogues of Raft's `currentTerm`, `votedFor`,
and `log[]` (Raft 2014 explicitly lists the same fields).

### Why VR Revisited matters

- *Engineering reference.* The 2012 paper is the most
  comprehensive description of a primary-backup SMR with full
  recovery and reconfiguration.
- *Pedagogy.* Used in Liskov's MIT 6.824 course as the
  canonical example.
- *Production lineage.* Many systems (e.g. early PaaS
  databases) implement VR Revisited directly rather than
  Paxos.

## Formalisation aspects

The state diagram refinement vs 1988 makes VR Revisited a
slightly easier formalisation target: each phase is an
explicit LTS state, transitions are labelled by message types
and persisted-field updates.

```text
inductive VrStatus where
  | normal
  | view_change
  | recovering
  | reconfiguring (new_set : Finset NodeId)

structure VrRevState where
  status         : VrStatus
  view_num       : Nat
  op_num         : Nat
  commit_num     : Nat
  log            : List (View × Operation)
  last_dvc_vote  : Option Nat
  client_table   : Map ClientId (RequestId × Result)
```

CSLib's `LTS` covers it; the reconfiguration becomes an LTS
sub-machine (a "state-modifying" transition).

## Verifiability and circuit encoding

**Tag: `friendly`.**

VR Revisited's verifiability is identical to VR (module 0020).
Reconfiguration adds:

- Reconfiguration commit: signed by current primary, ~3k
  constraints.
- New replicas' acknowledgements: BLS-aggregated, ~10^6.
- View transition into the new set: same as a regular view
  change.

A verifiable reconfiguration demonstrates that the new replica
set has been authorised by a quorum of the old set; this is
the analogue of a "proof of stake transition" in PoS chains
(module 0049).

## Known attacks and limitations

- *Reconfiguration concurrency.* Concurrent reconfigurations
  can confuse the replica state. The 2012 paper handles this
  by serialising via the SMR.
- *Recovery complexity.* The recovery protocol must
  distinguish between "initial recovery" (just crashed) and
  "recovery during view change" (more complex). Production
  systems often have subtle bugs here.

## Implementation notes

The crate refers to module 0020's implementation as the base.
This module adds a *reconfiguration* test stub that
demonstrates the protocol's intent without a full
reconfiguration implementation.

## References

- Liskov and Cowling, "Viewstamped Replication Revisited",
  MIT-CSAIL-TR-2012-021, 2012.
- Oki and Liskov, "Viewstamped Replication", PODC 1988.
- Ongaro, "Consensus: Bridging Theory and Practice", PhD
  thesis 2014 (Raft).

See also [`HISTORY.md`](../HISTORY.md), section "1986 to 1999".
