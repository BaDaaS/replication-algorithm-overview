# Module 0032 Exercises

## Exercise 1 [T]: refinement proof

State and prove the refinement: every Compartmentalized
Paxos execution is a valid classic Paxos execution with
roles co-located.

## Exercise 2 [P]: role-pool sizing

For 1000 clients at 100k QPS, size the per-role pools
(proposers, acceptors, batchers, unbatchers, replicas).

## Exercise 3 [F]: pseudo-Lean roles

Define `Role` as a Lean inductive type. Define a per-role
state machine and the projection functions onto the
classic-Paxos state.

## Exercise 4 [V]: per-role verifiability

Sketch how each role's actions can be independently proved.
Discuss whether the composed proof is more or less expensive
than a monolithic Paxos proof.
