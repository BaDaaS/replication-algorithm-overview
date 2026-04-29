# 0113: Ouroboros Crypsinous

## Historical context

Thomas Kerber, Aggelos Kiayias, Markulf Kohlweiss, and Vassilis
Zikas published "Ouroboros Crypsinous: Privacy-Preserving Proof-
of-Stake" at IEEE S&P 2019. Crypsinous is the *privacy-
preserving* variant of Ouroboros: stakeholder identities and
stake amounts are hidden in zero-knowledge while still
satisfying the safety/liveness properties of Praos.

Privacy is essential for some applications (financial trading,
sensitive identities) but is incompatible with public stake
distributions. Crypsinous solves this by:

1. *Anonymous stake registration.* Each stakeholder registers
   stake to a pseudonymous commitment; the registration proof
   uses zero-knowledge.
2. *Anonymous slot leadership.* When a stakeholder is eligible
   for a slot, they prove eligibility without revealing
   identity (using ZK proof of stake-weighted lottery).
3. *Anonymous block signing.* Block signatures use ring-
   signature or ZK-NIZK schemes that prove "some valid
   stakeholder signed" without revealing which one.

## System and threat model

- **Network.** Bounded-delay PSS.
- **Failures.** Adaptive Byzantine; safety threshold
  `alpha > 1/2`.
- **Cryptography.** Pairing-based ZK-SNARK or zk-STARK,
  pseudonymous commitments, ring signatures.
- **Goal.** PoS with anonymous stake and anonymous block
  authorship.

## Theory

### Privacy goals

Crypsinous distinguishes between:

1. *Public information.* Total stake distribution (aggregate),
   block contents (transactions), chain history.
2. *Private information.* Individual stakeholder identities,
   per-stakeholder stake amounts, mapping from blocks to
   producers.

The privacy goal: an adversary observing the network cannot
identify which physical entity controls which stake or
produced which block.

### ZK proof of slot eligibility

Each stakeholder produces a *proof of slot eligibility* that
shows:

- They control some commitment in the registered stake set.
- Their VRF output for this slot is below the stake-weighted
  threshold.
- Without revealing which commitment.

This is a standard ZK proof of OR-disjunction: prove
membership in the stake set without revealing the index.

### Anonymous block signatures

Block authorship is similarly hidden: the block carries a ring
signature over the registered stake set, proving *some*
eligible stakeholder signed without revealing which.

### Theorem (Kerber et al. 2019, informal)

Under bounded-delay PSS with adaptive Byzantine `< 1/2`:
Crypsinous achieves CP / CG / CQ *and* anonymity (computational
indistinguishability of block-producer identity).

### Privacy comparison

| protocol         | stake privacy | identity privacy | content privacy |
| ---------------- | ------------- | ---------------- | --------------- |
| Bitcoin          | no            | partial (UTXO)   | no              |
| Zcash            | n/a (PoW)     | yes              | yes             |
| Praos            | no            | no               | no              |
| Crypsinous       | yes           | yes              | yes (with ZCash-like extension) |

### Properties

- *Stake-amount privacy.* Adversaries cannot infer per-
  stakeholder stake.
- *Authorship privacy.* Block producers are unidentifiable.
- *Same security guarantees as Praos.* CP / CG / CQ proven.

### Limitations

- *ZK overhead.* SNARK or ring-signature proofs are expensive
  (~10ms+ per proof, larger blocks).
- *Stake registration cost.* Anonymous registration requires
  more transactions than plain Praos.
- *Composability with public DEXs.* Stake-private chains are
  hard to integrate with public exchanges that require
  identity verification.

### Subsequent work

- *Aleo (snarkOS).* PoS chain with privacy-preserving
  smart contracts; uses Marlin SNARKs.
- *Privacy rollups.* Aztec, Polygon Hermez (now Polygon zk-EVM).
- *Mina (Samasika)* pairs anonymity with succinct proofs.

## Practice

Crypsinous itself was a research prototype; the privacy-PoS
ideas have been incorporated into:

- *Aleo (snarkOS, module 0149).* Privacy-preserving smart
  contracts and tokenisation.
- *Mina protocol* (with optional snapps).

### Production-implementation notes

- *Stake-set commitments.* Registered stake forms a Merkle
  accumulator; proofs of membership use Merkle paths.
- *VRF ZK predicate.* Eligibility-threshold predicate must be
  arithmetised for SNARK circuits.
- *Ring-signature size.* Linear in stake-set size; modern
  schemes (e.g., RingCT) can shrink to sub-linear.

## Verifiability and circuit encoding

**tag: `friendly`.**

Crypsinous is by design highly SNARK-friendly: every
participant's eligibility is already encoded in a ZK proof. A
SNARK light client for Crypsinous verifies the chain by
batch-verifying the per-block ZK proofs. Cost: ~10^5 to 10^6
constraints per block, depending on stake set size and
ring-signature scheme.

## Known attacks and limitations

- *Network-level deanonymisation.* IP traffic analysis can
  link block producers to physical machines; Crypsinous does
  not address this directly.
- *Stake-set size leaks.* Public total stake constrains
  privacy.
- *Posterior corruption.* Mitigated by forward-secure
  signatures, as Praos.

## References

- Kerber, Kiayias, Kohlweiss, Zikas, "Ouroboros Crypsinous:
  Privacy-Preserving Proof-of-Stake", IEEE S&P 2019.

## Implementation notes

The crate provides a `StakeCommitment` type and a `prove_eligibility`
predicate that combines a pseudo-VRF eligibility check with a
deterministic stake-set commitment. Tests verify correct
eligibility and rejection of forged proofs.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
