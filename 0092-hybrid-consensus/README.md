# 0092: Hybrid Consensus

## Historical context

Rafael Pass and Elaine Shi published "Hybrid Consensus:
Efficient Consensus in the Permissionless Model" at DISC 2017.
Like ByzCoin (module 0090) and Solida (module 0091), Hybrid
Consensus combines PoW for committee selection with BFT for
finality. The contribution: a *generic* framework that takes
any responsive permissioned BFT and any blockchain (PoW or
PoS) and combines them into a permissionless responsive
protocol.

The key abstraction: the chain protocol provides committee
*reconfiguration*, and a permissioned BFT runs *between
reconfigurations* to commit transactions. As long as at most
`f < c/3` of the current committee is corrupt, the BFT layer
provides immediate finality.

Hybrid Consensus is most often cited for its *responsive*
property: when the network is well-behaved, transactions
commit at network speed rather than at PoW block-interval
speed.

## System and threat model

- **Network.** Bounded delay outside committee, partially
  synchronous inside.
- **Failures.** PoW honest majority globally; `f < c/3`
  Byzantine inside committee of size `c`.
- **Cryptography.** SHA-256 PoW for committee membership;
  any threshold-signature scheme for BFT.
- **Goal.** Open-membership SMR with deterministic finality
  and responsiveness.

## Theory

### Two-layer architecture

Hybrid Consensus separates concerns more cleanly than ByzCoin:

1. *Chain layer.* Runs Nakamoto-style longest-chain (PoW or
   PoS) to maintain a long-term committee selection log.
2. *Daypay layer.* Permissioned BFT among the current
   committee, finalising transactions with immediate finality
   between reconfigurations.

Reconfiguration: each new chain block reconfigures the
committee, possibly adding or removing members.

### Modularity

The Pass-Shi paper proves: any "robust transaction ledger"
(GKL-style chain) plus any "responsive permissioned BFT" gives
a hybrid consensus. This is a *reduction*-style result: future
chain protocols (Praos, Snow White) and future BFT protocols
(HotStuff) automatically combine.

### Responsiveness theorem

Under network-typical (bounded actual delay `delta`)
conditions:

- *BFT inside committee.* Commits in `O(delta)`, not `O(Delta)`.
- *Reconfiguration via PoW.* Committed at PoW block-interval
  rate (~10 minutes), but BFT continues using the previous
  committee while waiting.

This gives the user *immediate* tx finality at network speed,
while the chain layer is still backstopped by Nakamoto-style
security.

### Comparison: hybrid family

| protocol         | chain layer  | BFT layer   | year | committee size |
| ---------------- | ------------ | ----------- | ---- | -------------- |
| ByzCoin          | Bitcoin      | PBFT + CoSi | 2016 | ~hundreds      |
| Solida           | Bitcoin      | sync BFT    | 2016 | ~tens          |
| Hybrid Consensus | generic      | generic     | 2017 | configurable   |
| Thunderella      | generic      | sync (fast) | 2018 | configurable   |
| Algorand         | none (BA*)   | BA*         | 2019 | ~thousands     |
| Pala / Pili      | sync         | rotated     | 2018 | configurable   |

Hybrid Consensus is the abstract framework; ByzCoin and Solida
are instantiations that pre-date but fit within it.

### Properties

- *Persistence.* Confirmed transactions are never reverted (BFT
  property).
- *Liveness with responsiveness.* In good conditions, latency
  is `O(delta)`.
- *Liveness with synchrony.* Even in bad conditions, latency
  is bounded by the chain protocol.
- *Open membership.* Anyone with PoW or stake can join.

### Slow / fast paths

Hybrid Consensus has a "fast path" (BFT, when `<= f` corrupt
committee members) and a "slow path" (re-elect committee via
chain layer). The fast path commits in `O(delta)`; the slow
path takes a few PoW blocks. This split is canonical for
modern designs (HotStuff timeouts, Tendermint nil-prevotes).

## Practice

Hybrid Consensus is foundational rather than directly deployed.
Its impact is via:

- *Snow White* (module 0124). Daian-Pass-Shi 2019 extends to PoS.
- *Thunderella* (module 0095). Pass-Shi 2018 sharpens the
  responsive layer.
- *DiemBFT* (module 0057). Permissioned BFT with reconfiguration.
- *Aptos / Sui.* PoS chain layer + BFT consensus.

### Production-implementation notes

- The committee size `c` must balance security (large `c`
  tolerates more attackers) and BFT scalability (small `c`
  commits faster).
- Reconfiguration latency: when the committee changes, in-flight
  BFT votes must be carried over or re-issued. Modern designs
  (HotStuff-2) make this almost free.
- Slashing and rewards: out-of-band incentive design; not
  specified by the framework.

## Verifiability and circuit encoding

**tag: `partial`.**

Hybrid Consensus circuits encode the chain layer's PoW (or PoS
VRF) plus the BFT layer's threshold signatures. Costs depend on
the chosen instantiation:

- PoW chain: SHA-256 dominates (~30k constraints/hash).
- PoS chain: VRF + BLS aggregate (~10^4 to 10^6 constraints).
- BFT layer: threshold signature verification (~100 to 1000
  constraints amortised).

Mina's Samasika protocol (module 0145) is a hybrid-style design
encoded in Pickles SNARKs.

## Known attacks and limitations

- *Long reconfiguration latency.* PoW committee rotation is
  slow (~10 min); attackers can exploit this window.
- *Committee corruption.* If `> f` committee members are
  corrupted, BFT safety fails until next reconfiguration.
- *Fast/slow path inconsistencies.* Care needed in the
  transition between paths to avoid safety violations.

## References

- Pass, Shi, "Hybrid Consensus: Efficient Consensus in the
  Permissionless Model", DISC 2017.
- Pass, Shi, "Thunderella: Blockchains with Optimistic Instant
  Confirmation", Eurocrypt 2018.

## Implementation notes

The crate provides a `Path` enum (`Fast` / `Slow`) and a
`select_path` function that picks the fast path when committee
honesty exceeds the BFT threshold and the slow path otherwise.
Tests verify the transition logic.

See also [`HISTORY.md`](../HISTORY.md), section "2014 to 2017".
