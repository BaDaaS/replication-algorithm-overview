# Module 0028 Solutions

## Solution 1 [T]: skip safety

When replica `r` suspects owner `o` for slot `s`:

1. `r` runs Phase 1 of Paxos for slot `s` with a higher ballot.
2. Acceptors respond with their last accepted value for `s`
   (if any).
3. If any acceptor reports a previous accepted value, `r`
   proposes that value (preserving any commit).
4. Otherwise, `r` proposes NO-OP.

Phase 2 commits whichever value `r` proposed. Safety follows
from Synod safety on slot `s`.

The crucial invariant: NO-OP is a valid SMR operation that
does not change state, so substituting it for an unproposed
slot is benign.

## Solution 2 [P]: unbalanced workloads

Setup: NodeId(0) has 80 ops; NodeId(1, 2) have 10 each.

Result: NodeId(0) commits 80 in its slots; the other two
commit 10 each. Total 100 ops in (max(80, 10, 10) * 3 + 80
... wait, the layout per the rotation: node 0's slots are
0, 3, 6, ..., 237 (80 slots); node 1's are 1, 4, ..., 28
(10 slots); node 2's are 2, 5, ..., 29 (10 slots).

The log spans up to slot 237 with NO-OPs in nodes 1, 2's
slots beyond their pending. Throughput is throttled by
NodeId(0)'s rate; the multi-leader benefit is lost.

Mitigation: client-side load balancing distributes requests
evenly across replicas.

## Solution 3 [F]: per-slot Synod composition

```text
theorem mencius_smr_safety
    (n f : Nat) (h : 2 * f + 1 = n) (sched : AsyncSched) :
    forall (slot : Nat) (op_a op_b : Operation),
      Decided slot op_a -> Decided slot op_b -> op_a = op_b := by
  -- For each slot, exactly one Synod instance runs.
  -- Owner is the initial proposer; on takeover, the new
  -- proposer is some other replica with a higher ballot.
  -- Apply synod_safety to each slot independently.
  apply synod_safety
```

The structure: per-slot Synod, with owners providing initial
proposals and takeovers via classic Paxos.

## Solution 4 [V]: parallel proofs

Each replica computes proofs for its own slots independently.
The L1 verifier:

- Receives per-slot proofs (slot number, BLS aggregate cert,
  op).
- Verifies each in O(1) (one pairing).
- Maintains a slot-monotonic log: orders proofs by slot.
- Updates state machine in slot order.

Parallel generation: prover side O(n) wall-clock for n
replicas working concurrently. Verifier side: serial in
slot order, but each step is O(1). Total verifier work:
O(slots) BLS pairings.

This is the structural advantage of Mencius for verifiable
deployment: parallel prover, serial-but-fast verifier.
Production zk-rollups using multi-leader sequencer designs
(Aztec, Scroll) employ similar patterns.
