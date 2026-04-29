# 0122: Aptos

## Historical context

Aptos (Aptos Labs) launched mainnet in October 2022 as a
successor to Meta's Diem (formerly Libra) project. Aptos's
consensus protocol *AptosBFT* (formerly *DiemBFT*) is a
production HotStuff-2 derivative (module 0059) with several
production refinements:

- *Quorum-store leader rotation* via stake-weighted random
  selection (RANDAO + VRF).
- *Block-STM* parallel execution: transactions execute
  optimistically; conflicts are resolved post-hoc.
- *Reconfiguration* per epoch (~24 hours).

Aptos is one of several Diem-heritage chains (along with Sui,
module 0123) that ship HotStuff-style consensus in production
with millions of users.

## System and threat model

- **Network.** Bounded delay (PSS).
- **Failures.** Byzantine `f < n/3`.
- **Cryptography.** BLS12-381 aggregate signatures, Ed25519
  for blocks.
- **Goal.** PoS chain with HotStuff-2 finality and parallel
  execution.

## Theory

### AptosBFT (HotStuff-2 derivative)

AptosBFT runs HotStuff-2 (module 0059): linear view-change,
two-phase commit, leader rotation. Block finality requires 2/3
voting power; one round per block in good case.

### Quorum-store

Aptos uses *Quorum-store*: validators batch-broadcast
transactions in parallel, indexed by hash; the leader
proposes only the digest of accepted batches. This decouples
data dissemination from consensus, similar to Narwhal-Tusk
(module 0072).

### Block-STM

Transactions execute in parallel using software transactional
memory: speculative execution, conflict detection, deterministic
re-execution on conflict. Throughput scales with CPU cores.

### Comparison: Aptos vs Sui vs Solana

| property            | Aptos              | Sui                 | Solana             |
| ------------------- | ------------------ | ------------------- | ------------------ |
| consensus           | AptosBFT (HS-2)    | Sui Lutris/Mysticeti | Tower BFT         |
| ordering            | total              | total               | total              |
| execution           | Block-STM (STM)    | object-based        | banking-stage      |
| validator count     | ~150               | ~150                | ~1500              |
| block time          | ~250-500 ms        | ~250 ms             | 400 ms             |
| ancestor chain      | Diem               | Diem                | independent        |

### Properties

- *HotStuff-2 finality* in `O(1)` rounds.
- *Parallel execution* via Block-STM.
- *Decoupled data and consensus* via Quorum-store.
- *PoS validator set* with delegation.

### Limitations

- *Validator count.* ~150-200; HotStuff scaling as `O(n)` per
  view.
- *Hardware.* Moderate; not as demanding as Solana but more
  than Cosmos.
- *Block-STM determinism.* Requires careful re-execution on
  conflicts.

### Subsequent work

- *Aptos 2.0.* Async DAG consensus exploration.
- *Quorum store evolution.* Throughput improvements.
- *MoveVM optimisations.*

## Practice

- *Aptos mainnet.* Production since October 2022.
- ~150 validators.
- Block time: 250-500ms.
- Throughput: ~10000 tx/s sustained.
- Diem-heritage Move language.

### Production-implementation notes

- *Validator rotation.* Daily epoch.
- *Slashing.* Active for double-sign; off for downtime
  initially.
- *Quorum store batches.* ~100ms batch interval.

## Verifiability and circuit encoding

**tag: `friendly`.**

Aptos uses BLS12-381 aggregate signatures, which are
SNARK-friendly. A SNARK light client for Aptos verifying
HotStuff-2 finality is feasible at ~10^4 to 10^6 constraints
per epoch.

## Known attacks and limitations

- *Long-range attacks.* Mitigated by weak subjectivity.
- *MEV.* Active; addressed by transaction ordering rules.
- *Stake centralisation.* Largest validators have significant
  voting power.

## References

- Baudet et al., "DiemBFT v4: State Machine Replication in the
  Diem Blockchain", 2021.
- Aptos Foundation, "Aptos Whitepaper", 2022.
- Gelashvili et al., "Block-STM: Scaling Blockchain Execution",
  2022.

## Implementation notes

The crate provides a `QuorumStoreBatch` and a `block_stm_run`
function modelling speculative parallel execution with conflict
detection. Tests verify the API.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to 2024".
