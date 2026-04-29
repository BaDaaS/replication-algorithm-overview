# 0099: Conflux

## Historical context

Chenxing Li, Peilun Li, Dong Zhou, Zhe Yang, Ming Wu, Guang Yang,
Wei Xu, Fan Long, and Andrew Chi-Chih Yao published "A
Decentralized Blockchain with High Throughput and Fast
Confirmation" at USENIX ATC 2020 (preprint 2018, arXiv
1805.03870). Conflux is a *tree-graph* (sometimes called
*epoch-DAG*) protocol that combines GHOST-style heaviest-tree
selection with a SPECTRE/PHANTOM-style block DAG.

Conflux's design is a hybrid: each block has a *parent*
(forming a tree on which GHOST chooses a *pivot chain*) plus
*reference edges* to other tips (forming the DAG). The pivot
chain provides total ordering of *epochs*; blocks within an
epoch are ordered topologically.

The Conflux Network mainnet went live in 2020. Throughput in
production is ~1000 to 6000 tx/s with confirmation latency
~tens of seconds.

## System and threat model

- **Network.** Bounded-delay PSS.
- **Failures.** Computational honest-majority hashing.
- **Cryptography.** SHA-256 PoW.
- **Goal.** Bitcoin-like security, Ethereum-like smart
  contracts, throughput orders of magnitude higher.

## Theory

### Tree-graph structure

Each block has:

- *Parent edge.* One block as parent (tree structure).
- *Reference edges.* Set of recent tips not in the parent's
  past (DAG structure).

The tree is a sub-graph of the DAG (the parent-only edges).
GHOST runs on the tree to pick the pivot chain.

### Pivot chain via GHOST

The pivot chain is the *heaviest* tree path from genesis,
selected by GHOST (as in module 0086). Pivot blocks define
*epochs*: the blocks that are referenced (via any path) by a
pivot block but not its predecessor.

### Total ordering

Within each epoch, blocks are topologically sorted. Across
epochs, the pivot chain's natural order is the epoch order.
This gives a deterministic total order over all blocks in the
DAG.

### Why Conflux is fast

- *Throughput.* Reference edges let many parallel blocks be
  included; no orphans.
- *Latency.* The pivot chain commits epochs at GHOST rate
  (medium); blocks within an epoch finalise as the epoch
  progresses.
- *Smart contracts.* Total order from the pivot chain enables
  EVM-compatible semantics.

### Comparison: PHANTOM, OHIE, Conflux

| protocol | order   | structure   | smart contracts | throughput | year |
| -------- | ------- | ----------- | --------------- | ---------- | ---- |
| SPECTRE  | partial | DAG         | partial         | high       | 2016 |
| PHANTOM  | total   | DAG (`k`-cluster) | yes        | high       | 2018 |
| GHOSTDAG | total   | DAG (greedy) | yes            | high       | 2020 |
| OHIE     | total   | parallel chains | yes         | medium     | 2020 |
| Conflux  | total   | tree + DAG  | yes (EVM)       | high       | 2020 |

Conflux's main strength is its EVM compatibility (it runs
Ethereum smart contracts) combined with high throughput.

### Properties

- *Total ordering* via pivot chain.
- *High throughput* via DAG reference edges.
- *EVM-compatible smart contracts*.
- *Permissionless* PoW.

### Limitations

- *DAG storage.* Like all DAG protocols, storage scales with
  block rate * time.
- *Pivot-chain depth.* For confirmation, pivot chain must be
  deep enough; latency depends on hash-rate parameters.
- *Reference-edge selection.* Miners must include all visible
  tips; failure to do so reduces total order quality.

### Subsequent influence

- *Aleph Zero, Narwhal, Bullshark.* DAG-BFT successors with
  similar tree+DAG patterns.
- *Sui, Aptos.* Use a DAG of certified-batch references.
- *Modular blockchains.* Inspired by Conflux's separation of
  ordering and execution.

## Practice

- *Conflux Network mainnet* deployed 2020. Throughput ~1000
  to 6000 tx/s; ~30 second finality.
- Used for stablecoins, DeFi, and Ethereum bridges.
- Developer ecosystem inherits Solidity and EVM tooling.

### Production-implementation notes

- Reference-edge limits: Conflux mainnet caps reference edges
  per block to bound storage and gossip costs.
- Pivot-chain reorganisation: the pivot chain can re-organise
  if a heavier branch appears, similar to GHOST. Reorganisation
  cascades through epoch ordering.
- EVM compatibility: Conflux uses an Ethereum-derived state
  model with conflict-free parallel execution within an epoch.

### Mining algorithm (proof-of-work function)

Conflux uses *Octopus*, a memory-hard PoW function adapted
from Ethash (module 0086). Octopus retains Ethash's DAG-based
memory-hard structure (~5 GB epoch dataset, refreshed every
~3 days) but tweaks the inner mixing function to be
incompatible with existing Ethereum/ETC ASICs.

Octopus inherits Ethash's design rationale: bandwidth-bound
verification favours commodity GPUs, keeping mining
participation broad. The memory cost also discourages mining
pools from optimising via custom silicon during the early
network phase.

| chain    | hash function | DAG | block rate |
| -------- | ------------- | --- | ---------- |
| Conflux  | Octopus       | yes | ~2 blocks/s (incl. tipset) |

## Verifiability and circuit encoding

**tag: `partial`.**

Conflux circuits encode Octopus PoW per block (Keccak-256 +
DAG memory access; substantially more expensive than SHA-256
in a SNARK due to the memory-access pattern), tree-graph
traversal (similar to GHOST), and epoch-block topological
sorting. Total cost exceeds a GHOST + SHA-256 circuit by the
DAG-access overhead.

A SNARK light client for Conflux can prove only the pivot
chain (cheap, like a Bitcoin SNARK light client); transaction
inclusion within an epoch requires Merkle membership.

## Known attacks and limitations

- *Pivot-chain re-org.* Long re-orgs (rare) cause epoch
  ordering changes.
- *Reference manipulation.* Miners may game reference-edge
  selection, but the GHOST pivot chain ultimately decides.
- *Spam blocks.* High-rate adversarial mining inflates DAG
  storage.

## References

- Li, Li, Zhou, Yang, Wu, Yang, Xu, Long, Yao, "A Decentralized
  Blockchain with High Throughput and Fast Confirmation",
  USENIX ATC 2020.
- Conflux Network whitepaper, 2020.

## Implementation notes

The crate provides a `TreeGraph` with parent edges (tree) plus
reference edges (DAG), and a `pivot_chain` function that
descends by heaviest-tree-subgraph (GHOST). Tests verify the
pivot chain is computed on the tree only, ignoring reference
edges.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to 2024".
