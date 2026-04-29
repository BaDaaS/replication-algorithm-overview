# 0118: Gasper

## Historical context

Vitalik Buterin, Diego Hernandez, Thor Kamphefner, Khiem Pham,
Zhi Qiao, Danny Ryan, Juhyeok Sin, Ying Wang, and Yan X Zhang
published "Combining GHOST and Casper" (arXiv 2003.03052) in
2020. Gasper is the consensus protocol of Ethereum's Beacon
Chain and post-Merge mainnet. It combines:

- *LMD-GHOST* (Latest Message Driven GHOST, module 0086)
  fork-choice rule.
- *Casper FFG* (module 0106) finality gadget.

Each component does a different job:

- LMD-GHOST tells nodes which chain to *follow* (live updates).
- Casper FFG tells nodes which checkpoints are *finalised*
  (irreversible).

Gasper has been running on Ethereum's Beacon Chain since
December 2020 and on the unified Ethereum mainnet since the
Merge in September 2022.

## System and threat model

- **Network.** Bounded-delay (PSS).
- **Failures.** Byzantine `< 1/3` of stake (for finality);
  honest-majority for liveness.
- **Cryptography.** BLS aggregate signatures; randao for
  proposer selection; VRF-style RANDAO mix.
- **Goal.** PoS chain with deterministic finality and
  responsive liveness.

## Theory

### Two layers

| layer    | function           | mechanism      | finality      |
| -------- | ------------------ | -------------- | ------------- |
| Block    | tx ordering        | LMD-GHOST      | live          |
| Checkpoint | finality       | Casper FFG     | deterministic |

### LMD-GHOST attestations

Each validator attests to (a) the block they consider current
head and (b) which checkpoint links they support for FFG.
Attestations are aggregated via BLS into committee
attestations.

LMD-GHOST: the canonical head is the block in the heaviest
subtree, where weight = sum of latest attestations from each
validator pointing into the subtree.

### FFG checkpoint finalisation

Every 32 blocks (one *epoch*) is a checkpoint candidate.
Validators vote on links between checkpoints. A checkpoint is
*justified* on 2/3 stake; *finalised* on 2/3 + immediate
descendant.

### Subtle interaction: balancing attacks

Gasper has a known issue: under adversarial conditions, the
LMD-GHOST + FFG combination can be manipulated to delay
finality (e.g., the *balancing attack* by Neuder-Moroz-Rao-Parkes
2020). Mitigations: proposer-boost (give the current proposer
extra weight in LMD-GHOST), confirmation rules, eventual
view-merge (Goldfish proposal).

### Theorem (Buterin et al. 2020, informal)

Under bounded-delay PSS with `< 1/3` Byzantine stake: Gasper
provides:

- *Plausibility* (no two conflicting checkpoints finalise).
- *Plausible liveness* (eventually finality progresses, modulo
  balancing attacks).

The plausibility result is rigorous; plausible liveness is
weaker than ideal (it does not rule out specific finality
delays).

### Comparison: Gasper vs Tendermint vs Praos

| property            | Gasper           | Tendermint      | Praos       |
| ------------------- | ---------------- | --------------- | ----------- |
| finality            | Casper FFG       | per-block       | probabilistic |
| fork choice         | LMD-GHOST        | locked          | longest chain |
| latency             | ~6 min finality  | seconds         | ~12 hours   |
| validator count     | ~1M              | ~150            | ~3000       |
| network model       | PSS              | PSS             | PSS         |
| Byzantine threshold | 1/3 (finality)   | 1/3             | 1/2 (CP)    |

### Properties

- *Deterministic finality* (FFG every ~6 min).
- *Live updates* (LMD-GHOST per block).
- *Massive scale* (Ethereum has ~1M validators).
- *BLS aggregation* enables this scale.

### Limitations

- *Balancing attacks* on LMD-GHOST + FFG interaction.
- *Inactivity leak* required when `> 1/3` validators offline.
- *Long finality latency* (~6 min vs Tendermint's seconds).

### Subsequent work

- *Goldfish* (Buterin 2022). Improved fork-choice rule
  resistant to balancing attacks.
- *3SF (3-slot Finality)*. Faster finality proposal.
- *Rainbow staking.* Validator role rotation.

## Practice

- *Ethereum Beacon Chain.* Production December 2020.
- *Ethereum mainnet.* Post-Merge September 2022.
- ~1M validators with 32 ETH each (~$100B+ total stake).
- Block time 12 seconds; epoch length 32 blocks (~6.4 min).

### Production-implementation notes

- *Validator client diversity.* Lighthouse, Prysm,
  Teku, Nimbus, Lodestar; diversity reduces single-bug
  risk.
- *MEV-Boost.* Block proposers outsource block construction
  to specialised builders for MEV revenue.
- *Restaking.* EigenLayer leverages staked ETH for additional
  services.

## Verifiability and circuit encoding

**tag: `friendly`.**

Gasper circuits are at the centre of Ethereum's SNARK light-
client ecosystem (zkBridge, Succinct's Telepathy, Helios).
BLS aggregate verification is well-suited to SNARKs (~10^4 to
10^6 constraints per epoch's aggregate). LMD-GHOST is more
complex but tractable.

## Known attacks and limitations

- *Balancing attacks* on LMD-GHOST.
- *Validator censorship* via MEV-Boost.
- *Finality stalls* during major outages (mitigated by
  inactivity leak).
- *Long-range attacks* mitigated by weak subjectivity.

## References

- Buterin et al., "Combining GHOST and Casper", arXiv
  2003.03052, 2020.
- Neuder, Moroz, Rao, Parkes, "Selfish Behavior in the Tezos
  Proof-of-Stake Protocol", AFT 2020 (balancing attack
  background).
- Buterin, "Goldfish Proposal", Ethereum Research blog, 2022.

## Implementation notes

The crate provides a `Validator` type, a `Gasper` state with
LMD-GHOST head + FFG justified/finalised checkpoints, and a
`vote` method that records attestations. Tests verify head
selection follows weight and FFG follows 2/3 quorum.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to 2024".
