# 0020: Viewstamped Replication

## Historical context

Brian Oki and Barbara Liskov published "Viewstamped Replication:
A New Primary Copy Method to Support Highly Available
Distributed Systems" at PODC 1988. The paper introduced *VR*,
the first practical SMR protocol with explicit primary-backup
structure, view changes, and a complete handling of crash-
recovery. VR was the foundation for the Argus distributed
system (Liskov 1988).

In 2012, Liskov and Cowling published "Viewstamped Replication
Revisited" (MIT TR), modernising the exposition and aligning
the protocol with contemporary systems language. The 2012
version is the canonical reference.

VR and Paxos were developed concurrently and independently. As
Lamport's Paxos took longer to publish (1998), VR was
historically the first published SMR protocol; in retrospect,
the two are functionally equivalent, with different
expositional choices.

## System and threat model

- **Network.** Asynchronous, reliable point-to-point.
- **Failures.** Crash-recovery with stable storage. `f < n / 2`
  crashes tolerated.
- **Cryptography.** Authenticated channels (MAC or signatures).
- **Goal.** Linearisable SMR over arbitrary state machines.

## Theory

### Configuration

The set of `n` replicas is divided per *view*:

- *View number `v`.* A view is a configuration. Initially
  `v = 0`.
- *Primary.* In view `v`, the primary is replica
  `v mod n`.
- *Backups.* All other replicas in view `v`.

### Protocol structure

VR has three sub-protocols:

1. **Normal-case operation.** The primary orders client
   requests and broadcasts them; backups acknowledge.
2. **View change.** When the primary is suspected, replicas
   elect a new primary and reconcile state.
3. **Recovery.** A crashed-recovered replica reconstructs its
   state from peers.

### Normal-case operation

```
client -> primary: REQUEST(op, client_id, request_id)
primary: assigns op number `op_no`, view `v`
primary -> backups: PREPARE(v, op_no, op, commit_no)
each backup: appends to log, replies PREPARE_OK
primary: collects f PREPARE_OK responses (quorum = f + 1
         including primary)
primary: marks op committed; advances commit_no
primary -> client: REPLY(result)
```

The primary's quorum (`f + 1` including itself) ensures that
any committed op is observed by a majority. The view-change
protocol relies on this for safety.

### View change

When backups suspect the primary (timeouts), they initiate a
view change to view `v + 1`:

```
each backup: sends START_VIEW_CHANGE(v + 1)
on receiving f + 1 START_VIEW_CHANGE:
  send DO_VIEW_CHANGE(v + 1, log) to new primary
new primary (v + 1):
  on receiving f + 1 DO_VIEW_CHANGE:
    selects the longest accepted log among the f + 1
    sends START_VIEW(v + 1, log)
backups (v + 1): adopt the new view's log; resume normal
operation
```

The "longest accepted log" rule preserves committed
operations: any op committed in a previous view is on at least
`f + 1` replicas' logs, so at least one of the `f + 1`
DO_VIEW_CHANGE messages contains it.

### Theorem (VR safety)

VR satisfies linearisability under crash-recovery with stable
storage and `f < n / 2`.

*Proof.* Liskov-Cowling 2012, Theorem 1. The core invariant:
after view change, the new primary's log contains every
operation committed in any earlier view. This follows from the
quorum-intersection property of `f + 1` quorums in `n =
2f + 1`.

### Theorem (VR liveness)

Under partial synchrony with `f < n / 2`, VR's view-change
mechanism eventually elects a stable primary and consensus
proceeds.

### Equivalence to Paxos

Van Renesse-Altinbuken 2014, "Paxos Made Moderately Complex",
explicitly maps VR to Multi-Paxos:

- VR's "view" = Paxos's "ballot".
- VR's "view change" = Paxos's "Phase 1" (prepare/promise).
- VR's "normal case" = Paxos's "Phase 2" (accept/accepted).
- VR's "primary" = Paxos's "leader" (a long-running ballot
  proposer).

The two protocols are structurally identical; the difference
is exposition. VR's primary-backup framing is more familiar
to systems engineers; Paxos's ballot/proposer framing is more
amenable to formal analysis.

## Practice

### Where VR (and its descendants) show up

- *Argus.* MIT's distributed object system, the original VR
  deployment.
- *Cassandra's Lightweight Transactions.* Built on a Paxos
  variant similar to VR.
- *Modern descendants.* Most production systems use Raft (a
  Paxos descendant explicitly designed for understandability)
  rather than VR proper. Raft inherits VR's view (= term)
  structure.

### View-change tuning

Production trade-offs:

- *Aggressive timeouts.* Short timeouts trigger spurious view
  changes, hurting throughput.
- *Slow timeouts.* Long timeouts delay failover under genuine
  primary failure.
- *Heuristics.* Most production systems use exponential
  back-off with a base `~10 * RTT` (Raft, etcd, ZooKeeper).

## Formalisation aspects

```text
structure VrState where
  view_num    : Nat
  op_num      : Nat
  commit_num  : Nat
  log         : List Operation
  status      : Status -- Normal | ViewChange | Recovering

theorem vr_safety
    (n f : Nat) (h : 2 * f + 1 = n)
    (sched : AsyncSchedule)
    (corrupt : CrashRecovery f) :
    Linearisable VR sched corrupt := by
  -- Inductive invariant: at every view boundary, the new
  -- primary's log contains every previously-committed op.
  sorry
```

CSLib's `LTS` framework with view-indexed transitions is the
natural target. The view-change is a *labelled transition* in
the LTS; safety is preserved across this transition by the
quorum-intersection invariant.

## Verifiability and circuit encoding

**Tag: `friendly`.**

VR's normal-case operation is straightforward in circuit:

- PREPARE: signed by primary, ~3k constraints.
- PREPARE_OK from each backup: BLS-aggregated quorum cert,
  `~10^6` constraints per QC.
- COMMIT marker: derived from QC.

View change adds:

- Each backup's signed DO_VIEW_CHANGE: ~3k each, `O(n)` total.
- New primary's selection of longest log: ~constraints
  proportional to log length.

Total per commit: `~10^6` constraints; per view change:
`~3 * 10^6` constraints. Recursion across slots gives
constant proof size.

VR's verifiability profile is essentially the same as
multi-Paxos's: a single signed leader publishes a quorum
certificate per commit; the verifier checks one pairing.

## Known attacks and limitations

- *Coordinator centralisation.* VR concentrates work at the
  primary; under high load, the primary becomes a bottleneck.
  Multi-master variants (EPaxos, Mencius) address this.
- *View-change cost.* Each view change costs `O(n^2)` messages
  in the naive implementation. Production amortises this with
  pipelining.
- *Recovery cost.* A recovering replica must catch up with all
  intermediate operations. Snapshotting bounds this.

## Implementation notes

The crate provides a minimal VR over the simulator:

- Three replica nodes, primary at index 0.
- Client requests are pre-loaded as a queue at the primary.
- Backups acknowledge; primary commits on `f + 1` acks.

The view-change protocol is sketched in comments but not fully
implemented; doing so requires a more elaborate state machine
than the per-module space allows. Module 0021 (VR Revisited)
adds the view-change details.

## References

- Oki and Liskov, "Viewstamped Replication: A New Primary Copy
  Method", PODC 1988.
- Liskov and Cowling, "Viewstamped Replication Revisited", MIT
  Tech Report MIT-CSAIL-TR-2012-021, 2012.
- van Renesse and Altinbuken, "Paxos Made Moderately Complex",
  ACM Computing Surveys 2015.

See also [`HISTORY.md`](../HISTORY.md), section "1986 to 1999".
