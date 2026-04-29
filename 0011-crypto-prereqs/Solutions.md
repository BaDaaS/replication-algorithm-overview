# Module 0011 Solutions

## Solution 1 [T]: EUF-CMA and Schnorr-from-DL

EUF-CMA game:

```
Setup:    sk, pk <- KeyGen(1^lambda)
Adversary A:
  receives pk, plus oracle access to Sign(sk, *):
  outputs (m*, sigma*) such that
    Verify(pk, m*, sigma*) = 1
    AND m* never queried to oracle.
Wins if last condition holds.
```

A scheme is EUF-CMA secure if no PPT `A` wins with non-
negligible probability.

Schnorr signature: `sigma = (R, s)` where
`R = k * G, s = k + H(R, m) * sk`. Verify:
`s * G = R + H(R, m) * pk`.

Forking-lemma reduction (Pointcheval-Stern 2000): if `A` forges
with non-negligible probability, then by rewinding `A` to the
random oracle query for `H(R, m)` and re-sampling, we obtain
two valid signatures `(R, s_1)` and `(R, s_2)` with
`s_1 != s_2`. Subtracting:

```
(s_1 - s_2) * G = (H_1 - H_2) * pk
```

so `pk = (s_1 - s_2) / (H_1 - H_2) * G^{-1}`, which solves the
discrete-log instance for `pk`. Hence DL hardness implies
Schnorr EUF-CMA.

## Solution 2 [T]: BLS aggregation soundness

The same-message aggregation rule is `sigma_agg = sum sigma_i`,
verified against `pk_agg = sum pk_i` via
`e(sigma_agg, g_2) = e(H_to_G1(m), pk_agg)`.

Soundness reduction: if an adversary forges `(sigma_agg', pk_agg')`
on `m'` against an honest set of public keys, then by linearity
of pairings, the discrepancy reveals a co-CDH instance solution.
The reduction is BLS-Boneh-Lynn-Shacham 2003 + the multi-signer
extension of Boldyreva 2003.

Rogue-key attack: adversary registers `pk_adv = adv_sk * g_2 -
sum pk_honest`. Then `pk_agg = pk_adv + sum pk_honest = adv_sk
* g_2`, and the adversary can sign on the aggregate.
Mitigation: proof-of-possession (PoP) requires the registrant
to prove knowledge of `sk` corresponding to `pk` (a self-
signature on a domain-separated tag). Used in Eth2, Aleo,
Aptos.

## Solution 3 [T]: VRF uniqueness

A pseudo-random function has many valid outputs: any function
indistinguishable from random suffices. A VRF additionally
requires *uniqueness*: for each `(pk, x)`, exactly one `y` is
accepted. This is *not* implied by pseudo-randomness; it is a
separate property tied to the verifier's consistency check.

The standard construction (Micali-Rabin-Vadhan 1999):

- `y = e(H_G1(x), sk * g_2)` with proof `pi = sk * H_G1(x)`.
- Verify: `e(pi, g_2) = e(H_G1(x), pk)` and `y = e(pi, g_2)`.
- Uniqueness: if `(y, pi)` and `(y', pi')` both verify with
  `pi = pi'`, then `y = y'` by determinism of `e`. The
  forking-lemma argument shows that two distinct `(y, pi)`
  pairs verify only if the adversary has solved a discrete-log
  instance.

## Solution 4 [P]: Ed25519 replacement

`ed25519-dalek` (an approved dep) provides:

- `SigningKey::generate(rng)`, `VerifyingKey::from(sk)`.
- `sk.sign(msg) -> Signature`.
- `vk.verify(msg, &sig) -> Result<()>`.

Tests that pass plain bytes around (e.g. `mock_sign_verify_
roundtrip`) would still work with minor adjustments. Tests
that compare `MockSig` for equality by `(signer, digest)` would
need to switch to `(signer, signature_bytes)` since real
signatures are non-deterministic if randomness is randomised
(Ed25519 is deterministic by RFC 8032).

The deterministic-VRF-via-randomness-reuse pattern (Algorand)
needs RFC 6979-style derandomisation; Ed25519 is naturally
derandomised, simplifying the protocol.

## Solution 5 [P]: BLS Rust libraries

| Library    | Constant-time | Threshold APIs | SNARK-friendly |
| ---------- | ------------- | -------------- | -------------- |
| blst       | yes (audited) | minimal        | not directly   |
| bls12_381  | partial       | none           | partial (PLONK)|
| arkworks   | partial       | yes            | yes (multi)    |

`blst` is the production choice for raw signatures
(Eth2, Aptos use it). `arkworks` is the SNARK-development
choice (used in zkSync, Aztec, parts of Aleo).

## Solution 6 [F]: pseudo-Lean SignatureScheme

```text
class SignatureScheme where
  SK         : Type
  PK         : Type
  Sig        : Type
  KeyGen     : Nat -> SK × PK
  Sign       : SK -> List UInt8 -> Sig
  Verify     : PK -> List UInt8 -> Sig -> Bool
  correctness :
    forall (sk : SK) (pk : PK) (m : List UInt8),
    KeyGen lambda = (sk, pk) ->
    Verify pk m (Sign sk m) = true
  euf_cma :
    forall (A : Adversary)
           (h_pt : PolynomialTime A) (lambda : Nat),
    Pr[ A wins forgery game with sec param lambda ]
      <= 1 / (2 ^ lambda)
```

Mathlib infrastructure:

- `Mathlib.Probability.ProbabilityMassFunction` for PMFs over
  signature outputs.
- `Mathlib.Topology.Algebra.InfiniteSum.Basic` for the
  `negl(lambda) -> 0` quantifier.
- `Mathlib.NumberTheory.LucasLehmer` and elliptic-curve
  infrastructure for concrete instances.

## Solution 7 [V]: in-circuit threshold-BLS combine

For threshold `t + 1 = 67` over BLS12-381:

- *Lagrange interpolation in G_1.* Each partial sig
  `sigma_i = (sk_i * H(m))` contributes a Lagrange-weighted
  group element. Total: 67 scalar multiplications in `G_1`,
  `~10^4` constraints each = `~7 * 10^5` constraints.
- *Final verification.* One pairing on the combined sig:
  `~10^6` constraints.
- *Total.* `~1.7 * 10^6` constraints.

Compare to a non-threshold BLS aggregate: `~1.1 * 10^6`. The
threshold combine adds the Lagrange overhead but enables
asynchronous coin generation (HoneyBadger BFT etc.).

## Solution 8 [V]: SNARK-friendly hash benchmarks

| Hash         | Constraints / 2-input invocation |
| ------------ | -------------------------------- |
| Poseidon-2   | ~150-200                         |
| Rescue-Prime | ~270                             |
| Anemoi       | ~140                             |
| Griffin      | ~100                             |
| Reinforced Concrete | ~600 (different design space) |
| MiMC         | ~600                             |
| SHA-256      | ~30,000                          |
| Keccak-256   | ~150,000                         |

Sources: Poseidon (Grassi et al. USENIX 2021), Rescue-Prime
(Aly-Ashur-Ben-Sasson-Dhooghe-Szepieniec 2020), Anemoi
(Bouvier-Briaud-Chaidos-Perrin-Salen-Velichkov-Willems 2023),
Griffin (Grassi-Hao-Khovratovich-Lueftenegger-Rechberger
-Schofnegger 2022).

The cryptographic-engineering message: SNARK-friendly hashes
beat Keccak by 100-1000x in circuit. This is why every
production zkVM (Mina, Aleo, Aztec, Polygon zkEVM, Scroll)
uses Poseidon or a close cousin.
