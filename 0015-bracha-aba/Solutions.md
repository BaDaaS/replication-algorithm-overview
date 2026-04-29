# Module 0015 Solutions

## Solution 1 [T]: Bracha 1987 termination

After step 1 (Vote via RB), every honest process sees the same
set of votes. After step 2 (Aux via RB), every honest process
sees the same `S_r`. Three cases:

- *|S_r| = 1, S_r = {v}.* All honest set `p_{i+1} = v`. With
  probability `1/2`, the coin equals `v`, and all honest decide.
- *|S_r| = 2, S_r = {0, 1}.* All honest set `p_{i+1} = coin`.
  In the next round, all honest votes are `coin`, validity
  triggers; decide in two more rounds.
- *|S_r| = 0.* All honest set `p_{i+1} = coin`. Same as above.

Expected rounds = `1 + 1/2 * 2 = 2`, plus the initial round, so
`<= 4`. Bracha 1987 gives the tight constant.

## Solution 2 [T]: RB necessity

Without RB, a Byzantine sender can broadcast `Vote(0)` to half
the honest processes and `Vote(1)` to the other half. Each
honest sees a different vote distribution; their computed
`majority(r)` may differ. The agreement-of-S_r property fails.

Bracha RB is what guarantees that all honest processes see the
*same* multiset of votes, regardless of Byzantine
equivocation. Production protocols use either Bracha RB
explicitly (HoneyBadger, Aleph) or signed broadcasts that play
the same role (CometBFT, HotStuff).

## Solution 3 [T]: Bracha vs Rabin

| Property               | Bracha 1987 | Rabin 1983   |
| ---------------------- | ----------- | ------------ |
| Voting layer           | Bracha RB   | direct       |
| Decision rule          | S_r vs coin | Echo majority |
| Coin role              | bias and progress | progress only |
| Auxiliary set          | yes (S_r)   | no           |
| Round complexity       | `<= 4`      | `<= 4`       |

The structural difference is the auxiliary set `S_r`, which
serves as a *certified* set of values that some honest process
saw as a majority. Rabin's protocol packs this information
into the Echo phase implicitly. CKS 2000 (module 0016) shows
the two are equivalent up to constants.

## Solution 4 [P]: RB at the wire layer

Adapt module 0009's `BrachaNode<M>` for `M = Vote(r, v)`. Each
ABA Vote becomes a Bracha-RB-broadcast. Message count per
round: `O(n^2)` for the Vote phase (Bracha RB is quadratic),
`O(n^2)` for the Aux phase, `O(n)` for the coin. Total `O(n^2)`
per round. For `n = 100`, ~`10^4` messages per round.

The simplification of using all-to-all + `NoOpAdversary` in
the test crate hides this constant factor; production
implementations use explicit RB.

## Solution 5 [P]: parallel ABAs

`n` parallel Bracha ABAs, one per proposer. Throughput: ~`n`
proposals decided per round (each ABA decides its own
proposal). Latency: same as one ABA (`O(1)` rounds).

HoneyBadger BFT composes:

- *Validated dispersal* (AVID, an RB variant) to disseminate
  large transaction batches.
- *`n` parallel Bracha ABAs* to decide which batches to
  include in the final block.
- *Threshold encryption* to hide transaction content from
  validators until ABA decides.

The result: a fully asynchronous BFT with throughput
proportional to network bandwidth, not consensus latency.

## Solution 6 [F]: pseudo-Lean reduction

```text
theorem aba_from_rb_and_coin
    (n f : Nat) (h : 3 * f < n)
    (RB : ReliableBroadcast Vote)
    (CC : CommonCoin) :
    AsyncByzAgreement n f Bool := by
  -- Construct an ABA whose voting and aux phases use RB and
  -- whose coin uses CC.
  refine {
    validity := ...,
    agreement := ...,
    termination := ...
  }
  -- Validity: prove via RB delivery agreement.
  -- Agreement: prove via S_r uniqueness across honest views.
  -- Termination: prove via geometric distribution on coin
  -- matches.
  sorry
```

## Solution 7 [V]: zk Bracha-ABA per round

For `n = 100, f = 33`, per round:

- *RB-Vote.* `2f + 1 = 67` Bracha-ready certificates, each ~3k
  constraints. ~`200k` constraints.
- *RB-Aux.* same, ~`200k`.
- *Common coin (threshold-BLS).* one pairing, `~10^6`.
- *Decision rule.* Set `S_r`, comparison with coin: ~100.

Total per round: `~1.4 * 10^6` constraints. Constant proof
with recursion across rounds.

## Solution 8 [V]: HoneyBadger aggregation

HoneyBadger has `n` parallel ABAs per epoch. Without
aggregation, the proof has `n * 1.4M = 1.4 * 10^8` constraints
per epoch. With recursive aggregation:

- Each ABA produces a small SNARK certificate (~constant).
- A "fold" recursion combines `n` certificates into one
  (Mariposa, Nova, ProtoStar style).
- The final proof is `O(1)` size.

The prover cost is still `O(n)` per epoch; the verifier cost
is constant. This is the structure of zk-HoneyBadger and
similar designs.
