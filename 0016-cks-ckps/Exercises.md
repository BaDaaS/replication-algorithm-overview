# Module 0016 Exercises

## Exercise 1 [T]: CKS termination

Reproduce the CKS 2000 termination proof: expected rounds
`<= 4`. Identify the role of the threshold-coin's
unbiasability.

## Exercise 2 [T]: random oracle dependence

Discuss why CKS 2000's proof uses the random oracle model.
What property of the threshold-RSA scheme requires it?

## Exercise 3 [T]: from CKS to HoneyBadger

Outline the structural transition from CKS 2000 to HoneyBadger
BFT. Identify what HoneyBadger adds (parallelism, threshold
encryption, AVID) and what it inherits unchanged.

## Exercise 4 [P]: replace threshold-RSA with threshold-BLS

Production CKS-style protocols use threshold-BLS for the coin.
Sketch the protocol changes (key generation, partial signing,
combine operation).

## Exercise 5 [P]: instrument the simulator with a real coin

Use `MockThresholdAccumulator` (module 0011) to implement the
CKS coin: each Vote message carries a partial signature on
the round number; the accumulator combines once `t + 1`
arrive. Verify termination remains in expected `O(1)` rounds.

## Exercise 6 [F]: pseudo-Lean RO-model proof

The RO-model proof requires an oracle abstraction. Sketch how
to model it in Lean: a global `RandomOracle : Hash -> Output`
function that the protocol queries. Discuss the relationship
to `Probability.IndepFun` and to the `MeasureTheory`
infrastructure.

## Exercise 7 [V]: zk-CKS circuit

Estimate the constraint count for one CKS round in a SNARK
circuit with `n = 100, f = 33`. Compare to the Bracha
estimate from module 0015.

## Exercise 8 [V]: aggregating coin shares

In CKS, `n - f` partial coin shares arrive per round; only
`t + 1` are needed to combine. Discuss the SNARK encoding of
the "any `t + 1` of `n - f`" combiner and its constraint cost.
