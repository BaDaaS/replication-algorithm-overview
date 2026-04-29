# Module 0024 Solutions

## Solution 1 [T]: cost savings

| `n` | full Paxos storage | Cheap Paxos main | saving |
| --- | ------------------ | ---------------- | ------ |
| 5   | 5                  | 3                | 40%    |
| 7   | 7                  | 4                | 43%    |
| 9   | 9                  | 5                | 44%    |

The saving asymptotically approaches 50% (one machine of every
two saves storage). At the cost of higher complexity and more
involved reconfiguration logic.

## Solution 2 [P]: reconfiguration trigger

```rust
fn on_timer(&mut self, _: u64, ctx: &mut StepCtx<'_, Msg>) {
    if self.is_leader && self.timed_out_acceptors().len() > 0 {
        // Run a configuration-change Synod to swap an
        // auxiliary into the active set.
        let new_config = self.derive_new_config();
        ctx.send(self.id, /* config-Synod target */, ...);
    }
}
```

The configuration-change Synod uses the *full* set of `n`
acceptors (mains + auxiliaries) to decide on the new
configuration; once decided, the protocol switches to the new
configuration's `f + 1` mains.

## Solution 3 [F]: two-level safety

```text
theorem cheap_paxos_safety
    (cfg : ConfigSMR) (data : DataSMR)
    (h_cfg : ConfigSafety cfg)  -- Synod safety on configs
    (h_data : DataSafety data) -- Synod safety on operations
    : SmrSafety := by
  intro op_a op_b ha hb
  -- Both decisions were made under some configuration.
  -- By config safety, all replicas agree on the active config.
  -- By data safety within that config, op_a = op_b.
  sorry
```

The structure: configuration safety ensures replicas agree on
"who is the current quorum"; data safety ensures decisions
within that quorum are unique.

## Solution 4 [V]: configuration as public input

The verifier's public input includes:

- The proven decision (slot, op).
- The configuration ID under which the decision was made.
- The configuration commitment (Merkle root or hash).

The verifier checks:

- The configuration commitment matches the chain's state.
- The decision's quorum certificate has signers in the
  configuration's main set.

This is the same pattern as Ethereum's validator-set proofs:
each proof carries the validator-set commitment; the L1
contract checks the commitment matches the current beacon-
chain state.
