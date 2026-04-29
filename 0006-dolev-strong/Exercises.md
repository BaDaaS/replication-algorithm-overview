# Module 0006 Exercises

## Exercise 1 [T]: full Dolev-Strong adversary strategy

The README sketches the adversary-strategy proof. Carry it out
in detail for `n = 4, f = 1`: construct two indistinguishable
schedules of length `r = 1` in which any deterministic
protocol must produce different outputs.

## Exercise 2 [T]: tightness

Prove that `SM(f)` from module 0004 runs in exactly `f + 1`
rounds, hence matches the Dolev-Strong lower bound.

## Exercise 3 [T]: crash-only variant

Adapt the Dolev-Strong argument to a synchronous *crash-stop*
model with `f` crashes. Show that the bound is `f + 1` here too;
identify whether authentication helps (it does not).

## Exercise 4 [P]: increase `rounds` and observe behaviour

Run the simulator with `rounds = 2, 3, 4` under the same
Byzantine equivocation pattern. Verify that all honest
followers always agree, and that increasing rounds does not
change the decision (it just gives more redundancy).

## Exercise 5 [P]: signed-chain economy

Modify the simulator's `Msg::Signed` to attach a *commitment* to
the chain rather than its full list (a Merkle path or a Poseidon
hash of the path). Discuss the bandwidth saved and the verifier
work required.

## Exercise 6 [F]: pseudo-Lean lower-bound theorem

Write the Dolev-Strong lower-bound theorem in pseudo-Lean as in
the README. Identify the cslib / Mathlib infrastructure for the
adversary-strategy game (cslib's `LTS` is insufficient on its
own; you'd need a `Game` typeclass for adversary-protocol
interaction).

## Exercise 7 [V]: succinct chain proofs

The Dolev-Strong upper-bound protocol's chain of `f + 1`
signatures admits a succinct proof: a single recursive SNARK
that checks each signature step. Sketch the recursion structure
(Pickles-style) and estimate the per-step constraint cost
assuming Schnorr-over-Pasta.

## Exercise 8 [V]: BLS aggregation flattens rounds

Show that BLS-aggregated quorum certificates collapse the
`f + 1` rounds of Dolev-Strong into a single `O(1)` aggregate
verification. Explain why this is a *circuit-level* improvement,
not a protocol-level round-complexity improvement: the
underlying broadcast still needs `f + 1` rounds of message
exchange; only the *verification* of the resulting attestation
becomes succinct.
