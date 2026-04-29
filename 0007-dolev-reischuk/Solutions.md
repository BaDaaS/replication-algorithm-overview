# Module 0007 Solutions

## Solution 1 [T]: pigeonhole

Suppose the honest-to-honest message graph has at most `(n - f) *
f / 2 - 1` edges. By pigeonhole, some honest process `p` has
in-degree at most `f / 2 - 1` (else the total would be at least
`(n - f) * f / 2`). Hence `p` has fewer than `f / 2` reports
about other honest processes' values.

The adversary's strategy: silence the `f` Byzantine processes
entirely, and additionally silence half of `p`'s reports
(possible since `p` has at most `f / 2 - 1` reports already).
The result is two indistinguishable schedules:

- `S_a`: input vector `a`. `p` sees its biased subset of
  reports.
- `S_b`: input vector `b` differing in some bit not visible to
  `p`. `p` sees the same biased subset.

In one of these scenarios `p` must produce an output
inconsistent with validity, contradicting `IsByzAgreement`.
Hence the message graph must have at least `(n - f) * f / 2`
edges. QED.

## Solution 2 [T]: PBFT matches

PBFT exchanges, per view: 1 `pre-prepare` (leader to all `n - 1`
followers), `n` `prepare` messages (each replica to all others),
and `n` `commit` messages (similarly). Total per view:
`O(n^2)`.

With `n = 3f + 1`, `n^2 = (3f + 1)^2 = 9f^2 + 6f + 1 = O(n * f)`
since `n = Theta(f)`. Matches Dolev-Reischuk asymptotically.

## Solution 3 [T]: HotStuff amortisation

HotStuff's chained variant pipelines four phases (`prepare`,
`pre-commit`, `commit`, `decide`) such that each new view
contributes one phase to four different proposals
simultaneously. Each phase is one round of QC formation,
costing `O(n)` messages. So `O(n)` messages per view, but each
view "completes" only the trailing proposal in the four-deep
pipeline.

Per *agreement instance* (one decision), the message count is
still `O(n)` direct + `O(n)` carried over from earlier views =
`O(n)` total. This is below Dolev-Reischuk only if `f` is
constant; for growing `f`, the worst-case path requires
`f + 1` view changes, each costing `O(n)`, totalling `O(n * f)`.
The amortisation is over *typical* views, not over the *worst-
case* recovery path.

## Solution 4 [P]: empirical validation

Code:

```rust
use replication_0000_introduction_smr::{
    Counter, CounterOp, LeaderBroadcastNode,
};
use replication_0007_dolev_reischuk::CountingAdversary;
use sim::{NoOpAdversary, NodeId, Scheduler};

fn count_smr(n: u32) -> u64 {
    let ids: Vec<NodeId> = (0..n).map(NodeId).collect();
    let mut sched = Scheduler::<LeaderBroadcastNode<Counter>>::new(0);
    for (i, &id) in ids.iter().enumerate() {
        let pending = if i == 0 {
            vec![CounterOp::Add(1)]
        } else {
            vec![]
        };
        sched.add_node(LeaderBroadcastNode::new(
            id, ids[0], ids.clone(), pending, Counter::default(),
        )).unwrap();
    }
    let mut adv = CountingAdversary::new(NoOpAdversary);
    sched.run(&mut adv, 10_000).unwrap();
    adv.count
}

assert_eq!(count_smr(4), 4); // leader broadcasts to all 4 (incl self)
assert_eq!(count_smr(8), 8);
```

The leader-broadcast SMR sends `n` messages per operation (one
to each node including the leader's loopback). For one operation,
total = `n`; for `k` operations, total = `n * k`. This is `O(n)`
per agreement instance, *below* the Dolev-Reischuk bound, because
the leader-broadcast SMR is not a Byzantine agreement protocol;
it tolerates *zero* Byzantine faults.

## Solution 5 [P]: bandwidth comparison

At `n = 100, f = 33`:

- *Non-aggregated.* Each PBFT message contains a signature of
  `~64` bytes (Ed25519) plus payload (`~256` bytes for a
  pre-prepare, `~64` bytes for a prepare/commit). Total per view:
  `O(n^2) * ~150` bytes = `~1.5 MB`.
- *Aggregated.* Each PBFT message becomes a partial QC; the
  aggregation reduces signatures to `~96` bytes total per QC
  (BLS12-381 G1). Three QCs per view = `~300` bytes for
  signatures, plus `~3 * n` bytes for the bitmap signers. Total
  per view: `O(n) * ~300` bytes = `~30 KB`.

The 50x bandwidth saving is the standard production motivation
for BLS aggregation in Cosmos, Aptos, and Ethereum.

## Solution 6 [F]: pigeonhole in pseudo-Lean

```text
lemma low_degree_witness :
    forall (V : Finset alpha) (E : Finset (alpha × alpha))
           (avgDeg : Nat),
      V.card * avgDeg <= 2 * E.card ->
      (forall v : V, V.degree E v >= avgDeg) ->
      E.card >= V.card * avgDeg / 2 := by
  -- Standard double-counting: sum of degrees = 2 |E|.
  intro V E avgDeg h_avg h_each
  have h_sum : Finset.sum V (degree E) >= V.card * avgDeg := by
    apply Finset.sum_le_sum_of_le
    exact h_each
  ...
```

Mathlib has `SimpleGraph.degree_sum` and
`Finset.sum_const`; use them to lift the double-counting.

## Solution 7 [V]: prover cost

At `n = 100, f = 33`:

- *Non-aggregated PBFT proof.* Per-view: `n^2 = 10^4` signature
  checks. At `~3k` constraints each: `~3 * 10^7` constraints
  per view. With 3 views (pre-prepare, prepare, commit) per
  decision: `~10^8` constraints per decision.
- *Aggregated PBFT proof.* Per-view: 1 BLS pairing + bitmap
  check. `~10^6 + 100` constraints. Per decision: 3 of these,
  `~3 * 10^6` constraints.

Aggregation saves about 30x on the prover side. The verifier
side is constant (one or two pairings) regardless. This is why
production zk-rollup sequencers and verifiable replication
designs all use BLS aggregation.

## Solution 8 [V]: zk-rollup sequencer

zk-rollup sequencers handle batches of `~10^4` to `~10^5`
transactions. The internal BFT is small (`n = 4` to `10`,
`f = 1` or `2` typically). Per-batch:

- BFT messages: `O(n^2)` per batch, but `n` is small. For
  `n = 5`: `~25` messages per batch.
- Transaction execution: `~10^4` ops, each at `~10^4`
  constraints (for an EVM-style VM): `~10^8` constraints per
  batch.

The BFT layer is *not* the binding cost; transaction execution
is. Dolev-Reischuk's `O(n * f)` floor is small in absolute
terms compared to the per-batch transaction cost. Aztec's
sequencer documents this explicitly: the L1 gas budget for
verifying a batch is dominated by the EVM-execution circuit,
not by the sequencer-BFT circuit.
