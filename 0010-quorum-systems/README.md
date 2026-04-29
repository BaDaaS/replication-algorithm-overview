# 0010: Quorum Systems and Intersection

## Historical context

Quorum systems were introduced by Garcia-Molina and Barbara
1985 as a way to coordinate replicated data without requiring
universal participation. Malkhi and Reiter generalised them to
Byzantine settings in 1998 [`mr1998quorum`]; Vukolic's 2012
survey [`vukolic2012`] is the canonical modern reference.

A *quorum system* is a family `Q` of subsets of a process set
`U` (the *quorums*) such that any two quorums intersect. The
intersection guarantees a *witness*: any decision committed by
one quorum is observed by every other quorum, so subsequent
operations cannot contradict it.

Quorum systems are the structural primitive behind every BFT
protocol. The `2f + 1` of `3f + 1` quorum is the most common,
but variants exist for different fault models, geographic
hierarchies, and randomised committees.

## System and threat model

Parametric in the failure model. Specific results below quantify
intersection bounds for crash-stop and Byzantine.

## Theory

### Definition (quorum system)

A *quorum system* over a universe `U = {p_1, ..., p_n}` is a
non-empty family `Q ⊂ 2^U` such that:

- **(Intersection.)** For all `Q_1, Q_2 in Q`, `Q_1 ∩ Q_2 ≠ ∅`.

The *intersection bound* is the smallest size of any pairwise
intersection: `inter(Q) := min_{Q_1, Q_2 in Q} |Q_1 ∩ Q_2|`.

### Definition (Byzantine quorum system)

A quorum system is *`f`-Byzantine* if for any `Q_1, Q_2 in Q`,
`|Q_1 ∩ Q_2| > f`. Equivalently, every pairwise intersection
contains at least one honest process beyond what the adversary
controls.

### Theorem (threshold quorum)

For `n = 3f + 1` and `Q := { S ⊂ U : |S| = 2f + 1 }`, `Q` is
an `f`-Byzantine quorum system. Specifically,
`|Q_1 ∩ Q_2| >= 2 (2f + 1) - n = f + 1` for all `Q_1, Q_2 in Q`.

*Proof.* `|Q_1 ∩ Q_2| = |Q_1| + |Q_2| - |Q_1 ∪ Q_2| >=
(2f + 1) + (2f + 1) - n = 4f + 2 - 3f - 1 = f + 1`. QED.

### Theorem (consensus from intersection)

A consensus protocol's safety reduces to the existence of a
quorum system with `inter(Q) > f`. The protocol commits a
proposal once a quorum has accepted it; any later quorum's
intersection with the committing quorum contains an honest
process whose vote prevents inconsistent commitments.

This is the core invariant of PBFT (`prepare` and `commit`
quorums of `2f + 1`), HotStuff (QC of `2f + 1`), Tendermint
(`2/3` voting power), and most other BFT protocols.

### Refinements

- *Crash-stop.* `f`-resilient quorum: `|Q| >= n / 2 + 1`. The
  intersection is non-empty (Garcia-Molina-Barbara 1985).
- *Weighted quorums.* Each process `p_i` has weight `w_i`. A
  quorum is a subset of *total weight* `>= W / 2 + 1`. Used in
  Stellar SCP and in PoS protocols where stake = weight.
- *Geographic quorums.* Quorums must contain processes from
  multiple regions. Used in Spanner for global consistency.
- *Probabilistic quorums.* Random subsets of size
  `O(sqrt(n))` intersect with high probability. Used in
  scalable consensus (Algorand committees).

## Practice

### How quorums show up in production

| Protocol         | Quorum size       | Justification         |
| ---------------- | ----------------- | --------------------- |
| Paxos            | majority `n/2 + 1`| crash, intersect 1    |
| Raft             | majority          | crash                 |
| PBFT prepare     | `2f + 1`          | Byzantine intersect 1 |
| PBFT commit      | `2f + 1`          | Byzantine intersect 1 |
| HotStuff QC      | `2f + 1`          | Byzantine intersect 1 |
| Tendermint       | `2/3 + 1`         | weighted              |
| Algorand         | random committee  | probabilistic         |
| Cosmos           | `2/3` voting power| weighted (PoS)        |
| Bitcoin          | implicit (longest)| computational         |

### Stellar's federated quorums

Stellar SCP (Mazieres 2016) introduces *quorum slices*: each
process locally specifies a set of acceptable quorums. The
global quorum system emerges from intersecting slices. This is
the core innovation of *federated Byzantine agreement* and is
covered in module 0110.

### Probabilistic quorums

Algorand uses cryptographic sortition to elect a random
committee per round. With `n = 10000` validators and committee
size `k = 1000`, two random committees intersect in expectation
`k^2 / n = 100` processes; with probability close to 1, this
intersection contains an honest majority. The benefit: each
round only requires `O(k)` participation, not `O(n)`.

## Formalisation aspects

```text
structure QuorumSystem (U : Finset alpha) where
  quorums      : Finset (Finset alpha)
  intersection : forall Q1 Q2 : Finset alpha,
    Q1 ∈ quorums -> Q2 ∈ quorums -> (Q1 ∩ Q2).Nonempty

def IntersectionBound
    (qs : QuorumSystem U) : Nat :=
  Finset.min' (qs.quorums.product qs.quorums)
    (fun (Q1, Q2) => (Q1 ∩ Q2).card)

theorem byzantine_threshold_quorum
    (n f : Nat) (h : n = 3 * f + 1) (U : Finset alpha)
    (h_card : U.card = n) :
    let Q := { S | S ⊆ U /\ S.card = 2 * f + 1 }
    forall Q1 Q2 : Finset alpha, Q1 ∈ Q -> Q2 ∈ Q ->
      (Q1 ∩ Q2).card >= f + 1 := by
  sorry  -- inclusion-exclusion
```

cslib has no direct quorum-system support; Mathlib's `Finset`
machinery is the natural foundation. The intersection theorem
is straightforward inclusion-exclusion (`Finset.card_inter`).

## Verifiability and circuit encoding

**Tag: `friendly`.**

Quorum verification is the dominant SNARK constraint in
authenticated BFT. With BLS aggregation:

- A *quorum certificate* is a pair `(content, agg_sig,
  signers_bitmap)` with `signers_bitmap` having popcount
  `>= 2f + 1`.
- Verifier work: one BLS pairing (~`10^6` constraints), plus
  bitmap popcount (`O(n)` constraints), plus the public-key
  product (`~n` group additions, `~n * 100` constraints).

Total: `~10^6 + 100n` constraints per QC verification. For
`n = 100`, this is `~1.1 * 10^6` constraints.

Without aggregation, a quorum check is `2f + 1` independent
signature verifications: `~10^7` constraints for `f = 33`. The
aggregate gain is one order of magnitude.

Mina, Aleo, and Ethereum's Casper FFG all use BLS-aggregated
quorum certificates for this reason. Modern recursive proof
systems (Pickles, Halo 2, Nova) verify these in constant size
across many slots.

## Known attacks and limitations

- *Below the intersection threshold.* Below `n - 2f`
  intersection, two quorums can have an entirely Byzantine
  intersection: safety fails. The threshold is the boundary.
- *Adaptive corruption.* If the adversary can corrupt any
  member of any quorum, the static threshold may be
  insufficient. Adaptive-resistant constructions use single-
  secret leader election or VRF-based committees.
- *Heterogeneous trust.* Federated Byzantine agreement
  (Stellar) drops the global threshold in favour of
  per-process slices, requiring a more elaborate intersection
  argument (module 0110).

## Implementation notes

The crate provides:

- A `ThresholdQuorum { n, threshold }` type that classifies a
  given `Vec<NodeId>` as a quorum.
- A `verify_intersection` function that checks the pairwise
  intersection bound.
- A simulator-side test that builds a leader-broadcast SMR with
  `n = 4, f = 1, threshold = 3` and confirms that, with at
  least one quorum agreeing on a value, all other quorums
  share at least one honest member.

## References

- Garcia-Molina and Barbara, "How to Assign Votes in a
  Distributed System", JACM 1985.
- Malkhi and Reiter, "Byzantine Quorum Systems", Distributed
  Computing 1998. [`mr1998quorum`].
- Vukolic, "Quorum Systems: With Applications to Storage and
  Consensus", Morgan & Claypool 2012. [`vukolic2012`].
- Mazieres, "The Stellar Consensus Protocol", Stellar
  whitepaper 2016.

See also [`HISTORY.md`](../HISTORY.md), section "1986 to 1999".
