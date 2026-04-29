# Module 0035 Solutions

## Solution 1 [T]: linearisability

Define the linearisation point of a write at its tail-arrival
time. Reads at the tail always return the latest tail-stored
value, which is the latest committed write. Real-time order:
if write `w_1` finishes (tail acks) before write `w_2` is
issued, then `w_1`'s tail-time precedes `w_2`'s
issue-to-tail time. So `w_1` precedes `w_2` in the
linearisation, and any read after `w_1`'s tail-time sees
`w_1` or later.

QED.

## Solution 2 [P]: head failure

Configuration master detects head crash via heartbeat
timeout. New configuration: remove old head, promote
next-in-chain. Master commits the new configuration via
Paxos. Replicas adopt the new chain on next contact with
master.

Pseudo-code:

```rust
fn on_head_crash(&mut self) {
    self.master.commit_config(NewChain {
        head: self.chain[1].clone(),
        rest: self.chain[2..].to_vec(),
    });
    // Old head is gone; new head accepts new writes.
}
```

## Solution 3 [F]: pseudo-Lean

```text
structure ChainState where
  chain   : List NodeId
  store   : Map NodeId (Map Key Value)
  log     : List (NodeId × Update)

theorem chain_linearisable :
    forall (sched : AsyncSched),
    Linearisable ChainState sched := by
  -- Tail-ack point + FIFO chain order.
  sorry
```

## Solution 4 [V]: signature chain

Per-write proof: a chain of signatures, one per replica:

```
update: (k, v)
sig_1 = head_sk.sign(update)
sig_2 = R1_sk.sign(update || sig_1)
...
sig_n = tail_sk.sign(update || sig_{n-1})
```

The chain of signatures attests to the propagation through
the chain. Verifier verifies each link: ~n * 3k constraints
(Schnorr-over-Pasta).

For chain length 5: ~15k constraints per write. Smaller than
BLS-aggregate Paxos's ~10^6.

The trade-off: chain replication's proof structure is
explicitly serial; the prover work scales linearly with chain
depth, but per-link cost is small.
