# 0133: Threshold Cryptography in Circuit

## Historical context

For verifiable consensus, the threshold-signature primitives
underlying BFT must themselves be encoded in SNARK circuits.
This module surveys the constraint costs of common primitives
on common SNARK-friendly curves.

The key insight: BFT consensus produces a *threshold signature*
(2/3 of validators agreeing), and a SNARK verifier needs to
check this signature on-chain or in a proof. Some signature
schemes are dramatically cheaper than others to verify in a
circuit.

## Primitives covered

### BLS aggregate signatures

BLS (Boneh-Lynn-Shacham) over BLS12-381 is the standard for
modern BFT systems (Ethereum, Cosmos with BLS, Tezos).
Verification cost in a SNARK:

- Pairing check: ~10^4 to 10^6 constraints depending on
  proof system.
- Aggregate verification: same cost as one signature regardless
  of signer count.

### Schnorr signatures

Schnorr on a SNARK-friendly curve (Pasta, Vesta, Pallas):

- Single signature: ~3000 constraints (Pasta).
- N-of-N aggregate (FROST): ~3000 * N constraints if no
  aggregation in circuit.
- Threshold-Schnorr: requires distributed key generation
  (DKG); per-round share verification.

### Ed25519

Ed25519 on standard curves (Curve25519):

- Not SNARK-friendly. Curve operations require expensive
  field arithmetic emulation.
- Per signature: ~10^6+ constraints.
- Used by Bitcoin (no), Ethereum (transactions), Cosmos
  default, Algorand, Solana.

### ECDSA on secp256k1

Bitcoin and Ethereum address signatures:

- Per signature: ~10^6+ constraints (similar to Ed25519).
- Optimisations: Halo 2 + custom gates ~10^5 with care.

### VRFs (Verifiable Random Functions)

VRFs combine signatures with randomness extraction:

- Goldberg VRF on Ed25519: ~10^6+ constraints.
- BLS VRF (Boldyreva): ~10^4 to 10^5 on BLS12-381.

### Comparison: signature schemes in circuits

| scheme               | curve            | constraints/sig    |
| -------------------- | ---------------- | ------------------ |
| BLS aggregate        | BLS12-381        | ~10^4 to 10^6      |
| Schnorr              | Pasta            | ~3000              |
| Ed25519              | Curve25519       | ~10^6+             |
| ECDSA                | secp256k1        | ~10^6+             |
| BLS VRF              | BLS12-381        | ~10^4 to 10^5      |

### Cycle of curves

Mina's *Pasta cycle* (Pallas + Vesta) enables recursive
SNARKs: each curve's scalar field is the other's base field.
Schnorr signatures over Pasta are extremely cheap to verify
in a SNARK.

## Implementation notes

Different SNARK systems have different gate types and
constraint patterns. Constraint counts vary by:

- *Field size.* SNARK-friendly fields (BLS12-381 scalar field,
  Pasta) are much cheaper than arbitrary fields.
- *Custom gates.* PLONK with custom gates (e.g., ECC point
  doubling gates) can dramatically reduce cost.
- *Proof system.* Groth16 (smallest proofs), PLONK (universal
  setup), STARK (no setup but larger proofs).

## Verifiability and circuit encoding

**tag: `deployed`.**

Threshold cryptography is the bottleneck for verifiable
consensus circuits. The primary lesson:

- *BLS over SNARK-friendly curves* is the most efficient.
- *Schnorr over Pasta* is the cheapest single-signature option.
- *Ed25519 and ECDSA* require workarounds for SNARK
  verification.

## References

- Boneh, Lynn, Shacham, "Short Signatures from the Weil
  Pairing", JC 2004.
- Komlo, Goldberg, "FROST: Flexible Round-Optimized Schnorr
  Threshold Signatures", SAC 2020.
- Bonneau, Meckler, Rao, Shapiro, "Mina: Decentralized
  Cryptocurrency at Scale", 2020 (Pasta cycle).

## Implementation notes

The crate provides `circuit_cost` returning the approximate
constraint count for each primitive type. Tests verify
relative costs.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to 2024".
