# 0087: Selfish Mining

## Historical context

Ittay Eyal and Emin Gun Sirer published "Majority is Not Enough:
Bitcoin Mining is Vulnerable" at FC 2014. The paper showed that
a strategic miner with less than 50% hash power can earn a
disproportionate share of block rewards by *withholding* blocks
and revealing them strategically. This is now called the
*selfish mining* attack.

The discovery was significant for two reasons. First, it
contradicted Nakamoto's claim that an honest majority makes
Bitcoin secure: the protocol satisfies its safety properties
under honest-majority assumptions, but it does not satisfy
*incentive compatibility*. A self-interested rational miner
deviates from the honest protocol and profits.

Second, it foreshadowed a class of subsequent attacks (stubborn
mining, optimal selfish mining, intermittent selfish mining)
and motivated incentive-aware analyses for all blockchain
protocols.

## System and threat model

- **Network.** Synchronous, with a propagation parameter `gamma`
  representing the fraction of the honest network that hears
  the selfish miner's block first when there is a tie.
- **Failures.** A single rational miner with hash-power fraction
  `alpha`. The remaining `1 - alpha` is honest.
- **Cryptography.** Standard PoW.
- **Goal.** The selfish miner maximises their *long-run revenue
  share* (fraction of the canonical chain mined by the
  attacker), not their absolute block count.

## Theory

### Strategy: keep blocks private

Honest behaviour: when you find a block, broadcast it.
Selfish behaviour: when you find a block, keep it private and
extend it. Reveal blocks only when honest miners catch up.

The selfish miner maintains a private chain `P` and tracks the
public chain `H`. The strategy:

1. If `len(P) - len(H) >= 2`: keep extending privately.
2. If `len(P) - len(H) == 1` and a new public block matches:
   release the private block to compete (race condition).
3. If `len(P) - len(H) == 1` and the attacker finds another
   private block: keep both private (now ahead by 2).
4. If `len(P) - len(H) == 0` and the attacker is in a race:
   release the held block and try again on the head.

When the attacker releases their private chain, all previously
withheld blocks become canonical, and the public chain
discards their work. The honest miners' work was wasted; the
attacker's work was preserved.

### The threshold

Eyal-Sirer derive the revenue share as a function of `alpha`
and `gamma`:

```
R(alpha, gamma) = ( alpha * (1 - alpha)^2 * (4*alpha + gamma * (1 - 2*alpha))
                  - alpha^3 ) / ( 1 - alpha * (1 + (2 - alpha) * alpha) )
```

For `gamma = 0` (no propagation advantage), selfish mining is
profitable above `alpha > 1/3`. For `gamma = 1` (full
propagation control), profitable above `alpha > 0`.
Realistically, `gamma` near `0.5` and the threshold sits
near `alpha > ~25%`.

### Why this works

In honest mining, your revenue share equals your hash share:
`alpha`. In selfish mining, you cause the honest network to
waste work on chains that get orphaned. Since your private
chain is preserved, you keep all your blocks; honest miners
lose theirs in races. The net effect is a revenue share that
exceeds `alpha` for sufficiently large `alpha`.

The mathematical model is a Markov chain on the state
`d = len(P) - len(H)`; standard Markov-chain stationary-
distribution analysis gives the closed form above.

### Comparison with subsequent results

| paper                           | refinement                          | year |
| ------------------------------- | ----------------------------------- | ---- |
| Eyal-Sirer                      | initial selfish mining strategy     | 2014 |
| Sapirshtein-Sompolinsky-Zohar   | optimal selfish mining (MDP)        | 2016 |
| Negy-Rizun-Sirer                | intermittent selfish mining         | 2020 |
| Nayak-Kumar-Miller-Shi          | stubborn mining + eclipse synergy   | 2016 |
| Carlsten-Kalodner-Weinberg-Narayanan | undercutting attacks (post-subsidy) | 2016 |

These tighten the threshold (optimal selfish mining is
profitable above `alpha > 1/3` even with `gamma = 0`), combine
with eclipse attacks (lower threshold), or analyse different
incentive distortions.

### Selfish mining and other PoW chains

- *Bitcoin.* No confirmed instances at scale; pool dynamics
  rather than protocol rule discourage it.
- *Bitcoin Cash, Litecoin.* Same vulnerability.
- *Ethereum (pre-Merge).* GHOST mitigates but does not
  eliminate the attack; some analyses suggest GHOST has a
  slightly *higher* selfish-mining threshold (~25%, similar to
  Bitcoin) but throughput-related dynamics differ.
- *Monero, Zcash.* Same fundamental vulnerability.

## Practice

Selfish mining has not been observed at scale on Bitcoin
mainnet. Hypotheses for why:

1. *Reputation cost.* Major pools value their reputation; a
   detected selfish miner loses customers.
2. *Detection.* Selfish mining produces a distinctive
   orphan-rate signature; community monitoring (Eyal et al.
   themselves run analytics) deters it.
3. *Coordination cost.* Setting up a selfish mining
   infrastructure (private mempool, race-detection logic, fast
   propagation network) costs more than the marginal gain.
4. *Threshold not crossed.* Most pools sit below 25% hash share.

But selfish mining remains a known *risk*; it has been observed
on smaller chains (e.g., Vertcoin in 2018).

### Mitigations proposed

- *Eyal-Sirer's freshness preference.* Honest miners
  preferentially extend blocks heard *first* (fresher) rather
  than later, randomising the gamma parameter to ~0.5. Reduces
  the threshold to 25%.
- *Uniform tie-breaking.* Random choice between competing
  branches forces `gamma = 0.5`.
- *Publish-or-perish (Zhang-Preneel 2017).* Block must be
  published within 1 block-interval or be invalid.
- *Anti-collusion via PoW puzzles.* Force miners to commit
  publicly to which chain they mine.

None of these have been deployed in major chains; the
trade-offs (reduced fairness, increased orphan rate, deviation
from Nakamoto) have prevented adoption.

## Verifiability and circuit encoding

**tag: `na`.**

Selfish mining is an *attack analysis*, not a protocol; there is
no SNARK encoding to discuss. However, on-chain detection
mechanisms (orphan-rate monitors, block-arrival-time analyses)
could in principle be encoded in circuits to slash penalties on
PoS chains.

## Known attacks and limitations

- The selfish mining model assumes a single colluding attacker;
  multiple competing selfish miners is harder to analyse but
  appears worse for honest miners.
- Eyal-Sirer's analysis assumes constant difficulty; difficulty
  adjustments react to selfish mining and reduce the long-run
  benefit (but increase block-time variance).
- The `gamma` parameter is hard to estimate empirically; the
  attack threshold in practice is uncertain.

## References

- Eyal, Sirer, "Majority is Not Enough: Bitcoin Mining is
  Vulnerable", FC 2014.
- Sapirshtein, Sompolinsky, Zohar, "Optimal Selfish Mining
  Strategies in Bitcoin", FC 2016.
- Nayak, Kumar, Miller, Shi, "Stubborn Mining: Generalizing
  Selfish Mining and Combining with an Eclipse Attack", EuroS&P
  2016.
- Carlsten, Kalodner, Weinberg, Narayanan, "On the Instability
  of Bitcoin Without the Block Reward", CCS 2016.

## Implementation notes

The crate provides a closed-form revenue calculator
`selfish_share(alpha, gamma)` from Eyal-Sirer's formula. Tests
verify the function matches published values: `alpha = 0.33,
gamma = 0` is borderline-profitable, while `alpha = 0.4,
gamma = 0.5` is clearly profitable.

See also [`HISTORY.md`](../HISTORY.md), section "2009 to 2014".
