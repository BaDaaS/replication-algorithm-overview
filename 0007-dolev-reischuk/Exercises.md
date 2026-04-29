# Module 0007 Exercises

## Exercise 1 [T]: pigeonhole proof

Reproduce the pigeonhole step of Dolev-Reischuk: assume the
honest-to-honest message graph has fewer than `(n - f) * f / 2`
edges, deduce that some honest process has fewer than `f / 2`
incident messages, and complete the indistinguishability
argument that produces a wrong decision in some schedule.

## Exercise 2 [T]: PBFT matches the bound

Show that PBFT's `O(n^2)` per-view message count, with `n = 3f
+ 1`, equals `O(n * f)` and matches the Dolev-Reischuk bound up
to constants.

## Exercise 3 [T]: amortisation in HotStuff

HotStuff's chained variant uses `O(n)` messages per QC.
Discuss why this is *not* a violation of Dolev-Reischuk:
identify the per-instance unit and explain the amortisation.

## Exercise 4 [P]: empirical validation

Use `CountingAdversary` to instrument the leader-broadcast SMR
from module 0000 and the OM(1) protocol from module 0004.
Verify experimentally that each is `O(n^2)` per agreement
instance.

## Exercise 5 [P]: bandwidth vs message count

Aggregated signatures shrink each message. With BLS aggregation,
each message is constant-size (96 bytes for BLS12-381 G1).
Measure the bandwidth of an `O(n^2)` PBFT-style exchange at
`n = 100`, `f = 33`, and compare to a non-aggregated chain
exchange at the same `n, f`.

## Exercise 6 [F]: pigeonhole in pseudo-Lean

Write the pigeonhole lemma from the README as a pseudo-Lean
statement. Identify the Mathlib lemma you would reuse
(`Finset.card_lt_iff_exists_card_lt` or similar).

## Exercise 7 [V]: prover cost from message count

Estimate the prover cost (in constraints) of a SNARK proof of
PBFT execution at `n = 100, f = 33`, with and without BLS
aggregation. Identify which terms scale with `n * f` and which
are constant.

## Exercise 8 [V]: zk-rollup sequencer message count

zk-rollup sequencers (Aztec, Scroll, Linea) run a small
internal BFT to order transactions. Estimate the per-batch
message count and the per-batch SNARK constraint cost. How
does the Dolev-Reischuk bound interact with the L1 verifier's
gas budget?
