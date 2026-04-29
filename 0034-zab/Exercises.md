# Module 0034 Exercises

## Exercise 1 [T]: primary-order proof

Prove that, across view changes, ZAB preserves the primary's
order. Identify the role of the synchronisation phase.

## Exercise 2 [P]: discover phase

Extend the crate with a Discovery phase: on leader change,
the new leader collects each follower's last zxid and picks
the longest-prefix follower's log to follow.

## Exercise 3 [F]: pseudo-Lean ZAB phases

Define `ZabPhase` (Discovery | Sync | Broadcast | Recovery)
and the per-phase state in pseudo-Lean.

## Exercise 4 [V]: zxid-as-public-input

In a verifiable ZAB, each commit's zxid is public input. The
verifier checks zxid monotonicity and quorum cert. Estimate
constraint cost.
