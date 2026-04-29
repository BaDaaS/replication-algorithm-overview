# Module 0020 Exercises

## Exercise 1 [T]: VR safety invariant

State the VR safety invariant: at every view boundary, the new
primary's log contains every previously-committed op. Prove it
using `f + 1` quorum intersection in `n = 2f + 1`.

## Exercise 2 [T]: VR-Paxos equivalence

Make the VR-to-Multi-Paxos translation precise. Map each VR
state and message to its Paxos counterpart.

## Exercise 3 [P]: implement view change

Extend the crate's VR with view-change. Implement
`START_VIEW_CHANGE`, `DO_VIEW_CHANGE`, `START_VIEW`, and
verify that under primary failure, a new primary recovers and
resumes operation.

## Exercise 4 [P]: stable storage

VR's correctness under crash-recovery requires stable storage.
Identify which fields must be persisted before each protocol
step (the *write-ahead log* analogue).

## Exercise 5 [F]: pseudo-Lean VR

Define `VrState` and the safety theorem. Identify cslib's
`LTS` reuse: the per-view normal-case operation is one LTS
phase; view-change is another.

## Exercise 6 [V]: verifiable VR

Sketch a SNARK encoding of one VR commit. Public input:
view number, op_num, op, BLS-aggregated quorum cert.
Verifier: verify QC, check consistency. Estimate constraints.

## Exercise 7 [V]: view-change cost in circuit

A view-change involves `f + 1` DO_VIEW_CHANGE messages, each
with a per-replica log. Encoding this in circuit is more
expensive than normal-case operation. Estimate the cost.
