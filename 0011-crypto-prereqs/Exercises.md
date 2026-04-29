# Module 0011 Exercises

## Exercise 1 [T]: EUF-CMA reduction

State the EUF-CMA security game for a generic signature scheme.
Reduce the security of Schnorr to the discrete-log assumption
(forking-lemma argument).

## Exercise 2 [T]: BLS aggregation soundness

Show that the same-message BLS aggregation rule is secure
under the co-CDH assumption. Identify the role of
proof-of-possession in preventing rogue-key attacks.

## Exercise 3 [T]: VRF uniqueness

The VRF security game requires that each `(pk, x)` has exactly
one accepted output. Discuss why this is non-trivial: it is not
a property of pseudo-random functions in general, but is
specific to the *verifiability* requirement.

## Exercise 4 [P]: replace `MockSig` with Ed25519

Discuss the design changes needed to replace the
`MockSig`/`mock_sign` family with Ed25519 (using e.g.
`ed25519-dalek`). Identify which tests would still work
unchanged and which would need adjustment.

## Exercise 5 [P]: BLS aggregation in Rust

Compare the Rust libraries `blst`, `bls12_381`, and `arkworks`
along the axes (i) constant-time correctness, (ii) APIs for
threshold combining, (iii) suitability for SNARK circuits.

## Exercise 6 [F]: pseudo-Lean signature scheme

Write the `SignatureScheme` typeclass with EUF-CMA as a
hypothesis. Identify the Mathlib infrastructure
(`Mathlib.Probability.UniformOn` for randomness,
`Mathlib.Topology.Algebra.InfiniteSum` for negligibility).

## Exercise 7 [V]: in-circuit threshold-BLS combine

Sketch a circuit that takes `t + 1` BLS partial signatures and
verifies that the Lagrange-combined aggregate is a valid full
BLS signature. Estimate the constraint count for `t + 1 = 67`
in BLS12-381.

## Exercise 8 [V]: SNARK-friendly hash benchmarks

Compare the constraint counts of Poseidon, Rescue-Prime, Anemoi,
and Griffin for SHA-256-equivalent inputs. Cite the original
papers' published numbers.
