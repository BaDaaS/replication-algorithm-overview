# 0109: Snow White

## Historical context

Phil Daian, Rafael Pass, and Elaine Shi published "Snow White:
Robustly Reconfigurable Consensus and Applications to
Provably Secure Proof of Stake" at FC 2019. Snow White takes
the *sleepy* model (Pass-Shi 2017, module 0089) and constructs
the first provably-secure proof-of-stake protocol explicitly in
that model.

Snow White's contribution: a PoS chain protocol with formal
reductions to the GKL backbone properties (CP / CG / CQ),
robust against intermittent failures (sleepy adversary),
adaptive adversaries (player-replaceable role assignment), and
*reconfiguration* (stake updates over time).

The "robustly reconfigurable" property is the key advance over
previous PoS designs: Snow White handles validators joining
and leaving without losing security, and supports arbitrary
stake-updates as the underlying ledger changes.

## System and threat model

- **Network.** Bounded-delay (sleepy + PSS).
- **Failures.** Sleepy: intermittent unboundedly-many crashes;
  Byzantine adaptive within the sleepy framework.
- **Cryptography.** VRF (verifiable random function) for
  proposer selection; standard signatures.
- **Goal.** Sleepy-secure PoS with handles for stake
  reconfiguration.

## Theory

### Slot-based protocol

Snow White divides time into *slots*; each slot has zero or
one *eligible proposer*. Eligibility is determined by a
slot-leader VRF: proposer with VRF-output below a stake-
weighted threshold becomes the eligible leader.

This is similar to Algorand's sortition (module 0108) but at
the leader level, not committee level.

### Robust reconfiguration

The protocol explicitly supports *stake updates*: as new
transactions add or remove stake, the slot-leader sortition
adjusts. The challenge is *consistency*: which stake snapshot
do we use to determine slot leaders?

Snow White solves this with a *snapshot delay*: stake at slot
`s` is determined by the ledger as of slot `s - delta`, where
`delta` is large enough that the ledger up to `s - delta` is
finalised in all honest views.

### Sleepy theorem (Daian-Pass-Shi 2019, informal)

Under the sleepy model with online honest stake fraction
`alpha > 1/2 + epsilon`, Snow White satisfies CP / CG / CQ
with parameters depending on `alpha` and the snapshot delay.

### Comparison: Sleepy chain protocols

| protocol     | reconfiguration | adaptive | sleepy-secure | year |
| ------------ | ---------------- | -------- | -------------- | ---- |
| Sleepy 2017  | static           | partial  | yes            | 2017 |
| Snow White   | yes (snapshot)   | yes      | yes            | 2019 |
| Praos        | yes (epoch)      | yes (forward-secure) | yes | 2018 |
| Algorand     | yes (committee)  | yes (replaceability) | partial sync (not sleepy) | 2019 |

Snow White is sleepy-secure (handles arbitrary offline) where
Algorand requires partial synchrony.

### Properties

- *Sleepy-secure* (intermittent crashes tolerated).
- *Adaptive-adversary resistant* via player replaceability.
- *Robust reconfiguration* via snapshot delay.
- *Permissionless PoS.*

### Subsequent influence

- *Ouroboros Praos* (module 0126). Forward-secure signatures
  for sleepy-secure PoS.
- *Solana.* Slot-based protocol with VRF-style leader
  selection.
- *Aptos / Sui.* PoS leader rotation with stake-weighted
  selection.

## Practice

Snow White itself was not deployed; the design is a research
foundation for production PoS chains. Cosmos (Tendermint),
Cardano (Praos), and Algorand all incorporate sleepy-style
robustness in their production deployments.

### Production-implementation notes

- *Snapshot delay.* Production chains use ~6-24 hours of
  finalised history to determine current slot leaders.
- *Validator activation/deactivation.* Snow White supports
  arbitrary stake updates; production chains usually limit
  per-epoch changes.
- *VRF construction.* Production VRFs are built on Ed25519
  (Algorand) or BLS (Ethereum, Praos).

## Verifiability and circuit encoding

**tag: `partial`.**

Snow White circuits encode VRF verification, slot-leader
selection, and the sleepy-snapshot bookkeeping. Cost is
similar to Algorand: VRF per slot, signature verification per
block.

A SNARK-friendly variant could use BLS-VRF and aggregate
signatures, similar to Mina's Samasika (module 0145).

## Known attacks and limitations

- *Snapshot delay.* Too short -> reconfig conflicts; too long
  -> stale stake distribution.
- *Long-range attacks.* Mitigated by weak subjectivity.
- *Posterior corruption.* Mitigated by forward-secure
  signatures.

## References

- Daian, Pass, Shi, "Snow White: Robustly Reconfigurable
  Consensus and Applications to Provably Secure Proof of
  Stake", FC 2019.
- Pass, Shi, "The Sleepy Model of Consensus", Asiacrypt 2017.

## Implementation notes

The crate provides a `Snapshot` struct holding a stake-snapshot
at some slot `s`, and a `slot_leader` function deterministically
picking the leader for slot `s + delta` based on the snapshot.
Tests verify the snapshot-delay logic.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
