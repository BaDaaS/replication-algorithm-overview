# Module 0018 Exercises

## Exercise 1 [T]: threshold-BLS unbiasability

Show that, given a Pedersen DKG with `t + 1` honest
contributors and an unforgeable threshold-BLS scheme, the
per-round coin is uniformly distributed.

## Exercise 2 [T]: VRF withholding attack

Sketch the "minimum VRF" rule: coin = lowest bit of
`min(VRF_outputs over n - f processes)`. Show that the
adversary controlling `f` outputs can bias the coin by at
most `f` bits per round.

## Exercise 3 [T]: VDF uniqueness

State and prove the VDF uniqueness property: given input `x`
and time `T`, only one valid output `y` exists. Discuss why
this is the load-bearing property for unbiasability.

## Exercise 4 [P]: instrument an ABA with threshold-BLS

Replace the `CoinFn` constants in modules 0014-0017 with a
real threshold-BLS coin: each Vote message carries a partial
share; the accumulator combines once `t + 1` arrive. Compare
the round latency.

## Exercise 5 [P]: drand integration

Sketch the integration pattern of a CKS-style ABA with a drand
beacon: the protocol fetches the beacon, hashes it with the
round number, and uses the result as the coin.

## Exercise 6 [F]: pseudo-Lean common-coin typeclass

Define the `CommonCoin` typeclass with `unbiasability` as a
theorem (probability of any particular bit equals 1/2).
Identify the construction-specific instances.

## Exercise 7 [V]: in-circuit comparison

Tabulate the per-coin SNARK constraint counts for each
construction (threshold-BLS, threshold-Schnorr, VRF, VDF).
Pick the optimal for `n = 100, f = 33`.

## Exercise 8 [V]: post-quantum coins

Discuss the post-quantum readiness of each construction. Which
primitives have PQ analogues? Which have not yet matured?
