# Module 0015 Exercises

## Exercise 1 [T]: Bracha 1987 termination bound

Reproduce the proof that, with a fair common coin, expected
rounds to termination is `O(1)`. Identify the constants
(Bracha 1987 gives `<= 4`).

## Exercise 2 [T]: RB layer is necessary

Show that without RB (i.e. with point-to-point messaging),
Bracha-style ABA fails: equivocation by Byzantine processes
breaks the `S_r` agreement property.

## Exercise 3 [T]: comparison with Rabin

Compare Bracha 1987 with Rabin 1983 (module 0014). Identify
where the protocols agree and where they diverge (the auxiliary
set `S_r` in Bracha vs the direct Echo majority in Rabin).

## Exercise 4 [P]: implement RB at the wire layer

Replace the simulator's all-to-all primitive in
`BrachaAbaNode` with explicit Bracha RB exchange (using the RB
layer from module 0009). Compare the message counts.

## Exercise 5 [P]: parallel ABAs (HoneyBadger structure)

Run `n` Bracha ABA instances in parallel, each with a
different proposer. Discuss the resulting throughput and
latency profile. Sketch how HoneyBadger BFT composes this with
Bracha RB.

## Exercise 6 [F]: pseudo-Lean ABA reduction

State the reduction "ABA from RB + common coin" as a Lean
theorem: given an RB primitive and a common coin, construct an
ABA satisfying validity, agreement, and termination probability
1.

## Exercise 7 [V]: zk Bracha-ABA per round

Estimate the constraint count of one Bracha-ABA round in a
SNARK circuit with `n = 100, f = 33`. Decompose the cost into
RB-Vote, RB-Aux, common coin, and decision.

## Exercise 8 [V]: aggregating multiple ABA instances

In HoneyBadger BFT, `n` parallel ABAs run per round. Sketch
how aggregation (recursive SNARKs across the parallel
instances) reduces the proof size from `O(n)` to `O(1)` per
HoneyBadger round.
