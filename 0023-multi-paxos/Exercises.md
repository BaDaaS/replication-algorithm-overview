# Module 0023 Exercises

## Exercise 1 [T]: amortisation argument

Show that Multi-Paxos's per-decision message count is `O(n)`
under stable leadership, vs `O(n)` per Synod-decision (which
includes Phase 1). Identify the savings.

## Exercise 2 [P]: log catch-up

Extend the crate to handle a follower joining mid-execution:
the follower sends a `CATCH_UP(slot)` request; the leader
replies with the relevant log slots.

## Exercise 3 [F]: per-slot Synod safety composition

State the Multi-Paxos safety theorem as the composition of
per-slot Synod safety theorems plus log monotonicity.

## Exercise 4 [V]: verifiable Multi-Paxos

Per-decision SNARK proof: one BLS aggregate cert per slot,
chained recursively across the log. Estimate the prover cost
for `n = 100` over 1000 slots.

## Exercise 5 [V]: proof aggregation across slots

Discuss how Pickles-style recursion folds 1000 slot-commits
into one O(1) proof. Estimate the prover work and the final
proof size.
