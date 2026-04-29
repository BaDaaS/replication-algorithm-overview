# 0127: Filecoin

## Historical context

Filecoin (Protocol Labs) launched mainnet in October 2020 as a
decentralised storage network. Its consensus is built on
*Proof-of-Storage* (storage providers prove they hold client
data) integrated with a chain protocol:

- *Expected Consensus (EC).* The original Filecoin chain
  protocol. A leader-election lottery weighted by *power*
  (storage provided), with multiple winners per round (a *tipset*).
- *F3 (Fast Finality)* added in 2024. PBFT-style finality on
  EC tipsets, providing deterministic finality in seconds.

Filecoin's *Expected Consensus* is unique in that the
"chain" is a sequence of *tipsets* (sets of blocks at the
same height), all of which contribute to the canonical state.

## System and threat model

- **Network.** Bounded-delay PSS.
- **Failures.** Byzantine `f < 1/2` of storage power (EC) or
  `f < 1/3` of finality committee (F3).
- **Cryptography.** BLS aggregate signatures, VRF, SDR/SealEC
  proofs of storage.
- **Goal.** Decentralised storage with chain-based payments.

## Theory

### Expected Consensus

Each round, every storage provider:

1. Computes their *power* (data they have stored, attested via
   Proofs-of-Spacetime / PoSt).
2. Evaluates a VRF on the round; if below their power-
   proportional threshold, they are an *eligible leader*.
3. Multiple winners propose blocks; all valid blocks form a
   *tipset*.

The *heaviest tipset* (by aggregate weight) is canonical.

### Tipsets and weight

A tipset is a set of blocks at the same height with the same
parent tipset. The chain is a sequence of tipsets.

Weight is computed by summing per-block weights based on
power. Heaviest-tipset rule applies.

### F3 fast finality

F3 (added 2024) runs a PBFT-style protocol over EC tipsets:

1. *Quorum-power vote.* `2/3` of total power signs a tipset.
2. *Justification.* Tipset becomes justified when 2/3 sign.
3. *Finalisation.* Two consecutive justified tipsets means the
   first is finalised.

Inspired by Casper FFG (module 0106), adapted for EC.

### Comparison: Filecoin vs Bitcoin vs Ethereum

| property              | Filecoin         | Bitcoin       | Ethereum       |
| --------------------- | ---------------- | ------------- | -------------- |
| Sybil resistance      | proof-of-storage | proof-of-work | proof-of-stake |
| canonical chain       | heaviest tipset  | longest chain | LMD-GHOST      |
| finality              | F3 (recent)      | probabilistic | Casper FFG     |
| block time            | 30 sec           | 10 min        | 12 sec         |
| storage utility       | yes              | no            | no             |
| year                  | 2020             | 2008          | 2015           |

### Properties

- *Storage-utility consensus.* Sybil resistance via useful
  storage proofs.
- *Tipset structure* (multiple winners per round).
- *Fast finality via F3.*
- *Permissionless.*

### Limitations

- *Storage proof complexity.* PoSt and PoRep are computationally
  intensive.
- *Tipset semantics.* More complex than linear chains.
- *Storage-power centralisation.* Largest storage providers
  dominate.

### Subsequent work

- *F3 finality.* Production deployment 2024.
- *Filecoin Plus.* Verified storage with subsidies.
- *Storage market.* Smart-contract-driven storage deals.

## Practice

- *Filecoin mainnet.* Production since October 2020.
- ~3000 active storage providers.
- Block time: 30 seconds.
- Storage capacity: ~20 EiB (exbibyte).
- F3 finality: ~30-60 seconds (post-2024 upgrade).

### Production-implementation notes

- *PoSt.* Proof-of-Spacetime; storage providers post
  WindowedPoSts every 24 hours.
- *PoRep.* Proof-of-Replication; providers prove they have
  unique replicas.
- *FVM.* Filecoin Virtual Machine for smart contracts on EC.

## Verifiability and circuit encoding

**tag: `partial`.**

PoRep and PoSt use SNARKs heavily (Groth16 in production). EC
itself is more complex due to the tipset structure but
tractable. F3's BLS aggregate signatures are SNARK-friendly.

## Known attacks and limitations

- *Storage power centralisation.* Largest providers dominate.
- *Sybil via tipsets.* Mitigated by the EC weight rule.
- *PoSt failures.* Slashing for missed PoSts.

## References

- Protocol Labs, "Filecoin Whitepaper", 2017.
- Filecoin Specification, 2020.
- Protocol Labs, "F3: Fast Finality", 2024.

## Implementation notes

The crate provides a `Tipset` struct holding a set of block
ids and an aggregate weight. Tests verify weight-comparison
rule.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
