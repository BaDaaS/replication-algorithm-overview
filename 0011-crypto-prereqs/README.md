# 0011: Cryptographic Prerequisites

## Historical context

Consensus protocols rely on a small toolkit of cryptographic
primitives whose roles are remarkably stable across decades.
The primitives in this module appear in every later Part of the
course; understanding their security definitions, modelling
assumptions, and SNARK-friendliness up-front saves repetition
later.

The references span fifty years: Diffie-Hellman 1976 (key
exchange), Goldwasser-Micali-Rivest 1988 (digital signatures
formalised), Boneh-Lynn-Shacham 2001 (BLS), Boneh-Boyen 2004
(threshold BLS), Micali-Rabin-Vadhan 1999 (VRF), and the
SNARK-friendly hash family (Poseidon, Rescue) of the late 2010s.

## System and threat model

Each primitive comes with a security game: an adversary's
advantage in distinguishing or forging is bounded by a
*negligible function* in the security parameter. We give the
games at a level appropriate for graduate students familiar
with provable security.

## Theory

### Collision-resistant hash functions

A hash function `H : {0, 1}^* -> {0, 1}^lambda` is *collision
resistant* if, for any polynomial-time adversary `A`,

```
Pr[(x, x') <- A(1^lambda) : x != x' /\ H(x) = H(x')]
    is negligible.
```

Practical instantiations: SHA-256 (Bitcoin, Ethereum), Keccak-256
(Ethereum opcodes), Blake2 / Blake3, Poseidon (SNARK-friendly).

### Digital signatures

A digital signature scheme `(KeyGen, Sign, Verify)` is
*existentially unforgeable under chosen-message attacks*
(EUF-CMA) if no PPT adversary, with access to a signing oracle
on chosen messages, can produce a valid `(m, sigma)` pair on a
fresh `m` except with negligible probability.

Production schemes:

- *ECDSA* (Bitcoin, Ethereum L1). Secure but quirky: requires
  per-signature randomness that, if reused, leaks the private
  key.
- *Ed25519* (CometBFT, Cosmos, Sui). Deterministic, fast, no
  random reuse risk.
- *Schnorr* (Bitcoin Taproot, Mina). Linear, supports
  aggregation, SNARK-friendly when over a SNARK-native curve.
- *BLS* (Ethereum, Aleo, Aptos, Sui). Pairing-based, supports
  non-interactive aggregation, supports threshold variants.

### BLS aggregation

A BLS signature on a message `m` is `sigma = sk * H_to_G1(m)`,
where `H_to_G1 : {0, 1}^* -> G_1` is a hash to the BLS12-381
group `G_1`. Verification is `e(sigma, g_2) = e(H_to_G1(m), pk)`
for `pk = sk * g_2`. Aggregation:

- *Signature aggregation.* `sigma_agg = sum sigma_i`. Verifies
  with `e(sigma_agg, g_2) = product e(H_to_G1(m_i), pk_i)`.
- *Public-key aggregation.* If all signers signed the same `m`,
  `pk_agg = sum pk_i` and `e(sigma_agg, g_2) = e(H_to_G1(m),
  pk_agg)`: a single pairing.

The same-message case is the typical BFT setting (all
signatories vote on the same content), so verification reduces
to one pairing.

### Threshold signatures

A `(t, n)` threshold signature scheme allows any `t + 1` of `n`
signers to produce a signature; fewer than `t + 1` cannot. The
scheme has:

- A distributed key generation (DKG) phase producing shares
  `sk_1, ..., sk_n` and a public key `pk`.
- Partial-signing functions `partial_sign(sk_i, m) -> sigma_i`.
- A combine function that turns any `t + 1` partials into a
  full signature.

For BLS, the combination is Lagrange interpolation over the
shares; the resulting signature is identical in form to a
non-threshold BLS signature. Used in: HoneyBadger BFT, drand,
DFINITY's threshold randomness, Aptos's Quorum Store.

### Verifiable random functions

A VRF is a triple `(KeyGen, Eval, Verify)` such that:

- `Eval(sk, x)` returns `(y, pi)` where `y` is pseudo-random
  and `pi` is a proof.
- `Verify(pk, x, y, pi)` accepts iff `(y, pi) = Eval(sk, x)`.
- The *uniqueness* property: for each `pk` and `x`, there is
  exactly one `y` accepted.

VRFs are used for committee election (Algorand, Ouroboros
Praos), for leader election (Cardano, Mina), and for
cryptographic sortition. The Micali-Rabin-Vadhan 1999 scheme is
the canonical reference; production deployments use
Goldberg-Naor-Reyzin 2017 (GNR) or its variants.

### Common coins

A *common coin* is an oracle accessible to all processes that
returns the same uniformly random bit per query. It can be
realised:

- *From a threshold BLS scheme.* Each round, `t + 1` validators
  sign the round number; the resulting threshold signature, when
  hashed, gives a common coin. Used in HoneyBadger, Dumbo,
  drand.
- *From a randomness beacon.* drand publishes a per-second
  beacon over BLS12-381; consensus protocols can subscribe.
- *From a VDF.* Verifiable delay functions ensure that the coin
  is unbiased even by a strong adversary.

The common-coin abstraction is the FLP-escape mechanism for
asynchronous BFT (module 0070).

### SNARK-friendly variants

Standard hash functions and signatures are expensive in a
SNARK circuit. Substitutes used in production:

- *Poseidon, Rescue-Prime, Anemoi* for hashes. ~150-400
  constraints per invocation, vs ~30k for Keccak-256.
- *Schnorr over Pasta (Pallas/Vesta)* for signatures. ~3k
  constraints per signature, vs ~1.5M for ECDSA over secp256k1.
- *Pickles VRF* (Mina). Built from Schnorr + Poseidon.
- *In-circuit threshold BLS.* Requires pairing-friendly curve
  arithmetic; Mathlib coverage is partial; production uses
  pre-aggregated proofs.

The *circuit cost ratio* is the dominant decision factor for
verifiable replication. Mina, Aleo, and Pickles-based stacks
choose Pasta cycles + Poseidon for this reason.

## Practice

### Hash choice in production

| System         | Hash                    |
| -------------- | ----------------------- |
| Bitcoin        | SHA-256                 |
| Ethereum L1    | Keccak-256              |
| Cosmos         | SHA-256, Blake2         |
| Sui, Aptos     | SHA3-256, BLAKE3        |
| Mina, Aleo     | Poseidon                |
| Polygon zkEVM  | Poseidon, Keccak-256    |
| Aztec          | Pedersen, Poseidon      |
| Filecoin       | Blake2b, Poseidon       |

### Signature choice

| System           | Signature             |
| ---------------- | --------------------- |
| Bitcoin (legacy) | ECDSA secp256k1       |
| Bitcoin Taproot  | Schnorr secp256k1     |
| Ethereum L1      | ECDSA secp256k1       |
| Ethereum BC      | BLS BLS12-381         |
| CometBFT         | Ed25519               |
| Sui              | Ed25519, Secp256k1    |
| Aptos            | Ed25519, BLS BLS12-381|
| Mina             | Schnorr over Pasta    |
| Aleo             | Schnorr over BLS12-377|
| Cardano          | Ed25519               |

## Formalisation aspects

```text
class Hash (lambda : Nat) (H : {0,1}* -> {0,1}^lambda) where
  collision_resistant :
    forall (A : Adversary), Pr[A finds collision] < negl(lambda)

class SignatureScheme where
  KeyGen : SecParam -> SK * PK
  Sign   : SK -> Message -> Signature
  Verify : PK -> Message -> Signature -> Bool
  euf_cma :
    forall (A : Adversary with signing oracle),
    Pr[A wins forgery game] < negl(lambda)

class VRF where
  Eval   : SK -> X -> Y * Proof
  Verify : PK -> X -> Y -> Proof -> Bool
  uniqueness :
    forall pk x, exists ! y, exists pi,
      Verify pk x y pi = true
```

Mathlib contains:

- `Mathlib.Probability` for probability of adversary success.
- `Mathlib.NumberTheory` for modular arithmetic underlying ECDSA
  and Schnorr.
- `Mathlib.AlgebraicGeometry.EllipticCurve` for elliptic curves.
- Pairing-friendly groups are partial; `BLS12-381` is
  in-progress.

CSLib's role: a `Cryptography.Foundations` namespace would host
the abstract typeclasses; downstream protocols (PBFT, HotStuff,
Ouroboros) instantiate them.

## Verifiability and circuit encoding

**Tag: `friendly`** (this module is the toolkit).

Per primitive, in BLS12-381 / Pasta:

| Primitive               | Constraint cost       |
| ----------------------- | --------------------- |
| Poseidon hash           | ~200                  |
| Keccak-256 in circuit   | ~30,000               |
| Schnorr/Pasta sig verify| ~3,000                |
| Ed25519 sig verify      | ~10,000 (Pasta)       |
| ECDSA secp256k1 verify  | ~1,500,000            |
| BLS pairing             | ~1,000,000            |
| BLS aggregate (n = 100) | ~1,100,000 (one pair) |
| Pickles VRF eval        | ~5,000                |
| Threshold BLS combine   | ~500,000 (Lagrange)   |

These numbers determine which primitives a verifiable
replication algorithm can afford. Mina's choice of Schnorr +
Poseidon + Pickles VRF gives a per-slot proof of ~50k
constraints; zk-rollup sequencers (Aztec, Scroll) use Poseidon
+ BLS for ~1M constraints per batch.

## Known attacks and limitations

- *ECDSA reuse.* Reusing the same nonce across two signatures
  leaks the private key. Production systems mitigate via
  RFC 6979 deterministic nonces.
- *BLS rogue-key attacks.* Without proof-of-possession on
  registration, an adversary can compute a public key that
  cancels honest signatures in aggregates. Production fixes:
  PoP signatures (Eth2, Aleo) or proof-of-knowledge.
- *Threshold-BLS DKG.* The DKG phase is a complex
  multi-party protocol of its own. Production uses Pedersen DKG
  (Pedersen 1991) or Gennaro-Jarecki-Krawczyk-Rabin (GJKR 2007).
- *VRF biasability.* Some VRF constructions admit
  rejection-sampling biasing if the leader can refuse to publish
  unfavourable outputs. Algorand mitigates with seeded VRFs;
  Praos with chained VRFs.

## Implementation notes

The crate provides only what later modules need:

- A re-exported SHA-256 hash via the `sha2` crate.
- A `MockSignatureScheme` (HMAC-style) and a `MockVrf` for
  testing-only purposes. Real cryptography comes via the
  approved-deps list when a module needs it.
- A `MockThresholdSignature<f>` that records partial signatures
  and combines once `f + 1` are present (no real cryptography,
  just a counter).

These mocks are *pedagogical*: they let later modules' tests
demonstrate the protocol logic without committing to a specific
real cryptographic library.

## References

- Diffie and Hellman, "New Directions in Cryptography",
  IT 1976.
- Goldwasser, Micali, Rivest, "A Digital Signature Scheme Secure
  Against Adaptive Chosen-Message Attacks", SIAM Comp 1988.
- Boneh, Lynn, Shacham, "Short Signatures from the Weil
  Pairing", J. Cryptology 2004 (BLS).
- Boneh, Boyen, Shacham, "Threshold Signatures from the Weil
  Pairing", PKC 2003 (Threshold BLS).
- Micali, Rabin, Vadhan, "Verifiable Random Functions",
  FOCS 1999.
- Goldberg, Naor, Reyzin, "VRFs in the Random Oracle Model and
  Beyond", PKC 2017.
- Grassi, Khovratovich, Rechberger, Roy, Schofnegger,
  "Poseidon: A New Hash Function for Zero-Knowledge Proof
  Systems", USENIX 2021.

See also [`HISTORY.md`](../HISTORY.md), various sections.
