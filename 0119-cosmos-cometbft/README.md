# 0119: Cosmos / CometBFT

## Historical context

Cosmos Hub launched March 2019 with Tendermint Core (module
0055) as its consensus engine. Tendermint Core was renamed
*CometBFT* in February 2023 after a governance dispute split
the codebase. CometBFT remains the canonical consensus layer
for the Cosmos ecosystem; ~80+ chains in production use it.

The Cosmos design is a *hub-and-zone* architecture: each
*zone* runs its own CometBFT chain; zones communicate via
*IBC* (Inter-Blockchain Communication). The Cosmos Hub
itself is one such zone.

## System and threat model

- **Network.** Bounded delay (PSS).
- **Failures.** Byzantine `f < n/3` of voting power.
- **Cryptography.** Ed25519 signatures (default), BLS optional.
- **Goal.** PoS chain with deterministic finality at every
  block.

## Theory

### Tendermint algorithm

CometBFT runs the Tendermint algorithm (module 0055): a
three-phase commit (propose, prevote, precommit) per round,
with view-change on timeout. Each block is finalised by 2/3
voting-power signatures.

### Per-block finality

Unlike Ethereum's Gasper (module 0118) which finalises every
~6 minutes (epoch boundary), CometBFT finalises *every block*.
Trade-off: finalisation requires synchronous voting, limiting
validator count to ~150-200.

### Validator set

Each CometBFT chain has a fixed validator set (with delegated
proof-of-stake selecting validators). Validator changes occur
via on-chain governance and apply at epoch boundaries.

### IBC

Inter-Blockchain Communication enables cross-chain messages
between any two CometBFT chains. Light-client proofs are
verified on the destination chain, not requiring trust beyond
the source chain's validators.

### Comparison: CometBFT, Gasper, HotStuff-2

| property              | CometBFT       | Gasper        | HotStuff-2     |
| --------------------- | -------------- | ------------- | -------------- |
| finality              | per block      | per epoch     | per block      |
| validator scale       | ~200           | ~1M           | ~hundreds      |
| latency               | ~6 sec         | ~6 min        | ~1 sec         |
| view-change           | rotating       | per epoch     | linear         |
| signature aggregation | Ed25519 only   | BLS           | BLS optional   |
| year                  | 2014/2019      | 2020          | 2022           |

CometBFT's main strength is per-block deterministic finality
and IBC interop; weakness is validator-count scaling.

### Properties

- *Per-block finality.* No fork after commit.
- *PoS with delegation.* Stakeholders delegate to validators.
- *IBC interop.* Cross-chain messages with cryptographic
  proofs.
- *Validator slashing.* Double-sign and downtime penalised.

### Limitations

- *Validator count limit.* ~150-200 due to message complexity.
- *Per-block finality cost.* Latency higher than streamlined
  protocols (HotStuff-2).
- *No BLS by default.* Ed25519 limits aggregation; BLS being
  added.

### Subsequent work

- *CometBFT.* Forked from Tendermint Core, 2023.
- *Cosmos SDK.* Application framework atop CometBFT.
- *Inter-Blockchain Communication.* Cross-chain protocol.
- *Interchain Security.* Cosmos Hub's "shared security" for
  consumer chains.

## Practice

- *Cosmos Hub* (ATOM). Production since March 2019.
- *80+ chains* in production: Osmosis, Juno, Akash, dYdX v4,
  Sei, Celestia, Berachain, Injective, Kujira, etc.
- ~150 validators per chain on average.
- Block time: 6 seconds; finality: 6 seconds.

### Production-implementation notes

- *Block time.* Configurable per chain; typically 5-7 seconds.
- *Validator slashing.* 5% for double-sign; 1% per ~50% downtime
  per period.
- *IBC.* Cross-chain assets and messages; ~$100M+ daily volume.

## Verifiability and circuit encoding

**tag: `partial`.**

CometBFT circuits encode Ed25519 signature verification per
validator + the Tendermint state machine. Ed25519 is *not*
SNARK-friendly (~10^6 constraints/signature); production
SNARK light clients for Cosmos require either substituting
BLS-on-BLS12-381 or accepting high SNARK proving cost.

Polymer Labs and Cosmos's own zk-IBC efforts target this
problem.

## Known attacks and limitations

- *Double-sign attacks.* Slashed via on-chain evidence
  (provable via signed-block evidence).
- *Long fork stalls.* If `> 1/3` validators offline, no
  finality. Requires manual intervention (governance).
- *Liveness during partition.* CometBFT halts safety always,
  liveness halts during partition.

## References

- Buchman, Kwon, "Tendermint: Byzantine Fault Tolerance in
  the Age of Blockchains", 2014.
- Buchman, Kwon, Milosevic, "The latest gossip on BFT
  consensus", arXiv 1807.04938, 2018.
- CometBFT documentation, 2023 onward.

## Implementation notes

The crate provides a `Validator` set and a `Block` finaliser
that records 2/3 precommit signatures. Tests verify finality
threshold.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
