# 0027: EPaxos -- Egalitarian Paxos

## Historical context

Moraru, Andersen, and Kaminsky published "There Is More
Consensus in Egalitarian Parliaments" at SOSP 2013. EPaxos is
a multi-leader Paxos derivative that exploits Generalized
Paxos's commutativity. Every replica can act as a *command
leader* for its own incoming requests; conflicts (non-
commuting commands) trigger a coordination round.

The result: in geo-distributed settings, clients can submit to
the nearest replica, halving the typical commit latency. In
contention-free workloads, EPaxos commits each operation in 1
round-trip with no leader bottleneck.

## System and threat model

- **Network.** Asynchronous, reliable.
- **Failures.** Crash-recovery; `f < n / 2`.
- **Goal.** Multi-leader SMR with low typical-case latency.

## Theory

### Multi-leader design

Every replica is a potential leader. When replica `R` receives
a command `c`:

1. *Pre-accept phase.* `R` broadcasts `(c, R, deps)` where
   `deps` is the set of commands `c` depends on (commands `c`
   does not commute with). Recipients reply with their own
   computed `deps`.
2. *Fast path.* If a quorum of `f + ceil((f+1)/2)` replies
   have identical `deps`, commit on the fast path.
3. *Slow path.* If replies disagree, `R` runs a Paxos-style
   accept on the *union* of all `deps`.

### Quorum sizes

EPaxos uses two quorum sizes:

- *Fast quorum:* `f + floor(f/2) + 1`. For `n = 5, f = 2`,
  this is `4`.
- *Slow quorum:* `f + 1` (majority). For `n = 5, f = 2`, this
  is `3`.

The `fast quorum > slow quorum` invariant ensures any fast
decision is observed by any slow recovery.

### Theorem (EPaxos correctness)

EPaxos satisfies linearisability under crash-recovery and `f <
n / 2`, in the sense that any topological sort of the decided
dependency graph gives a valid linearisation.

*Proof.* Moraru-Andersen-Kaminsky 2013, Theorem 2. The proof
extends Generalized Paxos's correctness with the multi-leader
machinery: each command's decided `deps` is unique, and the
overall dependency graph is acyclic.

### Geographic latency

EPaxos's selling point is geographic deployment:

- *Classic Paxos with leader in US.* Clients in EU and Asia
  pay round-trip latency to US.
- *EPaxos.* Clients submit to the nearest replica; commit
  involves only nearby replicas (assuming no contention).

For workloads with low cross-region conflict rates, EPaxos
typically achieves 30-50% lower latency than classic
Multi-Paxos in geo-distributed deployments.

## Practice

### Where EPaxos shows up

- *CockroachDB.* Considered EPaxos for cross-region
  transactions; opted for simpler designs in production.
- *PaxosStore.* Uses Generalized Paxos ideas but not full
  EPaxos.
- *Research.* EPaxos is a benchmark for multi-leader
  protocols; subsequent work (Atlas 2020, Caesar 2017)
  refines it.

### Why production rarely deploys EPaxos

- *Complexity.* EPaxos requires per-command dependency
  tracking; the resulting state machine is more complex than
  Multi-Paxos.
- *Worst-case throughput.* Under high contention, EPaxos's
  slow path is more expensive than classic Paxos.
- *Operational overhead.* Conflict detection requires
  per-application commutativity definitions.

## Formalisation aspects

```text
structure EPaxosCommand where
  cmd       : Command
  leader    : NodeId
  deps      : Set CommandId
  status    : EPaxosStatus  -- PreAccepted | Accepted | Committed | Executed

theorem epaxos_correctness :
    forall (sched : AsyncSchedule),
      Linearisable EPaxos sched := by
  -- Per-command dependency graph is acyclic; topological sort
  -- gives a linearisation.
  sorry
```

## Verifiability and circuit encoding

**Tag: `partial`.**

EPaxos's verifiability is more challenging than classic Paxos:

- Per-command BLS aggregate cert: ~10^6 constraints.
- Dependency-graph encoding: depends on graph size; for `k`
  commands with `O(k)` dependencies each, `~10^4` constraints
  per command.
- Topological-sort verification: `O(k log k)` constraints.

For per-command verification: ~10^6 constraints (dominated by
BLS). For batch verification with explicit graph: `~10^7`
constraints for `k = 100` commands.

Production verifiable rollups using EPaxos-style consensus
would face significant prover cost from the dependency graph.

## Known attacks and limitations

- *Conflict-rate sensitivity.* High conflict workloads
  degrade to slow-path Paxos. Production must measure conflict
  rate.
- *Dependency-tracking overhead.* The per-command `deps` set
  grows over time; periodic GC is required.
- *Recovery complexity.* Multi-leader recovery is more
  complex than single-leader Paxos.

## Implementation notes

EPaxos is non-trivial to implement faithfully. This module
provides a simplified two-replica simulator with a single
command exchange. Full EPaxos is in the original paper and the
authors' epaxos-go reference implementation.

## References

- Moraru, Andersen, Kaminsky, "There Is More Consensus in
  Egalitarian Parliaments", SOSP 2013.
- Whittaker et al., "Atlas: Atomic, Latency-Optimal Asynchronous
  Distributed Consensus", 2020 (EPaxos descendant).

See also [`HISTORY.md`](../HISTORY.md), section "2009 to 2014".
