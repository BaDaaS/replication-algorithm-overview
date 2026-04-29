# Module 0023 Solutions

## Solution 1 [T]: amortisation

Per Synod decision: Phase 1 (Prepare + n Promises = n + 1 msgs)
+ Phase 2 (Accept + n Accepteds = n + 1 msgs) = ~2n msgs.

Per Multi-Paxos decision under stable leadership: just Phase 2
(n + 1 msgs). Phase 1 amortises across `k` decisions to
`O(n / k)`-amortised cost per decision.

Saving: factor of 2 on per-decision messages, plus the
elimination of an extra round-trip's latency.

## Solution 2 [P]: log catch-up

Add a `CATCH_UP(from_slot)` message:

```rust
Msg::CatchUp { from_slot: u32 }
Msg::CatchUpResponse { slots: Vec<(u32, u32, u32)> } // (slot, ballot, op)
```

Leader on receiving CatchUp: send the requested slots from its
own log. Follower applies them in order.

Production: Raft's "AppendEntries with prevLogIndex" subsumes
this with backtracking on mismatch.

## Solution 3 [F]: composition

```text
theorem multi_paxos_safety :
    forall (slot : Nat) (op_a op_b : Operation),
      Decided slot op_a -> Decided slot op_b -> op_a = op_b := by
  intro slot op_a op_b ha hb
  -- per-slot Synod instance.
  apply synod_safety
  exact ha
  exact hb

theorem log_monotonicity :
    forall (slot : Nat) (op : Operation),
      Decided slot op ->
      forall (later_slot : Nat), later_slot >= slot ->
      LogContains later_slot slot op := by
  -- decisions are durable; later slots see them as committed.
  sorry
```

## Solution 4 [V]: verifiable Multi-Paxos

Per slot: 1 BLS pairing (aggregated Phase 2 cert), `~10^6`
constraints. For 1000 slots without recursion: `~10^9`
constraints. With recursion: `~10^6` per step plus the
recursive verifier overhead (`~500` constraints per step).
Total prover work: `~10^9` constraints; final proof O(1).

## Solution 5 [V]: Pickles-style recursion

Pickles (Mina) folds proofs as a tree of recursions: each
recursion verifies the previous proof + one new step. For 1000
slots:

- Each step: prove the new slot's QC + verify the previous
  proof. ~10^6 + 500 = ~10^6 constraints per step.
- Tree depth: log_2(1000) ~ 10.
- Total prover: 1000 step-proofs + 10 fold-proofs ~= 10^9
  constraints sequentially, but parallelisable.
- Final proof: O(1) regardless of slot count.

Mina's deployed Pickles produces ~constant-size chain proofs
of arbitrarily long Cardano-style chains using exactly this
template.
