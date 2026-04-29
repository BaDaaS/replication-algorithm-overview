# Module 0014 Solutions

## Solution 1 [T]: termination probability

In each round, after the Echo phase, the protocol's per-process
preference depends on the Echo distribution and the coin. Two
cases:

- *Some honest already decided.* Termination happens within two
  rounds (decision propagates).
- *No decision.* Each honest process's next-round preference is
  either a value carried from the Echo phase (if any) or the
  coin bit. With probability `>= 1/2`, the coin matches the
  *correct* value (the value that would lead to validity in the
  next round). When matched, validity-style termination occurs
  in the next round.

So `Pr[terminate by round r + 1 | not yet] >= 1/2`. Expected
rounds = `2 + 2 * 1/2 + 2 * (1/2)^2 + ... = 4`. Cachin-Kursawe-
Shoup 2000 give the tight constant.

## Solution 2 [T]: Byzantine validity

Honest inputs all equal `v`. Phase 1: every honest broadcasts
`Propose(v)`. Each honest collects `n - f >= 2f + 1` Proposes,
of which `>= 2f + 1` honest must equal `v` (since at most `f`
are Byzantine and they could send `1 - v`). The honest count
exceeds `2/3` of the collected Proposes (`(2f + 1) / (3f + 1) >
2/3` for `f >= 1`), so `majority(r) = Some(v)`.

Phase 2: every honest broadcasts `Echo(v)`. Each honest
collects `n - f` Echos with `>= 2f + 1` honest = `v`. Decide
`v`.

## Solution 3 [T]: Echo agreement

Suppose two distinct Echo majorities for `v` and `1 - v` both
existed in round `r`. Each majority is `> 2/3 (n - f) > 2/3
(2f + 1)` Echos. Together they need `> 4/3 (2f + 1)` Echos, but
the total is `n - f = 2f + 1`. Contradiction.

Equivalently, two `> 2/3`-quorums in `n = 3f + 1` intersect in
`> n / 3 + 1`-many honest, so they cannot disagree.

## Solution 4 [P]: real common coin

Replace `CoinFn` with:

```rust
pub trait CommonCoin {
    fn add_partial(&mut self, signer: NodeId, round: u32, share: ...);
    fn try_combine(&self, round: u32) -> Option<bool>;
}
```

Protocol changes:

- Each round, after Phase 2, each process sends a partial
  threshold-BLS signature on the round number.
- Collect `t + 1` partials; combine to get the threshold
  signature.
- Hash the threshold signature; the lowest bit is the coin.

The flow gains one phase per round: Propose, Echo, Coin-share,
Decide. Latency increases by one round-trip per round but the
coin is provably unbiasable (assuming threshold-BLS security).

## Solution 5 [P]: Pedersen DKG sketch

Phase 1 (commitment):
- Each `i` picks polynomials `f_i, g_i` of degree `t`.
- Broadcasts `(C_i := { g^{f_i(j)} h^{g_i(j)} : j = 0..t })`.

Phase 2 (share distribution):
- For each `j`, send `(s_{i,j} := f_i(j), t_{i,j} := g_i(j))`
  privately to `j`.

Phase 3 (complaint):
- Each `j` checks share consistency against `C_i`. Complaints
  are broadcast.
- A complained-against `i` either reveals the share (clearing
  the complaint) or is disqualified.

Phase 4 (extraction):
- Each `j` sums shares from non-disqualified contributors.
- Public key `pk = product g^{f_i(0)}` for non-disqualified.
- Private share for `j`: `sk_j = sum s_{i,j}`.

Costs: `O(n^3)` messages worst-case (all complain), `O(n^2)`
typical. Production: drand uses GJKR DKG with proactive
refresh.

## Solution 6 [F]: pseudo-Lean common coin

```text
class CommonCoin (n : Nat) where
  query : Nat -> Bool
  honest_agreement :
    forall (r : Nat), forall i j : Fin n,
      IsHonest i -> IsHonest j ->
      observe i (query r) = observe j (query r)
  unbiased :
    forall (r : Nat) (b : Bool),
      Pr[ query r = b ] = 1 / 2
  independent :
    forall (r1 r2 : Nat), r1 ≠ r2 ->
      Independent (query r1) (query r2)
```

Mathlib's `Probability.Independence` provides the
`Independent` predicate; `MeasureTheory.Measure.Probability`
gives the `Pr[...]` notation. The independence axiom is
non-trivial: it says successive coin queries are independent,
which is needed for the geometric-distribution termination
proof.

## Solution 7 [V]: zk-HoneyBadger circuit

HoneyBadger BFT uses `n` parallel Bracha RBs (one per
proposer) and `n` parallel ABAs. Per round:

- *RBs:* `n * 200k = 2 * 10^7` constraints (each Bracha RB ~`200k`
  constraints in Schnorr-over-Pasta).
- *ABAs:* `n * 1.4M = 1.4 * 10^8` constraints (each Rabin ABA
  ~`1.4M` per round, expected 4 rounds).
- *Coin:* shared per round, `~10^6` constraints.

Total per HoneyBadger round: `~1.5 * 10^8` constraints. With
recursion across rounds, constant-size proofs are feasible but
the per-round prover cost is significant. Aleo and Aztec
sequencer designs that adopt HoneyBadger-style ABA face this
budget directly.

## Solution 8 [V]: drand as a common coin

drand publishes a threshold-BLS beacon every 30 seconds across
~30 trusted operators. To use it as a BFT common coin:

- *Latency.* The 30-second beacon period dominates per-round
  latency. A BFT round becomes 30+ seconds bottlenecked on
  drand. This is acceptable for L1 finality gadgets but not
  for high-throughput sequencers.
- *Trust.* drand is operated by the League of Entropy (a
  consortium). The threshold model means a `2/3`-Byzantine
  drand still produces unbiased coins. The trust model is
  inherited.
- *Bandwidth.* Each beacon is ~96 bytes (BLS12-381 G1).
  Negligible.

Production: Filecoin uses drand as part of its Expected
Consensus randomness; some L2s and DAOs piggyback on drand.
Aleo, Aptos, and Sui do not use drand directly because their
in-protocol randomness suffices.
