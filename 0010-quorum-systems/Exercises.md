# Module 0010 Exercises

## Exercise 1 [T]: prove the threshold theorem

Use inclusion-exclusion to prove the intersection bound for
threshold quorums: with `n = 3f + 1` and quorum size `2f + 1`,
any two quorums intersect in at least `f + 1` processes.

## Exercise 2 [T]: tightness

Show that with `n = 3f, threshold = 2f`, the intersection
bound drops to `f`, which is insufficient for Byzantine
resilience.

## Exercise 3 [T]: weighted quorums

Generalise the threshold theorem to weighted processes (each
`p_i` has weight `w_i`). Identify the analogue of
`2f + 1`: total weight `> 2/3 W`.

## Exercise 4 [P]: verify the simulator helper

Use `ThresholdQuorum` to verify that a 7-of-10 quorum is
2-Byzantine-resilient but not 3-Byzantine-resilient.

## Exercise 5 [P]: probabilistic quorum sampling

Implement a function that samples a random `k`-element subset
of `n`-process universe and computes the empirical
intersection of two such samples. For `n = 1000, k = 100`,
estimate the probability that the intersection is at least
1, 5, 10, 20.

## Exercise 6 [F]: pseudo-Lean QuorumSystem

Write `QuorumSystem` as a Lean structure (see README) and
prove the inclusion-exclusion lemma using
`Mathlib.Finset.card_inter`.

## Exercise 7 [V]: BLS quorum certificate verification

Sketch a circuit that takes a BLS-aggregated QC `(content,
agg_sig, signers_bitmap)` and accepts iff popcount
`>= threshold` and the pairing check passes. Estimate the
constraint count for `n = 100, threshold = 67`.

## Exercise 8 [V]: stake-weighted QC in circuit

Modify the QC verifier to support per-validator weights
(stake). The signer_bitmap becomes a stake-weight encoding;
the threshold becomes a total-weight check. Discuss the
constraint cost compared to flat-bitmap QCs.
