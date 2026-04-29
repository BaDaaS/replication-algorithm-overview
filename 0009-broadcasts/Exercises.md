# Module 0009 Exercises

## Exercise 1 [T]: Bracha RB safety in detail

Reproduce Bracha's safety proof: any two distinct contents
`m, m'` from the same sender cannot both be delivered by an
honest process. Use the `2f + 1 >= n - f + 1` quorum
intersection.

## Exercise 2 [T]: AB-consensus equivalence

Prove the Hadzilacos-Toueg equivalence: a primitive solving AB
gives a primitive solving binary consensus, and vice versa.

## Exercise 3 [T]: causal broadcast from RB

Construct CB from RB plus per-process FIFO order plus
vector-clock buffering. Identify the role of vector clocks in
ensuring the causal predicate.

## Exercise 4 [P]: Bracha under omission

Run the simulator with an `OmissionAdversary` (module 0002)
that drops 30% of messages from `NodeId(0)`. Verify that
delivery still succeeds at all honest nodes (Bracha tolerates
arbitrary omission as long as enough echoes survive).

## Exercise 5 [P]: replace echo set with threshold signature

Modify `BrachaNode` so the `n - f` echo collection produces a
threshold signature instead of explicit echo messages. Discuss
the bandwidth and SNARK-circuit consequences.

## Exercise 6 [F]: ReliableBroadcast typeclass in pseudo-Lean

Write the `ReliableBroadcast` and `AtomicBroadcast` typeclasses
in pseudo-Lean as in the README. Identify cslib's
`InferenceSystem` reuse points.

## Exercise 7 [V]: TOB as a SNARK-friendly object

A SNARK proof of TOB delivery is a proof of consensus on the
delivered sequence. Sketch the public input layout for a Mina-
style chain proof: commitment to the prefix, signature aggregate
of `2f + 1` validators, and slot/timestamp witness.
