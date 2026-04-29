# 0120: Polkadot

## Historical context

Polkadot launched mainnet in May 2020 (Web3 Foundation, Gavin
Wood). Its consensus is a *hybrid* combining three protocols:

- *BABE* (Blind Assignment for Blockchain Extension):
  block production via VRF-based slot leader election (Praos-
  inspired).
- *GRANDPA* (GHOST-based Recursive Ancestor Deriving Prefix
  Agreement): finality gadget over the BABE chain.
- *BEEFY* (Bridge Efficiency Enabling Finality Yielder, 2022):
  fast finality proofs for cross-chain bridges.

This separation of *block production* and *finality* is similar
to Ethereum's Gasper (LMD-GHOST + Casper FFG), but the Polkadot
designs differ in detail: GRANDPA finalises *chains* not
checkpoints; BABE uses VRF leader election (not RANDAO); BEEFY
provides BLS-aggregate finality proofs for light clients.

Polkadot's *relay chain* runs BABE+GRANDPA+BEEFY; *parachains*
run their own consensus and submit blocks to the relay chain
for inclusion validation.

## System and threat model

- **Network.** Bounded delay PSS.
- **Failures.** Byzantine `< 1/3` for finality, `< 1/2` for
  block production.
- **Cryptography.** Sr25519 (Schnorr on Ristretto25519) for
  block signatures; BLS12-381 for BEEFY aggregate signatures.
- **Goal.** PoS chain with deterministic finality + bridge-
  friendly succinct proofs.

## Theory

### BABE block production

Slot-based, VRF-driven (similar to Praos):

1. Each slot, validators evaluate VRF on slot+key.
2. Validators with VRF below threshold are *primary* slot
   leaders; if none, *secondary* slot leaders fall back to
   round-robin.
3. The leader produces a block.

### GRANDPA finality

GRANDPA finalises *chains* (sequences of blocks) rather than
individual blocks or checkpoints:

1. Validators vote on the highest known block.
2. A block is *finalised* if a chain ending at that block has
   2/3 stake-weighted votes.
3. Once finalised, all ancestors are also finalised.

This is more efficient than per-block PBFT: finalising a chain
of `k` blocks costs the same as finalising one block.

### BEEFY for bridges

BEEFY (added 2022) provides *succinct* finality proofs:

1. Each finalised block also gets a *MMR* (Merkle Mountain
   Range) commitment.
2. Validators sign these commitments with BLS.
3. The aggregate BLS signature lets a light client verify
   finality with O(1) data, regardless of validator count.

### Comparison: Gasper vs Polkadot

| component       | Ethereum Gasper      | Polkadot           |
| --------------- | --------------------- | ------------------ |
| block production | RANDAO + LMD-GHOST   | BABE (VRF)         |
| finality        | Casper FFG (per epoch) | GRANDPA (per chain) |
| bridge proofs   | sync committee        | BEEFY (BLS)        |
| block time      | 12 sec                | 6 sec              |
| epoch length    | 32 blocks             | 600 slots          |
| year            | 2020                  | 2020-2022          |

### Properties

- *VRF-based block production* (BABE).
- *Chain-based finality* (GRANDPA).
- *Bridge-friendly proofs* (BEEFY).
- *Permissionless validators* (DOT staking).

### Limitations

- *GRANDPA scalability.* Voting is `O(n^2)` per round.
- *BEEFY trust assumption.* Bridges trust BEEFY validator
  set, separate from on-chain validators.
- *Validator-count limit.* ~1000-1300 active validators.

### Subsequent work

- *Polkadot 2.0.* Async backing, agile coretime.
- *JAM* (Join-Accumulate Machine, 2024). Polkadot's next-gen
  unified consensus and execution.

## Practice

- *Polkadot relay chain.* ~300 validators (active), ~1200 total.
- *Kusama.* Polkadot's canary network.
- *Parachains.* ~50 in production: Acala, Moonbeam, Astar,
  Centrifuge, etc.
- Block time: 6 seconds; finality: ~12-60 seconds depending on
  conditions.

### Production-implementation notes

- *Sr25519 vs Ed25519.* Polkadot uses Sr25519 for block
  signatures (variant of Schnorr); BLS for BEEFY only.
- *Validator rotation.* Active validator set rotates each era
  (~24 hours).
- *Slashing.* Up to 100% for unjustified equivocation; smaller
  for offline.

## Verifiability and circuit encoding

**tag: `friendly`.**

BEEFY is specifically designed for bridge SNARK efficiency:
the aggregate BLS signature on the MMR commitment can be
verified in `O(1)` constraints (~10^4 constraints).

GRANDPA's vote aggregation is more complex but tractable.

## Known attacks and limitations

- *GRANDPA vote complexity* makes large validator sets costly.
- *BEEFY committee transitions* require careful handover.
- *Long-range attacks.* Mitigated by weak subjectivity.

## References

- Wood, "Polkadot: Vision for a Heterogeneous Multi-Chain
  Framework", 2016.
- Stewart, Kokoris-Kogias, "GRANDPA: A Byzantine Finality
  Gadget", arXiv 2007.01560, 2020.
- Brown-Cohen, Narayanan, Psomas, Weinberg, "Formal
  Barriers to Longest-Chain Proof-of-Stake Protocols", AFT
  2019 (BABE-related analysis).
- Web3 Foundation, "BEEFY Specification", 2022.

## Implementation notes

The crate provides three placeholder structs (BABE leader,
GRANDPA finaliser, BEEFY commitment) and tests the
finalised-chain semantics.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
