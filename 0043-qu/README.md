# 0043: Q/U -- Quorum-based BFT

## Historical context

Abd-El-Malek, Ganger, Goodson, Reiter, Wylie published "Fault-
Scalable Byzantine Fault-Tolerant Services" at SOSP 2005.
Q/U (pronounced "queue") is a leaderless BFT protocol: each
client operation runs against a Byzantine quorum without a
designated primary. The novelty: no view change, no leader
bottleneck. Trade-off: requires `n >= 5f + 1` (more replicas)
and assumes contention-free workloads.

## System and threat model

- **Network.** Asynchronous.
- **Failures.** Byzantine, `f` of `n`, with `n >= 5f + 1`.
- **Goal.** Linearisable BFT storage for non-conflicting
  operations.

## Theory

### Algorithm

Each replica stores per-object: latest accepted operation,
plus a *generation number*.

```
client write(obj, op):
  read latest op-state from a 4f+1 quorum
  pick a new generation number
  send (op, gen) to all replicas
  wait for f+1 acceptances
  commit
```

Conflict resolution: if two clients write concurrently, both
"lose" and must retry with refreshed state.

### Quorum constraints

- *Read quorum.* `4f + 1` (so honest majority observes the
  latest).
- *Write quorum.* `4f + 1` (same; ensures replicated
  durability).

The `5f + 1` total is required so that `4f + 1` quorums
intersect in `3f + 2 > 2f`, with `f + 1` honest in the
intersection.

### Theorem (Q/U correctness)

Under `n >= 5f + 1`, Q/U satisfies linearisability for non-
conflicting operations.

For conflicting operations: clients retry; eventually one
succeeds (probabilistic termination under contention).

### Performance

- *Best case.* No contention: 1 round-trip.
- *Worst case.* High contention: livelock-prone.

In contention-free workloads, Q/U is faster than PBFT because
it avoids the three-phase commit. In contended workloads,
PBFT's leader-mediated approach wins.

## Practice

### Where Q/U-style protocols show up

- *Storage systems with disjoint-key workloads.* Each key
  rarely contended.
- *Aliph (HQ, Zyzzyva, Q/U).* The "fast path" inspiration
  for many later BFT protocols.

Q/U itself is rarely deployed; its lessons (high `n` for
no-leader) influenced the field.

## Verifiability and circuit encoding

**Tag: `friendly`.**

Per-write proof: BLS aggregate from 4f + 1 replicas
~10^6 constraints + generation-number consistency.

## Known attacks and limitations

- *Contention-sensitive.* Performance degrades sharply under
  conflicts.
- *Larger replica count.* `5f + 1` vs PBFT's `3f + 1` is a
  significant deployment cost.

## Implementation notes

Conceptual module.

## References

- Abd-El-Malek, Ganger, Goodson, Reiter, Wylie, "Fault-
  Scalable Byzantine Fault-Tolerant Services", SOSP 2005.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
