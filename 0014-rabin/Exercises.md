# Module 0014 Exercises

## Exercise 1 [T]: Rabin termination probability

Show that, with a fair common coin, expected rounds to
termination is `<= 4`. Identify the case analysis on the
coin's bit and the protocol's preference.

## Exercise 2 [T]: validity in the Byzantine setting

Prove that, for `f < n / 3` Byzantine, if all honest inputs
equal `v`, every honest process decides `v` in round 1.

## Exercise 3 [T]: agreement under quorum intersection

Use `n = 3f + 1` and `2/3`-thresholds to show that two distinct
Echo majorities cannot exist. Identify the role of the
`f + 1`-intersection bound from module 0010.

## Exercise 4 [P]: real common coin

Replace `CoinFn = fn(u32) -> bool` with a threshold-BLS-style
oracle that aggregates partial signatures from a quorum.
Discuss the changes to the protocol's flow (extra phase to
collect partial sigs).

## Exercise 5 [P]: DKG simulation

Sketch a Pedersen DKG protocol over the simulator. Identify
the phases (commitment, share distribution, complaint,
disqualification) and the messages exchanged.

## Exercise 6 [F]: pseudo-Lean common coin

Define the `CommonCoin` typeclass with the *honest agreement*
and *unbiased* axioms. Discuss the relationship between this
typeclass and Mathlib's `Probability.Independence`.

## Exercise 7 [V]: zk-HoneyBadger circuit

Sketch a circuit that verifies one round of HoneyBadger BFT
(Rabin-style ABA + asynchronous reliable broadcast for
proposals). Estimate the constraint count for `n = 100`.

## Exercise 8 [V]: drand-as-coin in production

Read the drand documentation and discuss how its production
threshold-BLS beacon could be reused as a common coin for an
async BFT protocol. What are the latency and trust implications?
