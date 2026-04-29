# Module 0029 Exercises

## Exercise 1 [T]: composition safety

Prove that Vertical Paxos's two-SMR composition is safe:
master safety + per-config data safety => global SMR safety.

## Exercise 2 [P]: install protocol

Sketch the install-new-config protocol: how do new replicas
catch up with state from the old configuration?

## Exercise 3 [F]: pseudo-Lean Configuration

Define `Configuration` with epoch, replicas, start_slot in
pseudo-Lean. State the master-SMR safety theorem.

## Exercise 4 [V]: cross-config proof

A verifiable Vertical Paxos chain crosses configuration
boundaries. Sketch how a chain proof attests to the
configuration valid at each slot.
