# 0086: GHOST (Greedy Heaviest-Observed Subtree)

## Historical context

Yonatan Sompolinsky and Aviv Zohar published "Secure
High-Rate Transaction Processing in Bitcoin" at FC 2015
(an earlier preprint appeared in 2013), introducing the *GHOST*
fork-choice rule. GHOST replaces Nakamoto's longest-chain rule
with a *heaviest-subtree* rule: at each block, follow the child
whose subtree contains the most accumulated work, not the
longest single chain.

The motivation was throughput. Nakamoto's rule penalises high
block rates: when block intervals approach network propagation
delay, natural forks become frequent (per PSS analysis, module
0085). Honest blocks that fork are *orphaned* and their work
discarded. GHOST counts orphan blocks toward the parent's
weight, preserving security at higher rates.

GHOST became famous as the basis for Ethereum's pre-Merge
fork-choice rule (until 2022). Ethereum's variant (LMD-GHOST)
also influenced its post-Merge consensus (module 0087, Gasper).

## System and threat model

- **Network.** Bounded delay (PSS-style).
- **Failures.** Computational majority of honest hashing power.
- **Cryptography.** Random-oracle SHA-256.
- **Goal.** Higher block rate without sacrificing security.

## Theory

### Block tree, not block chain

In Nakamoto's protocol, miners maintain a *chain*: a linear
sequence of blocks. In GHOST, miners maintain a *tree*: every
block has at most one parent, but blocks may have multiple
children if forks occur.

Each leaf of the tree represents a candidate "tip"; the GHOST
rule selects which tip the next block extends.

### The GHOST rule

Starting at genesis, recursively choose the child whose subtree
has the most blocks (or, more precisely, the most accumulated
work). The chosen tip is the rightmost node reached by this
descent.

```
For node v in the tree:
  if v has no children: return v
  else: pick child c with heaviest subtree(c); recurse on c
```

The heaviest-subtree rule reuses orphan work rather than
discarding it.

### Theorem (Sompolinsky-Zohar, informal)

Under the bounded-delay model, GHOST satisfies common prefix and
chain quality at higher block rates than Nakamoto's longest-chain
rule. Specifically: the security margin of Nakamoto degrades as
`f * Delta` increases; GHOST tolerates higher `f * Delta` because
orphan blocks contribute to the security weight.

### Why GHOST helps

The intuition is that an honest fork is still honest work. In
longest-chain, only one branch survives, so honest work on the
losing branch is wasted. In GHOST, the losing branch's blocks
still vote for their parent, so the parent's subtree is heavier.
This makes it harder for a Byzantine miner to create a heavier
side-chain.

### Throughput improvement

Sompolinsky-Zohar argue GHOST permits ~10x higher block rates
than longest-chain. Empirical Ethereum data (block time ~13s,
~5x Bitcoin propagation delay) supports this in practice; the
network ran with significant uncle-block rates without consensus
breakdown.

### LMD-GHOST (Latest Message Driven)

Ethereum 2.0 adapted GHOST for proof-of-stake (LMD-GHOST):
each validator's *latest* attestation counts as a vote at every
ancestor block, not their accumulated work. This sidesteps the
PoW-specific weight calculation while preserving the
heaviest-subtree intuition. See module 0142 (Ethereum/Gasper).

### Comparison: longest chain vs GHOST

| property                      | longest chain | GHOST          |
| ----------------------------- | ------------- | -------------- |
| data structure                | chain         | tree           |
| orphan blocks                 | discarded     | counted        |
| max safe block rate           | low           | ~10x higher    |
| selection complexity          | O(L)          | O(\|T\|)       |
| fairness to small miners      | low           | high           |
| compatible with PoS           | trivial       | LMD-GHOST      |
| analysis maturity             | mature        | mature         |
| zk-circuit cost               | ~30k/hash     | ~30k/hash + tree |

GHOST's main cost is bookkeeping: tracking the entire visible
tree, not just the longest chain. For full nodes this is a
few hundred MB; for SNARK light clients, it complicates the
header chain.

### Inclusive Block Chain (IBC) protocol

Lewenberg, Sompolinsky, Zohar 2015 extended GHOST with
*inclusive blocks*: orphan transactions can be included by the
heaviest chain (subject to coherence). This further improves
throughput. Cardano's Ouroboros family (module 0125) adopts a
similar idea (Praos with weak forks).

## Practice

- *Ethereum (pre-Merge, 2015 to 2022).* Used a modified GHOST
  rule with uncle blocks rewarded with 7/8 the standard reward.
- *Bitcoin Cash, Bitcoin SV.* Reverted to longest-chain.
- *Conflux (module 0099).* Uses a GHOST-style tree-graph
  protocol.
- *Ethereum (post-Merge, 2022 onward).* LMD-GHOST in PoS.

### Production-implementation notes

- Ethereum stored uncle headers in each block (max 2 uncles per
  block). Miners earned a small reward for including uncles.
- Tip selection in real Ethereum was further modified for
  finalisation (Casper FFG, module 0123).
- Re-org prevention: clients prefer not to re-org if a
  competing branch appears too late.

### Mining algorithm (proof-of-work function)

Ethereum (pre-Merge, 2015 to 2022) ran *Ethash*: a memory-hard
hash function combining Keccak-256 (SHA-3) with a 1-2 GB DAG
("epoch dataset") that miners had to keep in memory. Ethash
was designed to be ASIC-resistant by being bandwidth-bound
rather than compute-bound; the DAG forced miners to use
high-bandwidth GPU memory rather than custom silicon. ASICs
eventually appeared (Bitmain E3, Innosilicon A10) but the
performance gap over commodity GPUs stayed within ~3-5x.

Ethereum Classic (after the Ethereum/ETC fork in 2016)
continued running the Ethash family. After Ethereum's
move to PoS in 2022, ETC kept PoW under a slightly modified
*Etchash* with a larger DAG to invalidate existing Ethereum
ASICs.

Conflux (module 0099) uses *Octopus*, an Ethash-derived
memory-hard hash function adapted for the tree-graph block
structure.

| chain               | hash function | DAG size | block time |
| ------------------- | ------------- | -------- | ---------- |
| Ethereum (pre-Merge) | Ethash       | ~5 GB    | 13 sec     |
| Ethereum Classic    | Etchash       | ~6 GB    | 13 sec     |
| Conflux             | Octopus       | ~5 GB    | 0.5 sec    |
| Ravencoin           | KAWPOW        | ~3 GB    | 1 min      |

## Verifiability and circuit encoding

**tag: `partial`.**

GHOST circuits must encode tree traversal, not just chain
verification. Each child branch's subtree weight is a sum that
must be carried through the circuit. SNARK light clients for
Ethereum (e.g., zkBridge variants) must verify LMD-GHOST
attestations via BLS aggregate signatures, which are
SNARK-friendlier than SHA-256 work-checks.

For PoW GHOST: cost ~30k/SHA-256 + tree-bookkeeping per block.

## Known attacks and limitations

- *Selfish mining.* GHOST does not prevent selfish mining
  (Eyal-Sirer, module 0087); some analyses suggest it slightly
  worsens the threshold.
- *Balance attacks.* In LMD-GHOST PoS, an adversary can
  manipulate validator votes to keep two branches at equal
  weight (Neuder et al. 2020). Mitigations: justified
  checkpoints (Casper FFG).
- *Subtree explosion.* Without limits on uncle inclusion, the
  tree can grow unboundedly. Ethereum capped uncles to 2 per
  block.

## References

- Sompolinsky, Zohar, "Secure High-Rate Transaction Processing
  in Bitcoin", FC 2015.
- Lewenberg, Sompolinsky, Zohar, "Inclusive Block Chain
  Protocols", FC 2015.
- Buterin, "On settlement finality" (Ethereum blog, 2016).
- Buterin et al., "Combining GHOST and Casper", arXiv 2003.03052
  (Gasper).
- Neuder, Moroz, Rao, Parkes, "Selfish Behavior in the Tezos
  Proof-of-Stake Protocol", AFT 2020.

## Implementation notes

The crate provides a tree of blocks and a `ghost_tip` function
implementing the heaviest-subtree rule. Tests verify the rule
selects the tip with the most descendant blocks, and contrasts
the result with longest-chain selection.

See also [`HISTORY.md`](../HISTORY.md), section "2009 to 2014"
and "2014 to 2017".
