# Module 0035 Exercises

## Exercise 1 [T]: linearisability

Prove that Chain Replication is linearisable: every read at
the tail returns the latest write that has been
acknowledged.

## Exercise 2 [P]: failure recovery

Implement head failure recovery: the configuration master
removes the failed head and promotes the next replica.

## Exercise 3 [F]: pseudo-Lean Chain Replication

Define the chain state and FIFO transition. State the
linearisability theorem.

## Exercise 4 [V]: signature chain

Sketch a verifiable Chain Replication where each replica
signs the update before forwarding. The proof is the chain of
signatures terminating at the tail.
