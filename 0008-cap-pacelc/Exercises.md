# Module 0008 Exercises

## Exercise 1 [T]: Gilbert-Lynch proof in full

The README's CAP impossibility proof is a sketch. Write it in
full, as Gilbert-Lynch 2002 do, including the formal
linearisability definition and the explicit argument that no
schedule satisfies all three properties.

## Exercise 2 [T]: which axiom can we relax?

For each of the three CAP properties, identify a concrete
weakening that a real system uses and the consequence:

- *Linearisable -> sequential consistency.*
- *Sequential consistency -> eventual consistency.*
- *Available -> available with bounded staleness.*

For each, identify the production system that takes that
choice.

## Exercise 3 [P]: implement bounded staleness

Modify the AP register to refuse reads if the local replica's
last write is more than `staleness_bound` ticks old. Test that
this gives "read your own writes" but no fresh-read guarantee.

## Exercise 4 [P]: where does Spanner sit?

Spanner uses TrueTime to bound clock uncertainty. Read the
Spanner paper (Corbett et al. 2012) Section 4 and explain how
TrueTime gives Spanner the right to claim "PC" under partition
without sacrificing typical low latency.

## Exercise 5 [F]: linearisability in pseudo-Lean

Write a `Linearisable` predicate over a history of operations.
Identify the Mathlib infrastructure (`List.Permutations`,
`Equivalence`, `Order.Embedding`).

## Exercise 6 [V]: verifiable AP-or-CP?

A verifiable replication algorithm that publishes only safety
proofs (state-transition consistency) is *agnostic* about
availability: if the chain stops growing, no proofs are
published. Discuss whether this is "AP" or "CP" in the CAP
sense, and give the relationship between the SNARK's role and
the protocol's availability under partition.
