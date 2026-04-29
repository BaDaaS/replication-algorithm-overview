# 0093: FruitChains

## Historical context

Rafael Pass and Elaine Shi published "FruitChains: A Fair
Blockchain" at PODC 2017. The paper addresses a fundamental
problem with Bitcoin and similar PoW chains: they are not
*fair*. A miner with hash fraction `alpha` should earn
approximately `alpha` of the rewards, but Eyal-Sirer (2014;
module 0087) showed selfish mining lets an adversary earn more
than `alpha`, and small miners are systematically disadvantaged
by orphan-block races.

FruitChains introduces a new block structure (*fruits* attached
to blocks) such that each fruit's inclusion probability is
proportional to its miner's hash power, regardless of which
chain wins. This makes mining *epsilon-incentive-compatible*:
no strategy earns significantly more than honest mining.

## System and threat model

- **Network.** Bounded delay (PSS-style).
- **Failures.** Computational majority of honest hashing power;
  Byzantine adversary.
- **Cryptography.** Two PoW puzzles per round (one for the
  block, one for fruit), both based on SHA-256.
- **Goal.** `epsilon`-fair distribution of block rewards
  proportional to hash share.

## Theory

### Fruits and blocks

FruitChains uses two PoW puzzles:

- *Block PoW.* As in Bitcoin: lower target, ~10 min interval.
  Determines chain ordering.
- *Fruit PoW.* Higher target, much more frequent (~10 sec).
  Each fruit references a recent block as its "stem".

A fruit is *recorded* when its stem is on the canonical chain
and the fruit's hash is included in any block within `kappa`
blocks of the stem. Reward goes to the fruit miner, not the
block miner.

### Why this is fair

Each unit of hashing power produces fruits at a rate
proportional to its share, independent of timing or selfish
mining strategy. Specifically:

- A fruit's inclusion only requires *any* honest block within
  `kappa` blocks of its stem, not a specific chain winner.
- Fruits cannot be censored without violating the chain quality
  property of the underlying backbone.

Pass-Shi prove: under the GKL model with honest hashing
fraction `alpha > 1/2`, every honest miner earns at least
`alpha * (1 - epsilon)` reward share for an arbitrarily small
`epsilon`.

### Theorem (Pass-Shi 2017, informal)

The FruitChains protocol is `epsilon`-fair: no mining strategy
earns more than `alpha + epsilon` reward share, where `alpha`
is the strategy's hash share. Selfish mining is no longer
profitable.

### Comparison: fairness in PoW

| protocol         | fair to small miners? | selfish-mining resistant? | block reward type |
| ---------------- | --------------------- | -------------------------- | ----------------- |
| Bitcoin          | partially (orphan losses) | no                     | block-only        |
| Ethereum (uncle) | better (uncle reward) | partially                  | block + uncle     |
| Bitcoin-NG       | partially             | no                         | key + microblock  |
| FruitChains      | yes (`epsilon`-fair)  | yes (`epsilon`-fair)       | fruit-based       |

### Implementation cost

The fruit puzzle increases miner CPU cost (one extra
hashing target per round) and the block size (fruits stored as
appended data). Pass-Shi argue the overhead is small because
fruit verification is cheap and miners can compute fruit hashes
in parallel with block hashes.

### Subsequent influence

- *FruitChains-style fairness.* Adopted in some PoS designs
  via attestation/checkpoint rewards (e.g., Casper FFG, module
  0123).
- *Ouroboros endorsement* (Praos and later). Slot leaders earn
  stake-proportional rewards; "endorsements" by other stake
  holders help fairness.
- *Fair leader election.* Pass-Shi 2017's framework underlies
  more recent fair-leader-election protocols (e.g., Chen-Micali
  Algorand 2019).

## Practice

FruitChains itself has not been deployed in production. Its
ideas have been adapted in:

- *Conflux (module 0099).* Tree-graph protocol with parallel
  block inclusion.
- *Prism (module 0098).* Voting blocks separate from
  transaction blocks (similar to fruits separate from blocks).
- *Cardano Praos.* Endorsement and stake-pool-based rewards.

### Production-implementation notes

- Fruit-target calibration: too high produces too many fruits
  (bandwidth); too low gives small-miner unfairness. Pass-Shi
  recommend ~100 fruits per block.
- `kappa` (inclusion-window) tuning: too small makes fruits
  expire often; too large lets adversarial selection skew
  rewards.
- Reward calculation: fruit rewards must be computed from the
  canonical chain post-confirmation, complicating SPV-style
  light clients.

### Mining algorithm (proof-of-work function)

FruitChains uses *two* PoW puzzles per round, both based on
double-SHA-256 (Bitcoin's substrate):

- *Block puzzle.* Same target as Bitcoin (~2^32 expected hash
  attempts, ~10-minute interval). Determines chain ordering.
- *Fruit puzzle.* Higher target by a factor of `~100`
  (~10-second expected interval). Each fruit references a
  recent block as its stem.

A miner mines both simultaneously: each hash attempt produces
a single output that may satisfy the fruit target (more
common), the block target (rarer), or neither. This shared
hashing means honest miners pay no extra computational cost
for the dual-puzzle structure beyond the fruit's reduced
difficulty.

Pass-Shi do not propose a custom hash function; any
collision-resistant hash modelled as a random oracle suffices.
SHA-256 is the natural default given FruitChains's positioning
as a Bitcoin enhancement.

## Verifiability and circuit encoding

**tag: `partial`.**

FruitChains circuits encode block and fruit puzzles. Both are
SHA-256-based, so per-round cost ~2x Bitcoin. The
inclusion-window mechanism adds a Merkle-membership proof per
fruit. Total cost: ~60-80k constraints per fruit + the standard
block PoW.

## Known attacks and limitations

- *Eclipse attacks.* Still possible (adversary isolates honest
  miner); FruitChains does not address network-level
  adversaries.
- *Bandwidth amplification.* Many small fruits per block
  increase network load.
- *Selfish fruit-mining.* Theoretical possibility of
  withholding fruits, mitigated by the inclusion-window
  mechanism.

## References

- Pass, Shi, "FruitChains: A Fair Blockchain", PODC 2017.

## Implementation notes

The crate provides `Block` and `Fruit` types with `Block`
referencing fruits via their hashes, and a function
`is_recorded` that checks whether a fruit's stem is within the
inclusion window of a given block. Tests verify the inclusion
predicate.

See also [`HISTORY.md`](../HISTORY.md), section "2014 to 2017".
