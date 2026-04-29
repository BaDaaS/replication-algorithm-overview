# 0033: Raft

## Historical context

Ongaro and Ousterhout's 2014 USENIX ATC paper "In Search of an
Understandable Consensus Algorithm" introduced Raft. The
explicit motivation was Paxos's reputation for being hard to
teach and implement; Raft achieves the same correctness via a
simpler exposition built around three independent
sub-protocols: leader election, log replication, and safety.

Raft has become the default crash-fault SMR for production
systems. Implementations include etcd (Kubernetes), Consul,
TiKV, CockroachDB, Hyperledger Fabric ordering, M3DB, and
hundreds of smaller projects. Diego Ongaro's Stanford PhD
thesis (2014) is the canonical reference.

## System and threat model

- **Network.** Asynchronous, reliable point-to-point.
- **Failures.** Crash-recovery with stable storage; `f < n / 2`.
- **Cryptography.** Authenticated channels.
- **Goal.** Linearisable SMR.

## Theory

### Server states

Each server is in exactly one state:

- *Follower.* Default state. Accepts log entries from leader.
- *Candidate.* Tries to become leader by collecting votes.
- *Leader.* Accepts client requests; replicates log entries.

State transitions:

```
+----------+     timeout       +-----------+    win election    +--------+
| Follower | -----------------> | Candidate | ------------------> | Leader |
+----------+                    +-----------+                    +--------+
     ^                              |                                |
     |  higher term seen,           |  higher term seen,             |  step down
     +------------------------------+--------------------------------+
```

### Term

A *term* is a logical period during which there is at most
one leader. Each term has a monotone integer label; terms are
the analogue of Paxos ballots.

### Leader election

```
on follower timeout:
  increment term
  vote for self
  send RequestVote(term, last_log_index, last_log_term) to all peers
  wait for majority votes:
    if win: become leader
    if higher term seen: step down to follower
    if timeout: increment term, retry

on RequestVote(term, log_index, log_term):
  if term < current_term: reject
  if (already voted in this term and not for them): reject
  if our log is more up-to-date: reject
  else: vote yes
```

### Log replication (AppendEntries)

```
leader on client request op:
  append (current_term, op) to local log
  for each follower:
    send AppendEntries(term, prev_index, prev_term, entries, leader_commit)
  collect majority Successes:
    advance commit_index
    apply newly committed entries to state machine

follower on AppendEntries:
  if term < current_term: reject
  if log doesn't match prev_index, prev_term: reject (leader retries with earlier index)
  else: append entries; update commit_index; respond Success
```

### Theorem (Raft safety)

Raft satisfies five core safety properties (Ongaro 2014, Ch.
3):

- *Election Safety.* At most one leader per term.
- *Leader Append-Only.* A leader never modifies its own log.
- *Log Matching.* If two logs contain an entry with the same
  index and term, the logs are identical up to that index.
- *Leader Completeness.* If an entry is committed in term `t`,
  it appears in every leader's log for terms `>= t`.
- *State Machine Safety.* If a server applies log entry at
  index `i`, no other server applies a different entry at
  `i`.

Together, these imply linearisability.

*Proof.* Ongaro 2014, Section 3.6, with TLA+ specification in
appendix.

### Membership change (joint consensus)

To safely change the replica set, Raft uses *joint consensus*:

- Phase 1: commit the configuration change as a *joint*
  configuration `C_old + C_new` (decisions need majorities in
  *both* old and new).
- Phase 2: commit the new configuration `C_new` alone.

Joint consensus prevents safety violations during the
transition.

### Comparison to Multi-Paxos

| Aspect              | Multi-Paxos        | Raft                 |
| ------------------- | ------------------ | -------------------- |
| Term/ballot         | ballot             | term                 |
| Phase 1 (election)  | prepare/promise    | RequestVote          |
| Phase 2 (commit)    | accept/accepted    | AppendEntries        |
| Log structure       | per-slot Paxos     | per-leader log       |
| Leader role         | implicit           | explicit             |
| Reconfiguration     | via SMR op         | joint consensus      |
| Tutorial accessibility | low             | high                 |

The two protocols have the same correctness, but Raft's
explicit structuring is easier to teach and implement.

## Practice

### Production implementations

- *etcd / Kubernetes.* etcd is Raft; Kubernetes uses etcd as
  its control plane.
- *Consul.* HashiCorp's service-discovery and config tool.
- *TiKV / TiDB.* Distributed transactional KV / SQL.
- *CockroachDB.* Per-range Raft groups.
- *Hyperledger Fabric.* Ordering service.
- *Aptos / Sui.* Use Raft-derived BFT (DiemBFT, HotStuff
  family) but inherit Raft's exposition.

### Configuration

Raft has just three timeouts to tune:

- `election_timeout`: typical 150-300ms.
- `heartbeat_interval`: typical 50ms (must be < election
  timeout).
- `apply_interval`: how often to apply committed entries.

The "election timeout randomisation" (each follower picks a
random timeout in a range) prevents repeated split votes.

### Operational notes

- *Fsync per AppendEntries.* Followers must fsync the log
  before responding. Production batches.
- *Snapshotting.* Logs grow; periodic snapshots truncate.
- *Pre-vote optimisation.* A candidate first asks if peers
  would vote *if* it requested. Avoids term churn under
  network partitions.

## Formalisation aspects

Raft has been formally verified in Coq (Verdi project, 2015):

```text
structure RaftState where
  current_term  : Nat
  voted_for     : Option NodeId
  log           : List (Term × Operation)
  commit_index  : Nat
  state         : ServerState  -- Follower | Candidate | Leader

theorem raft_state_machine_safety
    (s1 s2 : RaftState) (i : Nat) :
    Applied s1 i v1 -> Applied s2 i v2 -> v1 = v2 := by
  -- Verdi's proof: composition of the 5 invariants.
  sorry
```

Verdi's Coq formalisation runs to ~50000 lines and is the
gold standard for distributed-system mechanical
verification. A Lean port is in progress in cslib.

## Verifiability and circuit encoding

**Tag: `friendly`.**

Per-commit Raft proof:

- AppendEntries quorum cert (BLS aggregate of `f + 1`
  Successes): ~10^6 constraints.
- Log-matching proof: a Merkle path from the new entry to
  the previous log root, ~10k constraints.

Total per commit: ~10^6 constraints (BLS-dominated). Identical
to Multi-Paxos's profile.

Joint-consensus reconfiguration adds ~3 * 10^6 (two phases
each ~10^6, plus state transfer).

## Known attacks and limitations

- *Leader bottleneck.* All requests go through the leader.
  Multi-Raft (one leader per range) addresses this in
  CockroachDB.
- *Network partitions.* The minority partition cannot make
  progress; production includes pre-vote and timeout
  randomisation to mitigate.
- *Snapshot transfer cost.* New replicas must catch up via
  snapshot + log; bandwidth-intensive.

## Implementation notes

The crate provides a minimal Raft simulator with:

- Three nodes; one starts as leader (term 1).
- Pre-loaded operations at the leader.
- AppendEntries flow.

Tests verify all replicas reach the same log.

Full Raft (with leader election, snapshotting, joint
consensus) requires substantially more state machinery; we
defer to the etcd-raft and tikv/raft-rs codebases for
production-quality reference implementations.

## References

- Ongaro and Ousterhout, "In Search of an Understandable
  Consensus Algorithm", USENIX ATC 2014.
- Ongaro, "Consensus: Bridging Theory and Practice", Stanford
  PhD thesis 2014.
- Wilcox et al., "Verdi: A Framework for Implementing and
  Formally Verifying Distributed Systems", PLDI 2015.

See also [`HISTORY.md`](../HISTORY.md), section "2009 to 2014".
