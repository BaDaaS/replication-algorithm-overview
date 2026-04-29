# 0135: Open Problems

## Capstone summary

This module surveys open problems in replication and consensus
as of 2024-2026, organised by the four pillars of this course
(theory, practice, formalisation, verifiability).

## Open problems in theory

### Tight bounds for adaptive corruption

Most BFT protocols (PBFT, HotStuff, Tendermint) have tight
bounds for *static* adversaries: `f < n/3`. Tight bounds for
*adaptive* adversaries (who corrupt nodes after seeing
messages) are an active research area.

- *Open.* Is `f < n/3` achievable with adaptive adversary in
  the partial-synchrony model with O(1) round complexity?

### Async DAG-BFT lower bounds

Modern DAG-BFT (Bullshark, Mysticeti) achieves O(1) commit
latency under good conditions. Tight latency lower bounds
under asynchrony with `f < n/3` Byzantine remain open.

- *Open.* Match the upper bounds with tight async DAG-BFT
  lower bounds.

### Sleepy + adaptive interaction

Sleepy model (module 0089) and adaptive corruption have been
combined informally; tight analysis (e.g., for Praos +
adaptive sleepy) is partially open.

## Open problems in practice

### Validator-count scalability

Modern PBFT-style protocols scale to ~150-200 validators;
Avalanche-style probabilistic protocols to ~thousands;
PoW chains to *anyone*. Combining BFT-finality with
massive-scale validator sets (e.g., Ethereum's 1M validators)
requires aggregate-signature optimisation and committee
selection.

### MEV resistance

Maximal Extractable Value (MEV) is the value extracted by
transaction ordering. Decentralised sequencers, encrypted
mempools (SUAVE, Espresso), and threshold encryption are
active research.

### Byzantine validator detection

Real-time detection of Byzantine validators (from observable
on-chain behaviour) is partial. Better detection enables
faster slashing and better security.

## Open problems in formalisation

### Coq / Lean BFT proofs

Tezos's Tenderbake has a Coq formal proof. Most other production
BFT (HotStuff, Tendermint) do not. A unified proof framework
covering multiple protocols would advance the field.

### Concurrency hazards

Real implementations have concurrency, locking, and timing
issues that pure pseudocode-level proofs miss. Tools like
TLA+, Coq, and Lean for low-level concurrent code remain
nascent.

## Open problems in verifiability

### Bitcoin SNARK light client

Verifying Bitcoin's longest-chain rule + SHA-256 PoW in a
SNARK is computationally expensive (~6*10^7 constraints for
1000 blocks). A practical Bitcoin light client SNARK would
unlock trustless cross-chain bridges to Bitcoin.

- *Open.* Sub-millisecond SNARK light client for Bitcoin
  with reasonable proving cost.

### Verifiable randomness

Decentralised randomness beacons (drand, threshold-BLS) are
production. Verifying them in a SNARK is more challenging.

### Recursive aggregate verification

Aggregating many SNARK proofs into one (recursive
verification) is a Mina/Risc Zero/Aleo theme. Proving
high-throughput protocols (DAG-BFT) recursively is open.

## Pedagogical: course retrospective

This 135-module course covered:

- *Foundations* (timing models, failure models, FLP, lower
  bounds).
- *BFT* (PBFT through HotStuff-2).
- *DAG-BFT* (Aleph through Mahi-Mahi, Sailfish, Autobahn).
- *PoW* (Bitcoin, GHOST, GHOSTDAG, Prism, Conflux).
- *FBA / Avalanche* (Stellar SCP, Ripple, Avalanche family).
- *PoS* (Casper, Algorand, Snow White, Ouroboros family).
- *Production blockchains* (Ethereum/Gasper, Cosmos, Polkadot,
  Solana, Aptos, Sui, Hedera, NEAR, Internet Computer,
  Filecoin, Hyperledger Fabric, Tezos, Mina).
- *Verifiable consensus* (zkBridge, Aleo, zk-rollups, threshold
  cryptography).
- *Meta-theory* (modular blockchains, open problems).

## References

- See `references.bib` for full bibliography.
- Anthropic, OpenAI, GitHub: tools that helped build this
  course at scale.

## Implementation notes

This module is documentation-only; no Rust implementation.
A trivial `lib.rs` is provided to satisfy the workspace.

See also [`HISTORY.md`](../HISTORY.md), section "2024 to 2026".
