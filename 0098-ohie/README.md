# 0098: OHIE

## Historical context

Haifeng Yu, Ivica Niemann, Yu Kozhaya, Yokogawa, and Prateek
Saxena published "OHIE: Blockchain Scaling Made Simple" at S&P
2020. OHIE (Optimal Hash-based Initial Embedding) takes a
different approach to PoW scaling than Prism (module 0097): run
many *parallel Nakamoto chains* and merge them into a total
order via a deterministic interleaving.

The key insight is that Prism's voting-chain mechanism, while
effective, is complex. OHIE shows that simply running `m`
parallel Nakamoto chains, each with its own PoW target, plus a
merge rule, is enough to achieve `m`x throughput while
preserving Nakamoto security. Compared to Prism, OHIE is
*simpler* (no separate block types, no voter chains) but
provides strictly weaker latency guarantees.

## System and threat model

- **Network.** Bounded-delay PSS.
- **Failures.** Computational honest-majority hashing.
- **Cryptography.** SHA-256 PoW.
- **Goal.** `m`x Bitcoin throughput with similar security
  guarantees.

## Theory

### Parallel chains

OHIE runs `m` independent Nakamoto chains. Each block has a
*chain index* (e.g., taken from low-order bits of the block
hash). A block is valid if its hash is below the target *and*
the chain-index bits match the chain it claims to extend.

Each miner mines blocks for *all* `m` chains simultaneously
(the same hash work covers any chain whose index matches).
Block rewards are split across all `m` chains.

### Merge rule: rank-based interleaving

Each block at position `i` in chain `j` has a logical rank
`(i, j)`. The total order interleaves blocks by rank: blocks at
rank `(0, 0), (0, 1), ..., (0, m-1), (1, 0), (1, 1), ...`.

This rank-interleaving ensures every chain contributes equally;
a slow chain holds up the entire ledger only for its specific
rank position. The total order is *deterministic* given the
chains.

### Throughput: m times Nakamoto

If each chain achieves Bitcoin-like throughput `B` (~7 tx/s),
OHIE achieves `m * B`. With `m = 1000`, OHIE can match Prism's
prototype throughput.

### Confirmation latency

A transaction in chain `j` at position `i` is confirmed when:

- The chain `j` has at least `k` more blocks past `i`, *and*
- All other chains have at least one block at rank `>= i`.

The latter requirement is the cost of total ordering; if any
chain is slow, the entire ledger waits. OHIE handles this with
a *liveness penalty*: chains that fall behind a deadline are
"frozen" and re-bootstrapped.

### Security theorem (Yu et al. 2020, informal)

Under PSS bounded-delay with honest hashing fraction
`alpha > 1/2 + epsilon`: each individual chain satisfies CP/CG/
CQ; aggregated, OHIE provides total ordering with `m`x
throughput.

### Comparison: parallel-chain protocols

| protocol | block types  | total order   | throughput | latency | year |
| -------- | ------------ | ------------- | ---------- | ------- | ---- |
| Bitcoin  | one          | yes           | low        | ~1 hour | 2008 |
| Prism    | three        | yes           | high       | seconds | 2018 |
| OHIE     | one (`m` chains) | yes (rank merge) | high  | minutes | 2020 |
| Aleph 0  | one (DAG)    | yes (linearisation) | high | seconds | 2021 |

OHIE's main attraction is simplicity: no new block types, just
many parallel Nakamoto chains.

### Properties

- *m-x throughput.* Linear in number of chains.
- *Same security.* Each chain inherits Nakamoto's bounds.
- *Simple.* Fewer moving parts than Prism.
- *Deterministic merge.* Total order is canonical.

### Subsequent influence

- *Modular blockchain* designs (Celestia, EigenLayer):
  separate data availability from consensus.
- *Subnet/shard chains.* Cosmos, Polkadot, Avalanche all run
  multiple chains, though with different merge rules.
- *Layer-2 rollups.* Each rollup as a "chain" merged into L1.

## Practice

OHIE has not been deployed at scale; it is mostly a research
benchmark. Its conceptual influence is via the simplicity-vs-
performance trade-off it makes explicit.

### Production-implementation notes

- Choosing `m`: too many chains makes coordination overhead
  grow; OHIE recommends `m = 100` to `1000`.
- Merge-rule deadlines: too short freezes chains often; too
  long delays the ledger.
- Cross-chain transactions: OHIE does not natively support
  atomic cross-chain transfers; an L2 mechanism is required.

### Mining algorithm (proof-of-work function)

OHIE uses double-SHA-256 (Bitcoin's substrate). A single
mining attempt produces one hash; the *low-order bits* of the
hash determine which of the `m` chains it would extend (chain
index = `hash mod m`), and the remaining bits determine
whether it satisfies that chain's target.

This means each hashing attempt has probability `1/m` of
addressing any specific chain, but probability `1` of
addressing some chain. Total honest hash power is therefore
distributed evenly across chains by construction; no miner
needs to choose which chain to target.

The Yu et al. paper does not propose a custom hash; SHA-256
inherits Bitcoin's security analysis. Any collision-resistant
hash would suffice for the per-chain CP/CG/CQ proofs.

## Verifiability and circuit encoding

**tag: `partial`.**

OHIE circuits encode SHA-256 PoW for each chain. Total cost is
roughly `m` times Bitcoin's circuit. The merge rule is a simple
deterministic interleaving (cheap to verify).

## Known attacks and limitations

- *Slow-chain DoS.* An adversary can keep one chain slow,
  delaying the entire ledger.
- *Cross-chain consistency.* No native support; smart contracts
  spanning chains require external mechanism.
- *Parameter `m` calibration.* Mis-calibration breaks throughput
  or security.

## References

- Yu, Niemann, Kozhaya, Yokogawa, Saxena, "OHIE: Blockchain
  Scaling Made Simple", IEEE S&P 2020.

## Implementation notes

The crate provides an `OhieLedger` that holds `m` parallel
chains and a `merge` function that produces the total order via
rank-interleaving. Tests verify small cases.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to 2024".
