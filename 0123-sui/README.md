# 0123: Sui

## Historical context

Sui (Mysten Labs) launched mainnet May 2023 as a Diem-heritage
chain emphasising *object-centric* execution and a custom
consensus stack:

- *Sui Lutris.* Hybrid optimistic/sequencer consensus for
  *single-owner* transactions: many transactions skip global
  consensus entirely if they touch only owner-private state.
- *Mysticeti* (module 0076). Sui's DAG-BFT consensus for
  *shared-object* transactions (which require total order).

This dual-mode design distinguishes Sui from Aptos (module
0122) and other HotStuff-2 derivatives: Sui's UTXO-style
parallel paths bypass consensus when possible.

## System and threat model

- **Network.** Bounded delay PSS for shared-object consensus;
  asynchronous safety for single-owner.
- **Failures.** Byzantine `f < n/3`.
- **Cryptography.** BLS12-381 aggregate signatures.
- **Goal.** Production object-store with low-latency consensus.

## Theory

### Single-owner fast path (Sui Lutris)

Each Sui *object* has an owner. Transactions modifying only
owner-private objects don't need global ordering: the owner
signs, validators sign, transaction is committed. Latency:
network round-trip (~hundreds of ms).

### Shared-object slow path (Mysticeti)

Transactions touching *shared* objects (e.g., AMMs, DEXs)
require total ordering. Mysticeti (module 0076) is Sui's
DAG-BFT consensus: validators run a Narwhal-style mempool
DAG, then commit waves linearly.

### Comparison: single-owner vs shared

| path                | mechanism            | latency     |
| ------------------- | -------------------- | ----------- |
| single-owner        | Lutris (sigs only)   | ~400 ms     |
| shared-object       | Mysticeti DAG-BFT    | ~600 ms     |

Most Sui transactions are single-owner (NFT transfers, simple
payments); shared-object only for AMMs, governance, etc.

### Properties

- *Dual-mode execution.* Most transactions skip global order.
- *DAG-BFT for shared.* Mysticeti commits in `O(1)` waves.
- *Object-centric.* Move-derived language with strong
  ownership.
- *PoS validator set.*

### Limitations

- *Validator count* (~150-200).
- *Shared-object bottleneck.* Mysticeti's throughput limits
  shared-object workloads.
- *Object model complexity.* Application developers must reason
  about ownership.

### Subsequent work

- *Mysticeti-C.* Communication-optimised variant.
- *Mahi-Mahi* (module 0077). Further DAG-BFT improvements.
- *Sui 2.0.* Forthcoming optimisations.

## Practice

- *Sui mainnet.* Production since May 2023.
- ~150 validators.
- Block time: ~600ms (shared-object), ~400ms (single-owner).
- Throughput: ~10000+ tx/s.

### Production-implementation notes

- *Object model.* Each transaction declares input objects;
  conflicts detected at submission.
- *Validator weighting.* Stake-weighted; ~30% turnover annually.
- *Slashing.* Active for double-sign.

## Verifiability and circuit encoding

**tag: `friendly`.**

Sui uses BLS aggregate signatures throughout, well-suited to
SNARKs. Mysticeti's DAG structure is more complex but still
SNARK-tractable.

## Known attacks and limitations

- *Shared-object DoS.* High contention degrades throughput.
- *Object model bugs.* Application logic errors can lock
  objects.
- *Stake centralisation.* Largest validators dominate.

## References

- Mysten Labs, "The Sui Smart Contracts Platform", 2022.
- Babel et al., "Mysticeti", 2024.
- Blackshear et al., "Move: A Language With Programmable
  Resources", 2019.

## Implementation notes

The crate provides a `Tx` enum (`SingleOwner` /
`SharedObject`) and a `route` function dispatching to the
appropriate consensus path. Tests verify routing.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to 2024".
