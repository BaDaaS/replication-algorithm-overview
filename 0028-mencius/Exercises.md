# Module 0028 Exercises

## Exercise 1 [T]: skip-slot safety

Prove that the Mencius skip mechanism preserves SMR safety:
when a slot's owner is suspected, the takeover protocol
either re-proposes the owner's value (if it had been
accepted somewhere) or proposes NO-OP.

## Exercise 2 [P]: load balancing

Modify the simulator to have unbalanced workloads (NodeId(0)
gets 80% of requests, others 20% each). Measure throughput
and discuss whether Mencius is wasting capacity.

## Exercise 3 [F]: per-slot Synod composition

State the Mencius safety theorem as the per-slot Synod
composition. Identify how the per-slot owner replaces the
single Multi-Paxos leader.

## Exercise 4 [V]: parallel proof generation

In a verifiable Mencius, each replica produces proofs for its
own slots in parallel. Discuss how the L1 verifier handles
out-of-order slot proofs.
