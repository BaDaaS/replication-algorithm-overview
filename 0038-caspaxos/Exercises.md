# Module 0038 Exercises

## Exercise 1 [T]: linearisability

Prove that CASPaxos's per-key linearisability follows from
per-key Synod safety plus the f-application step.

## Exercise 2 [P]: leader pinning per key

Discuss leader pinning per key: a designated proposer per
key amortises Phase 1 across many CAS operations. Cost?

## Exercise 3 [F]: pseudo-Lean CASPaxos

Define the per-key CASPaxos state in Lean. State the
linearisability theorem.

## Exercise 4 [V]: verifiable CAS

Sketch a SNARK encoding of one CAS operation: BLS Phase 1
quorum + f-witness + BLS Phase 2 quorum. Estimate cost.
