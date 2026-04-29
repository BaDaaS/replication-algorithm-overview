# Module 0013 Exercises

## Exercise 1 [T]: agreement proof

Carry out Ben-Or's agreement proof in detail: if some honest
process decides `v` in round `r`, no honest process decides
`1 - v` in any round.

## Exercise 2 [T]: validity

Prove that, if all honest inputs equal `v`, every honest process
decides `v` in round 1.

## Exercise 3 [T]: termination probability

The README cites expected `O(2^n)` rounds. Reproduce the
argument: in each round, the probability that all honest coins
agree on the same value is `>= 2^{-(n - f)}`. Hence expected
rounds-to-termination is `<= 2^{n - f}`.

## Exercise 4 [P]: property test for termination

Use `proptest` to generate random `(n, f, inputs, schedule)`
tuples and verify termination occurs within `O(n^2)` simulated
ticks for `n = 4, f = 1`. Discuss the relationship between the
property test's success rate and the protocol's
termination probability.

## Exercise 5 [P]: replace local coin with common coin

Modify `BenOrNode` to use a `MockThresholdAccumulator` (from
module 0011) as a common coin. Show that termination drops
from expected `O(2^n)` to `O(1)` rounds.

## Exercise 6 [F]: pseudo-Lean termination probability

State the termination theorem: probability of termination by
round `r` is `>= 1 - 2^{-r}`. Identify the Mathlib
infrastructure (`Probability.Martingale`,
`Filter.Eventually.atTop`, `MeasureTheory.MeasurableSpace`).

## Exercise 7 [V]: verifiable Ben-Or

Sketch a circuit that verifies one round of common-coin Ben-Or
(see exercise 5). Estimate the constraint count assuming
threshold-BLS over BLS12-381.

## Exercise 8 [V]: VRF-based common coin

Replace the threshold-BLS coin with a VRF-based coin: each
process publishes a VRF output on the round number; the
*minimum* VRF output across all processes is the coin. Discuss
the trade-offs (latency, biasability, circuit cost).
