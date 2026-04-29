# 0091: Solida

## Historical context

Ittai Abraham, Dahlia Malkhi, Kartik Nayak, Ling Ren, and
Alexander Spiegelman published "Solidus: An Incentive-compatible
Cryptocurrency Based on Permissionless Byzantine Consensus" at
OPODIS 2017 (preprint 2016, often called *Solida*). It is a
contemporary of ByzCoin (module 0090) addressing the same
question (combine PoW for committee selection with BFT for
finality) but with stronger formal guarantees and a more
careful analysis of incentive compatibility.

The key contribution: under a Byzantine + sleepy adversary,
Solida achieves *responsive* termination (latency depends only
on actual network delay, not a known bound `Delta`) within the
committee, while only paying PoW costs for committee rotation
(slow path).

## System and threat model

- **Network.** Synchronous PoW outer loop; partially synchronous
  inside committee.
- **Failures.** PoW honest-majority globally; standard
  `f < n/3` Byzantine inside committee.
- **Cryptography.** SHA-256 PoW for membership; threshold
  signatures (BLS or Schnorr) for BFT.
- **Goal.** Permissionless cryptocurrency with deterministic
  finality, responsiveness inside committee, and
  incentive-compatible block rewards.

## Theory

### Two-tier protocol

Solida operates at two timescales:

1. *Slow PoW tier.* New miners earn committee membership by
   producing PoW blocks. Each PoW block rotates the committee.
2. *Fast BFT tier.* The committee runs a Byzantine-fault-tolerant
   protocol on transactions, producing committed transaction
   blocks at network speed.

This is a similar architecture to ByzCoin, but Solida specifies
the BFT protocol more carefully: a synchronous-tier protocol
with responsiveness inside the committee.

### Committee rotation

Like ByzCoin, the committee is a sliding window of the last `c`
PoW miners. Solida adds:

- *Reconfiguration via BFT.* Joining/leaving the committee is
  itself a BFT-committed operation, not a unilateral takeover.
- *Block rewards split.* PoW block reward is split between the
  miner and the committee, incentivising committee participation.
- *Slashing.* Misbehaving committee members lose future block
  rewards.

### Responsiveness

Solida's BFT layer is *responsive*: in the absence of failures
or with `<= f` failures, latency depends on actual message
delay `delta` (much less than the synchronous bound `Delta`).
This was a novel contribution at the time; later HotStuff
(module 0056) generalised the approach.

### Comparison with ByzCoin and Hybrid Consensus

| property            | ByzCoin       | Solida        | Hybrid Consensus |
| ------------------- | ------------- | ------------- | ---------------- |
| committee selection | last `w` PoW  | last `c` PoW  | rotating PoW     |
| BFT inside          | PBFT + CoSi   | sync BFT      | PBFT-style       |
| responsive (BFT)    | partially     | yes           | partially        |
| reconfiguration     | implicit      | BFT-committed | BFT-committed    |
| slashing            | none          | reward loss   | none             |
| analysis            | empirical     | formal        | formal           |
| year                | 2016          | 2016/17       | 2017             |

Solida's main contribution over ByzCoin is the formal analysis,
explicit reconfiguration protocol, and incentive-compatible
reward split.

### Properties

- *Safety.* No two honest committee members commit conflicting
  blocks (BFT property).
- *Liveness.* Committee progresses in `O(delta)` time (responsive).
- *Open membership.* Anyone can join via PoW.
- *Incentive compatibility.* Honest play maximises long-run
  rewards.

### Subsequent influence

- *Algorand (module 0143).* VRF-based committee + BA*; analogous
  to Solida but with PoS instead of PoW.
- *Polkadot (module 0144).* Validator nomination + GRANDPA
  finality.
- *Aptos / Sui.* PoS committee + BFT consensus.

## Practice

Solida was a research paper; no major chain has deployed it
verbatim. Its influence is via subsequent designs (Algorand
adopted the responsive-BFT-committee idea; Hot-Stuff family
adopted the partial-synchrony BFT inside a fixed committee).

### Production-implementation notes

The original paper specifies a synchronous BFT protocol; modern
implementations would use HotStuff or DAG-BFT inside the
committee, retaining the PoW outer loop only as a Sybil-
resistance gate.

### Mining algorithm (proof-of-work function)

Solida's PoW outer loop assumes Bitcoin-style double-SHA-256;
the paper does not propose a custom hash function. Any
collision-resistant hash modelled as a random oracle satisfies
the security analysis. In a hypothetical deployment, the hash
function would typically inherit from the chain Solida is
built on (e.g., double-SHA-256 if Bitcoin-anchored, Ethash if
Ethereum-anchored).

## Verifiability and circuit encoding

**tag: `partial`.**

Solida circuits encode PoW key-block validity (SHA-256), BFT
threshold signatures, and the committee membership window.
Schnorr/BLS aggregate signatures over a SNARK-friendly curve
make the BFT layer cheap to verify. Cost is dominated by SHA-256
PoW (per Bitcoin analysis).

## Known attacks and limitations

- *Reorganisation lag.* If `> f` recent miners are corrupted
  (i.e., a brief mining-power capture), BFT inside the
  committee can fail.
- *PoW outer-loop slowdown.* Committee rotation rate is bounded
  by PoW block rate.
- *Reward split tuning.* The miner/committee reward split must
  be calibrated; bad calibration breaks incentive
  compatibility.

## References

- Abraham, Malkhi, Nayak, Ren, Spiegelman, "Solidus: An
  Incentive-compatible Cryptocurrency Based on Permissionless
  Byzantine Consensus", arXiv 1612.02916; OPODIS 2017.

## Implementation notes

The crate provides a `RewardSplit` helper that computes the
miner share and per-committee-member share for a given block
reward, capacity, and split fraction. Tests verify the split
sums to the total reward (modulo rounding).

See also [`HISTORY.md`](../HISTORY.md), section "2014 to 2017".
