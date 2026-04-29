# Module 0033 Exercises

## Exercise 1 [T]: leader election safety

Prove the Election Safety property: at most one leader per
term. Identify the role of `voted_for` in stable storage.

## Exercise 2 [T]: log matching

Prove the Log Matching property: if two logs have an entry
with the same index and term, the logs agree up to that
index.

## Exercise 3 [P]: implement leader election

Extend the crate to include leader election:
RequestVote/Vote messages, follower timeout, candidate
state. Verify that a leader is elected after the initial
leader crashes.

## Exercise 4 [P]: joint consensus

Implement the joint-consensus reconfiguration: the protocol
commits in `C_old + C_new` for the transition phase, then in
`C_new`.

## Exercise 5 [F]: pseudo-Lean Raft state machine

Define `RaftState` and the five safety invariants in
pseudo-Lean. Reference Verdi's Coq formalisation.

## Exercise 6 [V]: verifiable Raft

Sketch a SNARK encoding of one Raft commit: term-witness +
BLS quorum cert + log-Merkle proof.
