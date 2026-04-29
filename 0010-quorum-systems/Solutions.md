# Module 0010 Solutions

## Solution 1 [T]: threshold inclusion-exclusion

`|Q_1 ∩ Q_2| = |Q_1| + |Q_2| - |Q_1 ∪ Q_2|`. Both quorums lie
in `U` of size `n = 3f + 1`, so `|Q_1 ∪ Q_2| <= n`.
`|Q_1| = |Q_2| = 2f + 1`, so

```
|Q_1 ∩ Q_2| >= (2f + 1) + (2f + 1) - n = 4f + 2 - 3f - 1 = f + 1.
```

QED.

## Solution 2 [T]: tightness at `n = 3f`

With `threshold = 2f` and `n = 3f`:

```
|Q_1 ∩ Q_2| >= 2 * 2f - 3f = f.
```

`f` Byzantine processes can occupy the entire intersection,
giving an all-Byzantine intersection. Safety fails: two
quorums can each reach a different commitment, both legitimate
to the protocol, contradicting agreement.

## Solution 3 [T]: weighted threshold

Let `W := sum_i w_i`. A weighted quorum is a subset `Q ⊆ U`
with `sum_{p ∈ Q} w_p >= T` for some threshold `T`. By the
same inclusion-exclusion, two weighted quorums of weight `T`
intersect in weight at least `2T - W`. For Byzantine
resilience with corrupted weight `f` (fractional), require
`2T - W > f`, i.e. `T > (W + f) / 2`. With `f < W / 3`, this
gives `T > 2W / 3`, the standard `2/3` voting power threshold.

## Solution 4 [P]: 7-of-10 verification

```rust
let q = ThresholdQuorum::new(10, 7);
assert_eq!(q.intersection_lower_bound(), 4);
assert!(q.is_byzantine_resilient(2));
assert!(q.is_byzantine_resilient(3));
assert!(!q.is_byzantine_resilient(4));
```

The intersection bound is `2 * 7 - 10 = 4`, so 7-of-10 is
3-Byzantine-resilient. (The exercise text was off by one;
correct answer: 7-of-10 is 3-Byzantine-resilient, not 2-only.)

## Solution 5 [P]: random sampling

For `n = 1000, k = 100`, the expected intersection is
`k^2 / n = 10`. The variance is `k^2 (n - k) (n - k) / (n^2
(n - 1)) ≈ 10 * 0.9 = 9`, so standard deviation ≈ 3.
Probability of intersection >= 1 is essentially 1; >= 5 is
~95%; >= 10 is ~50%; >= 20 is ~0.1%.

This is the calculation behind Algorand's committee size
choice: `k = 1000` over `n = 10000` gives expected
intersection 100, easily Byzantine-resilient.

## Solution 6 [F]: pseudo-Lean QuorumSystem

```text
structure QuorumSystem (alpha : Type) (U : Finset alpha) where
  quorums : Finset (Finset alpha)
  subset_of_universe :
    forall Q : Finset alpha, Q ∈ quorums -> Q ⊆ U
  intersect_nonempty :
    forall Q1 Q2 : Finset alpha,
      Q1 ∈ quorums -> Q2 ∈ quorums ->
      (Q1 ∩ Q2).Nonempty

theorem threshold_intersection
    {alpha : Type} [DecidableEq alpha] (U : Finset alpha)
    (n thr : Nat) (h_n : U.card = n) :
    forall Q1 Q2 : Finset alpha,
      Q1 ⊆ U -> Q2 ⊆ U ->
      Q1.card = thr -> Q2.card = thr ->
      (Q1 ∩ Q2).card >= 2 * thr - n := by
  intros Q1 Q2 hQ1 hQ2 hC1 hC2
  have h_union : (Q1 ∪ Q2).card <= n :=
    le_trans (Finset.card_le_card (by ...)) (h_n ▸ Nat.le_refl _)
  have h_inter : (Q1 ∩ Q2).card =
                 Q1.card + Q2.card - (Q1 ∪ Q2).card :=
    Finset.card_inter_eq ...
  ...
```

## Solution 7 [V]: BLS QC verification circuit

For `n = 100, threshold = 67`:

- *Pairing check.* `e(agg_sig, G2) = e(H(content), pk_agg)`.
  One pairing on BLS12-381: ~`10^6` constraints.
- *Aggregate public key.* Compute `pk_agg = sum pk_i for i in
  bitmap`. `~n * 100 = 10^4` constraints.
- *Bitmap popcount.* `~n` constraints.
- *Threshold check.* Compare popcount to `67`: `~10`
  constraints.

Total: `~1.011 * 10^6` constraints per QC verification.

In Mina's Pickles using Schnorr over Pasta, the same logic is
~`67 * 3k = 200k` constraints (no pairings needed because
Schnorr-over-Pasta does not require them). The trade-off:
non-aggregated Schnorr is cheaper *per signature* than BLS but
requires `threshold` signatures rather than one aggregate.

## Solution 8 [V]: stake-weighted QC

Replace the popcount check with a weighted-sum check:
`sum_{i in bitmap} stake[i] >= weight_threshold`. The bitmap
representation is unchanged; the constraint cost adds `~n * 32
= 3200` constraints for the multi-precision sum. Total: ~`10^6
+ 3200 = 1.003 * 10^6` constraints.

Stake weights are typically encoded as 64-bit field elements;
the sum is straightforward. Production: Ethereum's Casper FFG
uses BLS-aggregated QCs with stake-weighted thresholds; the
beacon chain's slashing logic verifies stake-weight totals on
chain.
