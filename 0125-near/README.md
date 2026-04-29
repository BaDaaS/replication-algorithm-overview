# 0125: NEAR

## Historical context

NEAR Protocol launched mainnet in October 2020 with a unique
sharded PoS design (NEAR Foundation, founders Alexander
Skidanov and Illia Polosukhin). NEAR's consensus stack:

- *Doomslug.* Practical block-time finality (next-block
  finality on the optimistic path).
- *Nightshade.* Sharded execution: each shard has its own
  state, all shards share the same chain.

NEAR uses *stateless validation* heavily, with a focus on
sharding for horizontal scalability.

## System and threat model

- **Network.** Bounded delay PSS.
- **Failures.** Byzantine `f < n/3`.
- **Cryptography.** Ed25519, BLS aggregate signatures.
- **Goal.** Sharded PoS with low latency.

## Theory

### Doomslug finality

Doomslug provides *practical finality* in two cases:

1. *Two-block finality.* If validators see two consecutive
   blocks signed by 2/3 stake, the first is final.
2. *Doomslug finality* (worst case). If validators see a single
   block signed by 2/3 stake AND no competing block in the
   next time window, the block is final.

Combined: most blocks finalise in 2 block intervals (~2 sec).

### Nightshade sharding

NEAR's *Nightshade* (named after the chain dataset's nightly-
build pattern) divides state into shards. Each shard:

- Has its own validator subset.
- Produces its own chunks (per-shard blocks).
- Shares the same block timeline with other shards.

A *block* is the aggregation of *chunks* from all shards,
linked together. Validators rotate across shards regularly.

### Subsequent improvements: Stateless validation

Recent NEAR upgrades introduced *stateless validation*:
validators verify chunks from any shard without holding the
state, using cryptographic proofs of state transitions.

### Comparison: NEAR vs Polkadot vs Solana

| property              | NEAR          | Polkadot     | Solana          |
| --------------------- | ------------- | ------------ | --------------- |
| sharding              | yes (built-in) | parachains  | no              |
| consensus             | Doomslug + NS | BABE+GRANDPA | Tower BFT       |
| validator count       | ~100 per shard | ~300 relay   | ~1500           |
| block time            | 1 sec         | 6 sec        | 0.4 sec         |
| stateless validation  | yes (recent)  | no           | no              |
| year                  | 2020          | 2020         | 2020            |

### Properties

- *Sharded scaling* (many shards run in parallel).
- *Practical finality* via Doomslug.
- *Stateless validation* via cryptographic proofs.
- *PoS* with delegation.

### Limitations

- *Cross-shard latency.* Cross-shard transactions take more
  block intervals.
- *Validator hardware.* Storage and bandwidth scale with shard
  count.
- *Security composition.* Per-shard security is weaker than
  global; shard validators can be more easily corrupted.

### Subsequent work

- *Stateless validation.*
- *NEAR DA* (data availability) for L2 rollups.
- *Chain abstraction.*

## Practice

- *NEAR mainnet.* Production since October 2020.
- ~100 validators per shard; multiple shards live.
- Block time: 1 second.
- Throughput: ~1000-100000 tx/s aggregate (across shards).

### Production-implementation notes

- *Shards live.* 4-6 shards historically; growing.
- *Chunk producers.* Validators rotate regularly to avoid
  bias.
- *Slashing.* Active for double-sign and downtime.

## Verifiability and circuit encoding

**tag: `partial`.**

NEAR's BLS aggregate signatures are SNARK-friendly. The
sharded design and stateless validation use cryptographic
proofs that map well to circuits. NEAR DA is a SNARK-aware
data-availability layer.

## Known attacks and limitations

- *Per-shard collusion.* If `> 1/3` of one shard's validators
  collude, that shard's safety can fail.
- *Stake centralisation.* Largest validators dominate.
- *MEV.* Active.

## References

- NEAR Foundation, "NEAR White Paper", 2018.
- NEAR Foundation, "Nightshade Sharding", 2020.
- NEAR Foundation, "Stateless Validation", 2024.

## Implementation notes

The crate provides a `Shard` struct with per-shard validator
lists and a `Block` aggregating chunk roots. Tests verify
chunk-aggregation invariant.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
