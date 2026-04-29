# Module 0016 Solutions

## Solution 1 [T]: CKS termination

In each round, the protocol's preference is either:

- *S_r = {v}.* All honest set `p_{i+1} = v`. If `>= 2f + 1`
  Auxes report `v`, decide. Else, the next round's votes are
  all `v`, validity triggers, decide in two more rounds.
- *S_r = {0, 1} or empty.* All honest set `p_{i+1} = coin(r)`.
  By unbiasability, `coin(r)` is uniformly random. With
  probability `1/2`, the coin matches the value that would
  have led to validity in the next round. Decide in two more
  rounds.

So `Pr[terminate within next 2 rounds] >= 1/2`. Expected
rounds `<= 4`.

## Solution 2 [T]: RO-model dependence

The threshold-RSA coin construction in CKS 2000:

- Each validator computes a partial signature on the round
  number `r`.
- `t + 1` partials are combined into a full signature.
- The signature is hashed to derive the coin bit.

The hash step (`H(threshold_signature) -> coin bit`) is the
source of the random-oracle assumption. The proof of
unbiasability requires that the hash output is uniformly
distributed regardless of the input, which is exactly the
random-oracle assumption.

Production: SHA-256 is conjectured to behave as a random
oracle in this context; modern proofs in the standard model
(without RO) require additional structural assumptions on the
underlying group.

## Solution 3 [T]: CKS to HoneyBadger

CKS 2000 is a single ABA instance. HoneyBadger 2016 adds:

- *Parallelism.* `n` ABA instances per round, one per
  proposer. The agreed set of proposers' values is the block.
- *Threshold encryption.* Transactions are encrypted with a
  threshold-public key; only after ABA decides on the block
  do validators decrypt. Prevents validator-side front-
  running.
- *AVID.* Asynchronous verifiable information dispersal for
  large transaction batches; each proposer disseminates its
  batch via AVID rather than direct broadcast.

What stays unchanged: the per-instance ABA structure, the
common-coin layer, the threshold-cryptography primitives.

## Solution 4 [P]: threshold-BLS coin

Replace threshold-RSA with threshold-BLS:

- *Setup.* DKG produces shares of a BLS secret key `sk`. Each
  validator gets `sk_i`.
- *Partial signing.* Each validator computes
  `sigma_i := sk_i * H_to_G1(r)` for round `r`.
- *Combining.* `t + 1` partials are combined via Lagrange
  interpolation: `sigma := sum_{i in S} L_{S, i}(0) * sigma_i`.
- *Coin extraction.* The lowest bit of `H(sigma)` is the
  coin.

Modern deployments (drand, HoneyBadger production variants)
use threshold-BLS over BLS12-381 or BLS12-377.

## Solution 5 [P]: real coin in simulator

```rust
struct CksWithRealCoin {
    accumulator: MockThresholdAccumulator,
    // ... other fields
}

impl CksWithRealCoin {
    fn on_vote(...) {
        let share = mock_sign(self.id, &round_bytes);
        ctx.send(self.id, peer, Msg::Vote { ..., share });
    }

    fn handle_share(&mut self, share: MockSig) {
        self.accumulator.add(share);
        if let Some(combined) = self.accumulator.combine(&round_digest) {
            // coin = first bit of combined.signers' hash
        }
    }
}
```

Termination remains expected `O(1)` rounds because the coin's
unbiasability is equivalent to the underlying threshold
scheme's security.

## Solution 6 [F]: RO-model in Lean

```text
class RandomOracle (Inp Out : Type) where
  query : Inp -> Out
  uniformity :
    forall (queries : List Inp) (i : Inp),
      i ∉ queries ->
      Pr[query i = ?] is uniform on Out

theorem cks_termination_in_ro_model
    [RO : RandomOracle (Round) Bit]
    [TS : ThresholdSig (with RO-based hash)] :
    forall sched, Pr[terminate within 4 rounds] >= 1/2 := by
  sorry
```

Mathlib's `Probability.IndepFun` provides independence;
`MeasureTheory.Measure.Probability` gives the `Pr[]`
notation. The challenge is encoding the "lazy" RO query
semantics (the oracle answers consistently across queries).

## Solution 7 [V]: zk-CKS circuit

For `n = 100, f = 33`, per round:

- Vote phase: verify `n - f = 67` signatures, count by value:
  `~200k` constraints.
- Aux phase: same.
- Coin combine: Lagrange interpolation on `t + 1 = 34` shares
  + pairing check: `~700k`.
- Decision rule: comparisons, ~100 constraints.

Total per round: `~1.1 * 10^6` constraints. Slightly less than
Bracha-with-separate-coin (module 0015) because the coin shares
are piggybacked on Vote messages, eliminating an extra phase.

## Solution 8 [V]: aggregating coin shares

The "any `t + 1` of `n - f`" combiner: given a multiset of
partials, pick any `t + 1` distinct signers and Lagrange-
interpolate.

In circuit:

- The witness is the indicator vector for the chosen `t + 1`
  signers (a length-`(n - f)` 0/1 vector with `t + 1` ones).
- Constraints: verify the vector has popcount `t + 1`
  (~`(n - f)` constraints), and for each chosen signer,
  multiply the partial by the precomputed Lagrange coefficient
  (one EC scalar mul per chosen signer, `~10k` constraints
  each, ~`340k` total for `t + 1 = 34`).

Total: `~340k` constraints for the combiner. The pairing
verification is separate (~`10^6`). The combiner constraints
are dominated by the EC scalar multiplications.

The "any `t + 1`" choice means the prover can select the
combination that minimises witness work, but the
verification cost is fixed.
