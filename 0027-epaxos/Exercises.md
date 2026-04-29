# Module 0027 Exercises

## Exercise 1 [T]: dependency-graph acyclicity

Prove that EPaxos's dependency graph is acyclic. Identify the
role of the leader's "earliest-among-conflicts" rule.

## Exercise 2 [P]: geo-distributed simulation

Configure an EPaxos simulator with 5 replicas in 5 regions
(US-East, US-West, EU, Asia, Australia). Measure typical
commit latency for a workload with 10% inter-region
conflicts.

## Exercise 3 [F]: pseudo-Lean dependency graph

Define a dependency-graph type in Lean. Prove the topological-
sort lemma: any topological sort of a DAG of commuting commands
gives a valid execution.

## Exercise 4 [V]: verifiable EPaxos at scale

Estimate prover cost for a verifiable EPaxos with 10000
commands per epoch and average 5 dependencies per command.
Compare to verifiable Multi-Paxos.
