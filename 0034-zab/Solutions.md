# Module 0034 Solutions

## Solution 1 [T]: primary order

Suppose leader `L_1` of epoch `e_1` proposed `op_1` at zxid
`z_1`, then `op_2` at zxid `z_2`, with `z_1 < z_2`. By
construction, `z_1.epoch = z_2.epoch = e_1` and
`z_1.counter < z_2.counter`.

A new leader `L_2` of epoch `e_2 > e_1` runs Synchronisation:
collects all followers' logs and picks the longest. By
quorum intersection, the longest log contains both `op_1`
and `op_2` (any committed op was on `f + 1` replicas; any
two such sets intersect). The longest log preserves the order
`(z_1, op_1) < (z_2, op_2)`.

`L_2`'s log thus has `op_1` before `op_2`. All subsequent
commits preserve this. QED.

## Solution 2 [P]: Discovery sketch

```rust
fn become_leader(&mut self, new_epoch: u32, ctx: ...) {
    self.epoch = new_epoch;
    for peer in &self.everyone {
        ctx.send(self.id, *peer, Msg::DiscoverRequest { epoch: new_epoch });
    }
}

fn on_discover_response(&mut self, env, ctx) {
    self.discoveries.insert(env.from, env.last_zxid, env.log_summary);
    if self.discoveries.len() >= self.quorum() {
        let longest = pick_longest_log(self.discoveries);
        self.log = longest;
        // Now broadcast SYNC to all.
    }
}
```

The Discovery phase emits `f + 1` messages and processes the
responses. The new leader's log is the longest seen.

## Solution 3 [F]: pseudo-Lean phases

```text
inductive ZabPhase where
  | discovery
  | sync
  | broadcast
  | recovery

structure ZabState where
  phase     : ZabPhase
  epoch     : Nat
  log       : List (Zxid × Operation)
  voted_for : Option NodeId
```

The state machine transitions: `Recovery -> Discovery -> Sync
-> Broadcast`. The full transition system is the LTS of the
ZAB protocol; cslib's LTS framework supports this directly.

## Solution 4 [V]: zxid public input

Per-commit:

- Zxid as public input: 8 bytes (epoch + counter).
- BLS quorum cert: ~10^6 constraints (one pairing).
- Zxid monotonicity check: ~constraints.

The verifier reads the zxid, checks the cert, and asserts the
zxid is greater than the previous block's zxid. Total ~10^6
constraints per commit, BLS-dominated.

Production: Cosmos chains using CometBFT (Tendermint) attach
similar (height, round) tuples; light clients verify the
chain's zxid sequence.
