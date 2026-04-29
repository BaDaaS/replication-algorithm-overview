# 0134: Modular Blockchain Meta-Theory

## Historical context

The "modular blockchain" thesis crystallised around 2021-2024,
articulated most prominently by Mustafa Al-Bassam (Celestia),
Vitalik Buterin (Ethereum's rollup-centric roadmap),
Sreeram Kannan (EigenLayer), and others. The thesis: a
blockchain is not a monolith but a stack of *separable*
components.

## The four pillars

A modular blockchain stack consists of four functions:

1. *Execution.* Running smart contracts and computing state
   transitions.
2. *Settlement.* Resolving disputes and providing finality
   anchoring.
3. *Consensus / Ordering.* Agreeing on the canonical order of
   transactions.
4. *Data Availability (DA).* Ensuring that transaction data is
   published and retrievable.

Each function can be a separate chain or layer:

- *Execution layer.* Optimistic rollups (Arbitrum, Optimism),
  zk-rollups (StarkNet, zkSync, Polygon zkEVM).
- *Settlement layer.* Ethereum mainnet (often).
- *Consensus / DA layer.* Celestia, EigenDA, Avail.

## Theory

### CAP-style trade-offs in modular stacks

Each layer has its own consistency/availability/partition
trade-offs. Modular composition introduces new questions:

- *Cross-layer reorgs.* If consensus layer reorgs, what
  happens to execution layer's state?
- *Cross-layer DoS.* DA-layer outage affects all rollups
  using it.
- *Trust assumptions.* The weakest link determines security.

### Data Availability Sampling (DAS)

Celestia's key contribution: light clients sample random
chunks of block data to *probabilistically verify
availability* without downloading the full block.

- Erasure coding (Reed-Solomon) makes any 1/4 of chunks
  reconstruct the full block.
- Light clients sample `O(log N)` chunks.
- DA Attack: requires hiding `> 3/4` of the block, which is
  heavily detected.

### Comparison: monolithic vs modular

| property             | Monolithic (Solana)  | Modular (Ethereum + L2s) |
| -------------------- | -------------------- | -------------------------- |
| execution            | on-chain             | rollups                    |
| settlement           | on-chain             | L1                         |
| consensus            | on-chain             | L1                         |
| data availability    | on-chain             | L1 / Celestia / EigenDA    |
| upgrades             | network-wide         | per-layer                  |
| MEV                  | concentrated         | dispersed                  |
| year                 | 2020                 | 2022-2024                  |

### Subsequent work

- *Celestia.* Production DA-only chain since 2023.
- *EigenLayer / EigenDA.* Restaking-based DA.
- *Avail.* Polkadot's DA chain.
- *Espresso.* Shared sequencer + DA.

## Practice

The modular thesis is shaping current Ethereum L2 architecture
(rollup-centric roadmap, EIP-4844 blob transactions). Multiple
DA chains are in production.

## Verifiability and circuit encoding

**tag: `partial`.**

Modular stacks compose verifiability across layers:

- L1 finality verifiable via SNARK light client (e.g.,
  zkBridge module 0130).
- Rollup state verifiable via validity proofs (zk-rollups) or
  fraud proofs (optimistic).
- DA verifiable via DAS + erasure coding proofs.

A SNARK light client can stitch verifiability across the
stack.

## Known attacks and limitations

- *DA outages.* If DA fails, rollups stall.
- *Settlement-execution misalignment.* L2 and L1 finality
  have different rates.
- *Cross-layer trust assumptions.* Weakest link determines
  security.

## References

- Al-Bassam, "Lazyledger: A Distributed Data Availability
  Ledger With Client-Side Smart Contracts", 2019.
- Buterin, "An incomplete guide to rollups", Ethereum blog,
  2021.
- Kannan et al., "EigenLayer: Restaking on Ethereum", 2023.

## Implementation notes

The crate provides a `Stack` struct enumerating layer
implementations and a `cap_layer` predicate identifying which
layer determines the stack's security parameters.

See also [`HISTORY.md`](../HISTORY.md), section "2024 to 2026".
