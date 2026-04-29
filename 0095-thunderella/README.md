# 0095: Thunderella

## Historical context

Rafael Pass and Elaine Shi published "Thunderella: Blockchains
with Optimistic Instant Confirmation" at Eurocrypt 2018.
Thunderella sharpens the Hybrid Consensus framework (module
0092) for the *common case*: when a designated *accelerator*
(or super-quorum) is honest and reachable, transactions
finalise *instantly* (single network round-trip).

The protocol was named after the "thunder" optimistic path.
When the accelerator misbehaves or goes offline, the protocol
falls back to the slow chain layer (Nakamoto-style PoW or PoS)
without losing safety.

The key contribution: instant finality is available
*optimistically* (i.e., when the network is good and the
accelerator behaves), with seamless fall-back to Nakamoto-rate
finality on bad days. This pattern (optimistic fast path,
fallback slow path) is now standard in modern BFT and rollups.

## System and threat model

- **Network.** Bounded-delay (PSS).
- **Failures.** Computational honest-majority globally. The
  accelerator is honest in the *good case*; the slow path
  tolerates a Byzantine accelerator.
- **Cryptography.** SHA-256 PoW + threshold signatures.
- **Goal.** Optimistic instant finality + Nakamoto-rate
  fallback.

## Theory

### Two paths

- *Fast path.* Accelerator (a single party or small committee)
  signs each transaction. Honest accelerator + 3/4 honest
  super-quorum -> single-round-trip finality.
- *Slow path.* Chain protocol (any robust transaction ledger
  per Pass-Shi 2017a) finalises blocks at the block-interval
  rate.

### Optimistic finality theorem

If `>= 3/4` of stake (or hashing power) is honest *and* the
accelerator is honest, transactions commit in `O(delta)` (one
network round-trip).

If the accelerator is malicious or fewer than `3/4` are
honest, the protocol falls back to the slow chain rate, and
safety is preserved by the chain-layer `< 1/2` Byzantine bound.

### Why 3/4 not 2/3?

The 3/4 threshold (rather than the standard PBFT 2/3) is
needed because the accelerator can equivocate; quorum-
intersection arguments require an extra `f` to cover the
equivocator's potential confusion.

### Comparison: optimistic finality protocols

| protocol      | fast-path latency | fast threshold | fallback path | year |
| ------------- | ----------------- | -------------- | ------------- | ---- |
| Hybrid Cons.  | O(delta) BFT      | `2/3`          | chain         | 2017 |
| Thunderella   | O(delta) BFT      | `3/4`          | chain         | 2018 |
| HotStuff      | O(delta) BFT      | `2/3`          | view change   | 2018 |
| Sync HotStuff | O(2 * delta)      | `1/2`          | timeout       | 2020 |
| BBCA-chain    | optimistic FBA    | `2/3`          | DAG           | 2024 |

Thunderella uses a higher fast-path threshold (3/4) but
benefits from a single-round-trip latency that PBFT-style
protocols cannot match.

### Properties

- *Optimistic instant finality.* Single round-trip in good case.
- *Nakamoto fallback.* Always available.
- *Permissionless.* Anyone can join the slow-chain layer.
- *Modular.* Works with any chain protocol satisfying GKL
  properties.

### Subsequent influence

- *Optimistic rollups* (Arbitrum, Optimism). Single-round-trip
  finality with fraud-proof fallback.
- *Sui Lutris.* Optimistic single-shard execution with BFT
  fallback.
- *DiemBFT (LibraBFT).* Two-phase commit pattern.
- *Modern BFT.* HotStuff-2 (module 0059) and Jolteon (0058)
  inherit the optimistic-then-fallback design.

## Practice

Thunderella has not been deployed verbatim. Its ideas appear in:

- *Optimistic rollup designs* with single-aggregator finality
  + 7-day challenge-window fallback.
- *Diem (Libra)* committee + chain reconfiguration.
- *Aptos / Sui.* PoS committee + fast/slow path.

### Production-implementation notes

- The accelerator must be highly available; SLA-grade
  redundancy (multiple physical nodes per accelerator key,
  failover) is standard practice.
- Fall-back detection: clients must distinguish "accelerator
  silent" from "accelerator down" to switch paths.
- Accelerator slashing or stake-loss is the typical disincentive
  against misbehaviour.

### Mining algorithm (proof-of-work function)

Thunderella's slow-path chain layer is generic: any
Sybil-resistant chain protocol satisfying GKL backbone
properties suffices. In a PoW instantiation:

- *Bitcoin-anchored.* Double-SHA-256.
- *Ethereum-anchored.* Ethash (Ethereum pre-Merge).
- *Custom chain.* Any cryptographic hash modelled as a random
  oracle.

A PoS instantiation replaces PoW entirely with a VRF-based
slot lottery (Praos, Snow White). The accelerator's signature
scheme is independent of the chain-layer hash; production
designs typically use BLS-aggregate threshold signatures
(BLS12-381) for the optimistic fast path.

## Verifiability and circuit encoding

**tag: `partial`.**

Thunderella circuits encode the optimistic threshold-signature
(BLS or Schnorr) plus the chain layer's PoW or PoS.
Threshold-signature verification is cheap (~100 constraints);
the chain layer dominates SNARK cost.

## Known attacks and limitations

- *Accelerator failure.* Liveness drops to chain rate.
- *Compromised accelerator.* Cannot break safety, but can stall
  the fast path until the chain layer rotates.
- *3/4 threshold.* Higher than PBFT; requires more honest
  parties.

## References

- Pass, Shi, "Thunderella: Blockchains with Optimistic Instant
  Confirmation", Eurocrypt 2018.

## Implementation notes

The crate provides a `Path` enum (`Optimistic`/`Fallback`) and
a `pick_path` function selecting `Optimistic` only when the
accelerator is online and the super-quorum (`>= 3 * n / 4`) is
honest. Tests verify the path selection.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
