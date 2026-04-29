# 0018: Common-Coin Constructions

## Historical context

The common-coin abstraction underwrites every Rabin-style
randomised consensus protocol: Rabin 1983, CKS 2000, MMR
2014/2015, HoneyBadger, Dumbo, and many DAG-BFT protocols. The
question of *how to instantiate* the common coin has driven
its own line of research:

- *Threshold signatures.* CKS 2000 proposed threshold-RSA;
  modern deployments prefer threshold-BLS (Boneh-Boyen-Shacham
  2003) or threshold-Schnorr.
- *Verifiable random functions (VRFs).* Goldberg-Naor-Reyzin
  2017 give the canonical efficient construction. Used by
  Algorand and Ouroboros Praos.
- *Verifiable delay functions (VDFs).* Boneh-Bonneau-Bunz-Fisch
  2018 introduced unbiasable VDF-based randomness. Used by
  Filecoin and Ethereum's RANDAO+VDF proposals.
- *Randomness beacons.* drand (Cloudflare's League of Entropy)
  publishes a threshold-BLS beacon every 30 seconds.

Each construction has its own threat model, latency, and
SNARK profile. This module surveys them and gives a comparison
matrix usable by later protocol modules.

## System and threat model

Common to all constructions:

- **Goal.** Provide a per-round bit (or wider random output)
  that every honest process sees identically and that the
  adversary cannot bias.
- **Adversary.** Byzantine, can corrupt up to `f < n/3` parties
  (or different bound depending on construction).
- **Trust assumptions.** Vary by construction: DKG honesty for
  threshold sigs; honest VRF holders for VRF beacons; honest
  VDF evaluator (single party) for VDFs.

## Theory

### Construction 1: Threshold-BLS coin

Setup: DKG produces shares `sk_1, ..., sk_n` of a master `sk`,
with public key `pk` and threshold `t + 1`.

Per round `r`:

```
each i computes sigma_i := sk_i * H_to_G1("coin-r")
broadcasts sigma_i to all
on receiving t + 1 distinct sigma_i:
  combine via Lagrange to get sigma := sk * H_to_G1("coin-r")
  coin_r := first bit of H(sigma)
```

Properties:

- *Unbiasability.* Once `t + 1` honest contribute, the coin is
  determined; the adversary cannot bias.
- *Liveness.* Requires `t + 1 <= n - f`, i.e. `t < n - f`.
- *Cost.* One BLS pairing check + Lagrange interpolation.
- *Verifiable.* Yes; one pairing per coin.

### Construction 2: VRF-based coin

Each process has a VRF key pair `(sk_i, pk_i)`.

Per round `r`:

```
each i computes (y_i, pi_i) := VRF.Eval(sk_i, "coin-r")
broadcasts (y_i, pi_i) to all
collect (y_j, pi_j) from at least n - f distinct senders
verify each VRF proof
coin_r := some agreed function of {y_j} (e.g. minimum)
```

Properties:

- *Unbiasability under non-rejection.* The VRF is unique per
  input, so no party can choose `y_i`. But any party can
  *withhold* an unfavourable `y_i`; mitigation is the
  "minimum of `n - f`" rule, which is robust to up to `f`
  withholdings.
- *Liveness.* No DKG needed; each process has its own key.
- *Cost.* `n - f` VRF verifications per coin.
- *Verifiable.* Yes; verify each VRF separately.

### Construction 3: VDF-based coin

A *verifiable delay function* takes input `x` and computes
`y = f^T(x)` (`T` sequential squarings) with a proof. The
output `y` is unbiasable because `T` is large enough that the
adversary cannot precompute alternative outputs.

Per round `r`:

```
seed_r := H(coin_{r-1} or some external seed)
y_r := VDF.Eval(seed_r, T)
publish (y_r, proof)
verifier checks proof; coin_r := first bit of H(y_r)
```

Properties:

- *Unbiasability.* The VDF takes time `T` regardless of
  attacker resources; the seed is committed before evaluation.
- *Single evaluator.* Only one party need compute the VDF;
  others verify.
- *Cost.* High setup latency (`T` sequential steps).
- *Verifiable.* Yes; the VDF proof attests correct evaluation.

### Construction 4: drand beacon

drand operates a threshold-BLS group of ~30 nodes. Every 30
seconds, the group publishes a threshold-BLS signature on the
round number. Anyone can verify with the group's public key.

Properties:

- *Trust.* Threshold against the drand operators (`>= 2/3`
  honest).
- *Latency.* 30 seconds per beacon.
- *Public.* Anyone can subscribe; not specific to any
  consensus protocol.
- *Verifiable.* Yes; one BLS pairing.

### Comparison matrix

| Construction       | Setup    | Latency  | Trust          | Cost (verify)   |
| ------------------ | -------- | -------- | -------------- | --------------- |
| Threshold-BLS      | DKG      | low      | DKG honesty    | 1 pairing       |
| Threshold-Schnorr  | DKG      | low      | DKG honesty    | sig verification|
| VRF (per-process)  | none     | low      | per-VRF holder | n-f verifs      |
| VDF                | none     | high (T) | single evaluator| VDF proof      |
| drand              | external | 30s      | drand operators| 1 pairing       |

### Adaptive corruption

A subtler issue: under *adaptive* corruption, the adversary can
choose which validators to corrupt during the protocol's run,
including based on partial coin contributions.

- *Threshold-BLS.* Vulnerable: collecting `t` partial signs
  before corruption gives the adversary the threshold key.
- *VRF-based.* Robust: each process's VRF output depends on
  its own secret only.
- *VDF.* Robust: the VDF is computed by one party and verified
  by others; corruption mid-VDF does not bias the output.
- *Proactive refresh.* Rotating threshold-BLS keys regularly
  (e.g. every epoch) defeats adaptive corruption beyond the
  refresh interval. Used by Algorand, Ouroboros Praos.

## Practice

### Production use

| Coin source          | Used by                    |
| -------------------- | -------------------------- |
| Threshold-BLS        | HoneyBadger, Dumbo, DFINITY|
| VRF                  | Algorand, Ouroboros Praos, Cardano, Mina (chain seed) |
| VDF + RANDAO         | Ethereum (proposed)        |
| drand                | Filecoin, Internet Computer (RNG) |

### Setup overhead

Threshold-BLS DKG is the main cost: GJKR DKG is `O(n^3)`
messages, takes ~30 seconds for `n = 100`. Production tends to
amortise across long epochs.

VRF-based coins have no setup (each process self-generates a
key) but require coordination on which `n - f` outputs to use.

VDF-based coins require trusted hardware or known-honest
evaluator (e.g. a pool of multiple evaluators with redundancy).

### MEV and front-running

Common-coin choice intersects with MEV. A coin biased toward a
specific outcome can be exploited by validators with foreknowledge.

- *Threshold-BLS coins* are resistant: the threshold ensures no
  single party knows the output before publication.
- *VRF-based coins* leak the output to each holder before
  publication; mitigation is to hash the VRF output with a
  later-published random seed.
- *VDF-based coins* have a built-in delay: the output is not
  known until the VDF completes, after the inputs are
  committed.

## Formalisation aspects

```text
class CommonCoin (n : Nat) where
  query : Round -> Bool
  honest_agreement : ...
  unbiased : ...

class ThresholdBlsCoin (n t : Nat) extends CommonCoin n where
  setup_assumption : DkgHonestlyExecuted
  pairing_check : PairingFriendlyGroup G1 G2 GT

class VrfCoin (n : Nat) extends CommonCoin n where
  per_process_keys : forall i : Fin n, exists sk_i, pk_i
  combine_rule : List (Output × Proof) -> CoinBit
```

The four constructions are alternative *instances* of the
abstract `CommonCoin` typeclass. Protocol-level proofs use
the typeclass; deployment chooses an instance.

## Verifiability and circuit encoding

**Tag: `friendly` to `deployed`.**

Per construction, in-circuit verification cost:

| Construction       | Per-coin constraints |
| ------------------ | -------------------- |
| Threshold-BLS      | ~10^6 (1 pairing)    |
| Threshold-Schnorr  | ~700k (Lagrange + verifications) |
| VRF (n-f outputs)  | (n-f) * 5k (Pickles VRF) ~340k for n=100, f=33 |
| VDF (Wesolowski)   | ~200k                |

VDFs are the cheapest in-circuit per coin; trade-off: high
real-time latency. Mina uses Pickles VRFs; Aleo uses similar.
zk-rollup sequencers tend to use threshold-BLS or trusted
randomness.

## Known attacks and limitations

- *Adaptive corruption.* Threshold-BLS without proactive
  refresh is vulnerable; mitigations: ephemeral keys, VRFs.
- *Coin leakage.* VRF holders learn outputs early; mitigations:
  delay-based hashing, VDF post-processing.
- *Liveness vs unbiasability.* Threshold schemes need `>= t +
  1` honest contributors; if too many crash, the coin halts.
- *Beacon dependence.* Using drand creates an external
  dependency; production must handle drand outages.

## Implementation notes

The crate provides four `CoinFn` implementations as compile-
time constants for use by ABA modules (0014-0017):

- `threshold_bls_coin_round`: a closure that simulates the
  threshold-BLS coin; in tests, returns `H(round) mod 2`.
- `vrf_coin_round`: simulates the "minimum VRF" rule; returns
  the lowest bit of `H(node || round) mod 2`.
- `vdf_coin_round`: simulates a VDF-derived coin; same shape.
- `drand_coin_round`: simulates a drand-style external coin.

Tests verify each function returns a deterministic per-round
value.

## References

- Boneh, Boyen, Shacham, "Threshold Signatures from the Weil
  Pairing", PKC 2003.
- Goldberg, Naor, Reyzin, "VRFs in the Random Oracle Model and
  Beyond", PKC 2017.
- Boneh, Bonneau, Bunz, Fisch, "Verifiable Delay Functions",
  CRYPTO 2018.
- League of Entropy, "drand: A Distributed Randomness Beacon",
  drand.love.

See also [`HISTORY.md`](../HISTORY.md), section "2015 to 2019"
and "2020 to 2023".
