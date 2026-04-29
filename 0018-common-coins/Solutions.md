# Module 0018 Solutions

## Solution 1 [T]: threshold-BLS unbiasability

Given a Pedersen DKG with `t + 1` honest shares and EUF-CMA
threshold-BLS, the round-`r` coin is

```
coin_r := lowest_bit(H(BLS_combine(t + 1 shares of "coin-r")))
```

The combine function is deterministic given the same `t + 1`
shares; different `t + 1` subsets all yield the same combined
signature (this is the *uniqueness* of BLS aggregation modulo
the group).

So the coin is determined by the BLS scheme's secret key plus
"coin-r". Since the secret is unknown to the adversary
(EUF-CMA), and `H` is a random oracle, the bit is uniform.

## Solution 2 [T]: minimum-VRF biasability

Adversary controls `f` of `n - f` VRF outputs. To bias the
coin (lowest bit of minimum), the adversary chooses its `f`
outputs to minimise the global minimum. With `f` choices among
2^256 possible outputs, the adversary can shift the minimum by
at most `f` outputs.

For a uniformly distributed minimum, the bias is bounded by
`f / (n - f)`. With `n = 100, f = 33`, the bias is `33 / 67
~ 50%` worst case (a single bit flip in the minimum's lowest
bit).

This is why production VRF beacons hash the minimum with a
non-attacker-controlled value (e.g. hash of recent block).

## Solution 3 [T]: VDF uniqueness

A VDF `(Setup, Eval, Verify)` is *unique* if, for any `x`, only
one `y` satisfies `Verify(x, y, proof) = true`. This is the
property that prevents biasability: the adversary cannot
"choose" alternate outputs.

Production VDFs (Wesolowski 2018, Pietrzak 2019) achieve
uniqueness through repeated squaring in a group of unknown
order; the algebraic structure forces a unique result.

The load-bearing role: combined with the time-`T` lower bound
(adversary cannot precompute), uniqueness gives unbiasable
randomness.

## Solution 4 [P]: real threshold-BLS coin

```rust
struct CksWithRealCoin { /* ... */ accumulator: MockThresholdAccumulator, }

impl Process for CksWithRealCoin {
    fn on_tick(...) {
        // ... regular Vote ...
        let share = mock_sign(self.id, &round_bytes);
        // Send share alongside Vote
    }

    fn handle_share_message(...) {
        self.accumulator.add(share);
    }

    fn fetch_coin(...) -> bool {
        let digest = sha256(&round_bytes);
        let combined = self.accumulator.combine(&digest)
            .expect("threshold reached");
        combined.signers.iter().next().unwrap().0 & 1 == 1
    }
}
```

Latency impact: the coin requires `t + 1` partial-sig
deliveries before the protocol can advance, adding one
round-trip per round. For HoneyBadger's `n = 100`, this is
typically `~300 ms` per round.

## Solution 5 [P]: drand integration

```rust
async fn fetch_drand_round(round: u32) -> [u8; 32] {
    // Query drand HTTP API for the per-round signature
    let url = format!("https://api.drand.sh/public/{}", round);
    let resp = http::get(url).await?;
    let sig = parse_bls_signature(&resp.signature);
    sha256(&sig.serialize())
}

// In the protocol:
let beacon = fetch_drand_round(self.round).await;
let coin_input = sha256(&[beacon, &self.round.to_be_bytes()].concat());
let coin = coin_input[0] & 1 == 1;
```

Trust assumption: drand operators (`>= 2/3` honest). Latency:
30 seconds per beacon. Use case: long-block-time finality
gadgets where 30-second granularity is acceptable.

## Solution 6 [F]: pseudo-Lean typeclass

```text
class CommonCoin (n : Nat) where
  query : Round -> Bool
  honest_agreement :
    forall r, forall i j : Fin n,
    IsHonest i -> IsHonest j -> view i (query r) = view j (query r)
  unbiased :
    forall r b, Pr[query r = b] = 1/2

instance ThresholdBlsCoin (n t : Nat) (DKG : ...) (BLS : ...)
    [unforgeable BLS] [honest_dkg DKG]
    : CommonCoin n where
  query := ...
  honest_agreement := ...
  unbiased := ... -- via BLS.euf_cma + RO model
```

## Solution 7 [V]: in-circuit comparison

For `n = 100, f = 33`:

| Construction       | Per-coin constraints | Notes              |
| ------------------ | -------------------- | ------------------ |
| Threshold-BLS      | ~1.0 * 10^6          | one pairing        |
| Threshold-Schnorr  | ~700k                | n-f Lagrange + verifs |
| VRF (Pickles)      | ~340k                | (n-f) * 5k         |
| VDF (Wesolowski)   | ~200k                | one VDF proof      |

VDF is cheapest per coin in-circuit. Combined with its
unbiasability, this would be the optimal choice if the high
real-time latency were acceptable.

In practice, Mina uses a Pickles-VRF chain for in-protocol
randomness; Aleo uses Schnorr-style; Ethereum's roadmap
mentions VDF for unbiased L1 randomness.

## Solution 8 [V]: post-quantum readiness

| Primitive           | PQ status                    |
| ------------------- | ---------------------------- |
| Threshold-BLS       | not PQ-safe (BLS broken by Shor) |
| Threshold-Schnorr   | not PQ-safe (DL broken by Shor) |
| Hash-based VRF      | PQ-safe (depends on hash)    |
| Lattice-based VRF   | PQ-safe but large            |
| VDF (group of unknown order) | partial; some constructions PQ |
| Hash-chain VDF      | PQ-safe                      |

A PQ-ready common coin would use either a hash-chain VDF or a
lattice-based VRF. Both are higher-cost than current
production constructions but match the security of post-
quantum hash functions and lattice-based primitives. The
trade-off is bandwidth and latency.

The current frontier: Aleo and Mina explore PQ-friendly
SNARK substrates but stick with classical assumptions for the
common coin pending PQ standardisation (NIST PQC, in
finalisation as of 2024).
