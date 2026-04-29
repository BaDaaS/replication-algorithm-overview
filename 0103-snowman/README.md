# 0103: Snowman

## Historical context

Snowman is a *linear-chain* variant of the Avalanche protocol
family (module 0102), introduced as part of the Avalanche
platform in 2020. Where Avalanche operates on a DAG of
transactions, Snowman operates on a single linear chain of
blocks. Each block is voted on via the Snowball mechanism.

Snowman was created because EVM smart contracts require total
ordering (a strict block sequence), which the DAG-based
Avalanche cannot provide cheaply. Snowman is the consensus
protocol used by Avalanche's *C-Chain* (EVM compatible), *P-
Chain* (platform), and most *Subnets*.

## System and threat model

- **Network.** Asynchronous (Avalanche family).
- **Failures.** Byzantine `f < n/3` (parameter-dependent).
- **Cryptography.** Standard signatures.
- **Goal.** Total-order consensus on a linear blockchain via
  Snowball voting.

## Theory

### Linear chain

Each block has exactly one parent. The chain is a linear
sequence of blocks, like Bitcoin. But block acceptance is
decided by Snowball (probabilistic gossip-based voting), not
PoW or PBFT.

### Snowball on blocks

For each block:

1. Sample `k` random nodes' preferences.
2. If `>= alpha * k` agree on a block, increment confidence
   for that block.
3. After `beta` consecutive successes, the block is *accepted*
   and added to the chain.

### Why linear, not DAG

DAG-based Avalanche provides high throughput but partial order
(per-pair ordering of conflicting transactions). Smart
contracts often require total order: e.g., DEX trades depend
on the order in which they execute, not just on consistency
within pairs.

Snowman trades parallelism for total order, providing
deterministic block-level finality.

### Comparison: Avalanche vs Snowman

| property               | Avalanche           | Snowman          |
| ---------------------- | ------------------- | ---------------- |
| structure              | DAG of transactions | linear chain     |
| order                  | partial             | total            |
| smart contracts        | limited             | EVM-compatible   |
| throughput             | very high           | high             |
| finality               | probabilistic       | probabilistic    |
| use case               | UTXO (X-Chain)      | smart contracts  |

### Properties

- *Total ordering* via linear chain.
- *Smart-contract compatibility* (EVM).
- *Snowball convergence.*
- *Async safety.*

### Subnets

Each Avalanche Subnet runs Snowman with its own validator
committee and parameters. This is the production deployment of
Snowman.

### Subsequent influence

- *Subnets-as-Services.* Avalanche Subnets enable dedicated
  app-chains.
- *Frosty (Yin Sirer 2024).* Successor protocol with formal
  security analysis.
- *Lux Network, Beam, others.* Avalanche Subnet ecosystems.

## Practice

- *Avalanche C-Chain.* Production EVM smart-contract chain.
- *Avalanche P-Chain.* Production platform chain (validator
  registration, subnet creation).
- *Avalanche Subnets.* Hundreds of subnets in production.
- ~1000 validators per chain.

### Production-implementation notes

- Same `k, alpha, beta` parameters as Avalanche
  (k=20, alpha=12, beta=15-20).
- Stake-weighted sampling for Sybil resistance.
- Block finality typically <= 1 second.

## Verifiability and circuit encoding

**tag: `partial`.**

Snowman circuits encode the linear-chain block sequence plus
the Snowball voting state machine. Like Avalanche, the
probabilistic finality is hard to encode directly; practical
SNARK light clients verify only finalised blocks.

## Known attacks and limitations

- Same as Avalanche: probabilistic finality, partition-
  liveness, parameter tuning.
- *Linear bottleneck.* Total order limits throughput vs DAG
  Avalanche.

## References

- Yin, Sekniqi, van Renesse, Sirer, "Snow*: Avalanche-style
  Probabilistic Consensus", Avalanche white paper, 2020.
- Avalanche Documentation, "Snowman Consensus", 2020 onward.

## Implementation notes

The crate provides a `SnowmanChain` that holds a Vec of
finalised blocks and a `pending` Snowball state for the next
block. Tests verify a block is accepted after `beta`
consecutive successful queries.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
