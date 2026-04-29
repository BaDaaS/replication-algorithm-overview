# Module 0036 Exercises

## Exercise 1 [T]: read linearisability

Prove that CRAQ reads are linearisable. Identify the case
analysis on clean vs dirty.

## Exercise 2 [P]: throughput modelling

For a chain of n = 5 with 90% reads, 10% writes, model the
throughput improvement vs vanilla Chain Replication.

## Exercise 3 [F]: pseudo-Lean clean/dirty

Define `KeyState` with clean and dirty in pseudo-Lean. Prove
that `local_read` returns a committed value or None.

## Exercise 4 [V]: per-replica read attestations

Sketch a verifiable CRAQ where each replica produces a signed
attestation of its clean state. The verifier checks the
attestation matches the chain's committed state.
