# Module 0005 Exercises

## Exercise 1 [T]: write out Lemma 1 in full

The README's proof of Lemma 1 (initial bivalence) is given in
sketch form. Write the proof in full, justifying:

- Why the Hamming-distance walk between `C(0,...,0)` and
  `C(1,...,1)` always passes through a non-0-valent
  configuration.
- Why the case "this `C_k` is 1-valent and `C_{k-1}` is 0-valent"
  yields a contradiction with one crash failure.

## Exercise 2 [T]: complete Lemma 2's case analysis

The README handles cases (a), (b), (c) of the distinguishing-step
analysis. For (b), expand the proof: explain why crashing `p`
*after* the distinguishing step (rather than immediately)
preserves the bivalence argument, and identify the precise role
of the "at most one crash" hypothesis.

## Exercise 3 [T]: FLP fails with synchrony

State and prove that the FLP construction breaks if the network
is synchronous (DLS Variant 0). Specifically: identify the step
in Lemma 2 that requires asynchrony, and show that the
synchronous round-by-round model invalidates it.

## Exercise 4 [P]: extend the simulation to 3 processes

Modify the crate to support 3 processes with the same vote-
exchange protocol. Construct an adversary schedule that keeps the
system bivalent for an arbitrary number of steps, deciding on the
fly which message to withhold so as to preserve bivalence (this
is the constructive content of Lemma 2).

## Exercise 5 [P]: estimate the failure probability of a
bounded-step protocol

For a deterministic asynchronous consensus protocol that decides
within `k` steps in the typical case, give a hand-wavy bound on
the probability that an adversarial scheduler can keep the system
bivalent past step `k`. Discuss why this bound is tight.

## Exercise 6 [F]: pseudo-Lean Configuration

Define `Configuration n` as a structure containing per-process
states and a multiset of in-flight envelopes. State `Reachable`
as the transitive closure of the single-step relation. Identify
the cslib / Mathlib infrastructure (`LTS`, `Multiset`,
`Relation.ReflTransGen`) that you would reuse.

## Exercise 7 [F]: phrase FLP as an LTL formula

Express the FLP impossibility as: "there exists an asynchronous
schedule under which `G F not decided` holds for some honest
process". Discuss the relationship between this formula and the
co-inductive construction of an infinite bivalent execution. Is
the LTL formulation easier or harder to formalise than the
configuration-graph one?

## Exercise 8 [V]: how randomised consensus restores
verifiability

Describe how Ben-Or 1983 escapes FLP via randomisation. Sketch
the additional public input a SNARK-based verifiable variant
would need (a randomness-beacon witness) and identify the modern
production system that uses this pattern (drand for HoneyBadger
BFT-style protocols).
