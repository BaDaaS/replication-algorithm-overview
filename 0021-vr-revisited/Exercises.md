# Module 0021 Exercises

## Exercise 1 [T]: reconfiguration safety

Prove that the SMR-mediated reconfiguration of VR Revisited
preserves linearisability across the replica-set transition.

## Exercise 2 [P]: client dedup table

Implement the `client_table` of VR Revisited. The table maps
each client's id to the latest seen request id and result.
Discuss what guarantees this provides (at-most-once execution
of client requests).

## Exercise 3 [F]: pseudo-Lean state diagram

Encode the four-state diagram (Normal, ViewChange,
Recovering, Reconfiguring) as a Lean inductive type with
transitions.

## Exercise 4 [V]: verifiable reconfiguration

Sketch a SNARK encoding of VR Revisited's reconfiguration
operation: the new set must be authorised by a quorum of the
old set. Estimate constraints.
