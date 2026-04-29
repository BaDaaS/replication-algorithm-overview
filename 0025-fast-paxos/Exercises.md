# Module 0025 Exercises

## Exercise 1 [T]: quorum-intersection

For `n = 3f + 1, Q_c = 2f + 1, Q_f = 2f + 1` (a stricter Fast
Paxos variant), compute the intersection. Confirm it exceeds
`f`.

## Exercise 2 [P]: contention scenarios

Construct a scenario in which two clients propose conflicting
values simultaneously. Show that Fast Paxos falls back to the
slow path and decides one of the two via classic recovery.

## Exercise 3 [F]: state transitions

Define the Fast Paxos state machine in pseudo-Lean: the
fast-path acceptance is one transition; the slow-path
recovery is another. Prove safety by case analysis.

## Exercise 4 [V]: verifiable Fast Paxos

A Fast Paxos commit's verification depends on which path was
taken (fast vs slow). The public input must indicate the
path. Estimate constraints for each path.
