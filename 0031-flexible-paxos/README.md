# 0031: Flexible Paxos

## Historical context

Howard, Malkhi, and Spiegelman published "Flexible Paxos:
Quorum Intersection Revisited" in 2016 (also Howard's PhD
thesis 2019). They observed that classic Paxos's "majority
quorum" requirement is *sufficient* but not *necessary* for
safety: the only essential property is that *Phase 1 quorums
intersect Phase 2 quorums*.

This insight separates the two quorum types:

- *Q1* (Phase 1 quorum): used during leader election.
- *Q2* (Phase 2 quorum): used per commit.

Safety requires `Q1 ∩ Q2 != empty` for every pair. There is
no requirement that `Q1 ∩ Q1` or `Q2 ∩ Q2` be non-empty.

## System and threat model

Same as Multi-Paxos.

## Theory

### Quorum-intersection requirement

For Flexible Paxos to be safe:

```
forall Q1 in P1_quorums, Q2 in P2_quorums:
  Q1 ∩ Q2 != empty
```

In classic Paxos, both `P1_quorums` and `P2_quorums` are
"all majorities of `n`"; majority pairs always intersect. In
Flexible Paxos, the two sets can be different.

### Examples

1. **Classic Paxos.** `P1 = P2 = majorities(n)`. Both have
   size `f + 1`; intersection guaranteed.

2. **Read-optimised.** `P1 = all n acceptors, P2 = any 1
   acceptor`. Phase 2 (frequent) is fast; Phase 1 (leader
   change, rare) is slow. Suitable for read-heavy workloads
   where reads are Phase 2.

3. **Grid quorums.** Acceptors arranged in a `sqrt(n) x
   sqrt(n)` grid. Q1 = any column, Q2 = any row. Each row
   intersects each column.

### Theorem (Flexible Paxos correctness)

For any choice of `(P1, P2)` with `Q1 ∩ Q2 != empty` for all
`(Q1, Q2)`, Flexible Paxos satisfies Synod safety.

*Proof.* Howard et al. 2016. The classic Paxos safety proof
relies only on `Q1 ∩ Q2 != empty`; relaxing the same-class
intersection does not affect the argument.

### Trade-offs

- *Smaller Q2.* Faster commits.
- *Larger Q1.* Slower leader election but allows smaller Q2.

In production: typical asymmetric workloads have leader
election rare (every few hours) and commits frequent (per
operation). Asymmetric quorums are advantageous.

## Practice

### Where Flexible Paxos is used

- *FPaxos.* Reference implementation, Howard's PhD thesis.
- *PaxosStore.* Uses similar quorum flexibility.
- *DynamoDB transactions.* Uses tunable read/write quorums (a
  relaxed analogue).

### Engineering implications

Production systems often hard-code majority quorums for
simplicity. Flexible Paxos is more useful when:

- *Workload is asymmetric.* Heavy commits, rare leader
  elections.
- *Geographic deployment.* Smaller Q2 = lower latency in
  geo-distributed settings.
- *Hardware tiering.* Some replicas have fast storage; route
  Q2 there.

## Formalisation aspects

```text
structure FlexibleQuorum (n : Nat) where
  p1_quorums : Set (Set NodeId)
  p2_quorums : Set (Set NodeId)
  intersect  : forall q1 q2,
    q1 ∈ p1_quorums -> q2 ∈ p2_quorums ->
    (q1 ∩ q2).Nonempty

theorem flexible_paxos_safety
    (FQ : FlexibleQuorum n) :
    SynodSafety := by
  -- Same as classic Paxos but with FQ.intersect
  sorry
```

## Verifiability and circuit encoding

**Tag: `friendly`.**

Verifiable Flexible Paxos: per-commit cost is determined by Q2
size, not n.

- Q2 = 1: ~3k constraints per commit (one signature).
- Q2 = majority: ~10^6 (BLS aggregate).
- Q2 = all: ~10^6 (BLS aggregate, larger threshold).

The trade-off: smaller Q2 -> cheaper proofs but larger Q1
-> more expensive leader-change proofs.

Mina's Pickles uses a similar idea: "fast path" with smaller
quorum, fallback to slow path with full quorum.

## Known attacks and limitations

- *Q1 must be large.* If `Q1 = majority` and `Q2 = 1`, leader
  election needs `f + 1` acceptors all available. Production
  must monitor Q1 availability.
- *No single optimal choice.* Workload-dependent. Operators
  must measure.

## Implementation notes

The crate provides a `FlexibleQuorums` struct with explicit
P1/P2 quorum sets and a `verify_intersection` method.

## References

- Howard, Malkhi, Spiegelman, "Flexible Paxos: Quorum
  Intersection Revisited", arXiv 2016.
- Howard, "Distributed Consensus Revised", PhD thesis 2019.

See also [`HISTORY.md`](../HISTORY.md), section "2015 to 2019".
