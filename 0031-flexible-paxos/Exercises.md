# Module 0031 Exercises

## Exercise 1 [T]: optimal Q1/Q2 trade-off

For `n = 7`, find Q1 and Q2 sizes minimising the per-commit
cost subject to the intersection property.

## Exercise 2 [P]: grid quorums

Implement grid quorums in the FlexibleQuorums helper:
acceptors arranged in a `sqrt(n) x sqrt(n)` grid, Q1 = any
column, Q2 = any row.

## Exercise 3 [F]: pseudo-Lean intersection

State the FlexibleQuorum class in pseudo-Lean. Prove that the
intersection property implies Synod safety.

## Exercise 4 [V]: workload-tuned verifiable Paxos

For a workload with 99% commits and 1% leader changes, choose
the optimal Q1/Q2 to minimise total proving cost.
