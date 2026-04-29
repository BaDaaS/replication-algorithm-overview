# Module 0013 Solutions

## Solution 1 [T]: agreement

Suppose `i` decides `v` in round `r`. Then `i` saw `>= f + 1`
Phase2 messages with value `v` in round `r`. Among those, at
least one is from an honest process, say `j`. `j` broadcast
`(Phase2, r, v)` because in step 1 of round `r`, `j` saw
`>= n / 2 + 1` Phase1 messages with value `v`.

Now consider any honest process `k`. In round `r`, `k` waits
for `n - f` Phase1 messages. Of these, at least
`(n - f) - (n / 2 - 1) >= n / 2 - f + 1 >= 1` overlap with
`j`'s observation. By the majority rule, `k`'s `bias` is either
`Some(v)` or `None`; it cannot be `Some(1 - v)` because the
opposite-value count is too low.

So no honest process broadcasts `(Phase2, r, 1 - v)`. Hence no
honest process can decide `1 - v` in round `r`.

For round `r + 1`, every honest process's preference is either
`v` (carried by Phase2 vote in round `r`) or set by a coin flip
that is independent of value. Inductively, no honest process can
ever decide `1 - v` after deciding `v`.

## Solution 2 [T]: validity

If all honest inputs are `v`, in round 1:

- Every honest process broadcasts `(Phase1, 1, v)`.
- By the time any honest process collects `n - f` Phase1
  messages, the count of `v` is at least `n - 2f` (from honest
  processes alone), which is `> n / 2` for `f < n / 2`. So
  `bias = Some(v)`.
- Every honest process broadcasts `(Phase2, 1, v)`.
- By the time any honest process collects `n - f` Phase2
  messages, the count of `v` is at least `n - 2f >= f + 1`. So
  the process decides `v`.

QED.

## Solution 3 [T]: termination probability

In round `r`, suppose the honest processes' preferences are
mixed (otherwise validity would have terminated). After Phase 2,
processes that did not see a Phase2 majority flip a coin. The
probability that all flipping processes get the same outcome is
`2 * (1/2)^{n - f}`, which is at least `2^{-(n - f) + 1}`.

If the coin flips agree (and Phase 2 votes did not force a
majority), every honest process enters round `r + 1` with the
same preference; round `r + 1` then satisfies the validity
hypothesis and terminates in step 2.

So `Pr[terminate in round r + 1 | not yet terminated]
>= 2^{-(n - f) + 1}`. Expected rounds-to-termination is
`<= 2^{n - f - 1}`.

## Solution 4 [P]: property test

```rust
proptest! {
    #[test]
    fn ben_or_terminates_with_high_probability(
        seed in 0u64..1000,
        b0 in any::<bool>(),
        b1 in any::<bool>(),
        b2 in any::<bool>(),
        b3 in any::<bool>(),
    ) {
        let mut sched = build_with_seed(seed, &[b0, b1, b2, b3]);
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 100_000).unwrap();
        for id in 0..4 {
            prop_assert!(sched.node(NodeId(id)).unwrap().decision.is_some());
        }
    }
}
```

The property holds with overwhelming probability. Failures
correspond to the seed picking a long unfavourable coin
sequence; with 100k step bound and `f = 1`, this is exponentially
unlikely.

## Solution 5 [P]: common-coin variant

Replace the local coin with a common coin derived from a
threshold-BLS signature on the round number. With a common
coin, all honest processes flip the same bit, so termination
occurs in `O(1)` expected rounds (in fact, with probability
1/2 per round, expected 2 rounds).

The drop from `O(2^n)` to `O(1)` is the load-bearing
improvement of HoneyBadger BFT and Dumbo.

## Solution 6 [F]: termination probability in pseudo-Lean

```text
theorem ben_or_termination_probability
    (n f : Nat) (h : 2 * f < n) (r : Nat) :
    Pr[ Decided BenOr by round r ] >=
      1 - (1 - 2^{-(n - f)})^r := by
  -- Geometric distribution argument: each round terminates
  -- with probability >= 2^{-(n - f)}; the cumulative
  -- probability of NOT terminating by round r is
  -- (1 - p)^r.
  sorry
```

Mathlib infrastructure:

- `Probability.Martingale.Convergence` for the limit `r ->
  infinity, Pr -> 1`.
- `Filter.Eventually.atTop` for "eventually decided".
- `MeasureTheory.MeasurableSpace` to define the protocol's
  output as a random variable.

## Solution 7 [V]: verifiable Ben-Or in circuit

Per round (with threshold-BLS coin):

- *Phase 1.* Verify `n - f` signatures and count. `(n - f) *
  3k = 67 * 3k = ~200k` constraints (Schnorr-over-Pasta).
- *Phase 2.* Same.
- *Coin.* One threshold-BLS pairing check: ~`10^6`.
- *Decision.* Comparison constraints: ~100.

Total per round: `~1.4 * 10^6` constraints. With recursion,
the proof is constant-size across many rounds.

## Solution 8 [V]: VRF-based common coin

Each process `i` computes `y_i = VRF(sk_i, round)` and
publishes `y_i` with proof. The *common* coin is some agreed
function of all the `y_i`'s, e.g. the minimum or the XOR of
the lowest bits.

Trade-offs:

- *Latency.* Need to wait for `n - f` VRF outputs before
  computing the coin. Threshold-BLS combines into one
  signature; VRF needs explicit collection.
- *Biasability.* A leader can refuse to publish an
  unfavourable VRF output. Threshold-BLS is unbiasable as long
  as the threshold is reached. VRFs require additional
  rejection-resistance (Algorand's seed chaining).
- *Circuit cost.* VRF verification is `~5k` constraints in
  Pickles per VRF, so verifying `n - f = 67` VRFs is `~335k`
  constraints, comparable to threshold-BLS but without the
  pairing.

Production: Algorand uses VRFs (with biasability mitigations);
HoneyBadger and Dumbo use threshold-BLS.
