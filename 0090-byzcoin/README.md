# 0090: ByzCoin

## Historical context

Eleftherios Kokoris Kogias, Philipp Jovanovic, Nicolas Gailly,
Ismail Khoffi, Linus Gasser, and Bryan Ford published
"Enhancing Bitcoin Security and Performance with Strong
Consistency via Collective Signing" at USENIX Security 2016.
The paper proposes *ByzCoin*: combine Bitcoin's PoW with PBFT
to achieve high throughput, low latency, and *strong*
(deterministic) consistency.

The key idea: PoW selects a *committee* of recent miners (the
last `k` block proposers); the committee runs PBFT to commit
each new block. PoW provides Sybil resistance and committee
selection; PBFT provides immediate finality.

This was an early example of the now-common pattern: use PoW or
PoS for committee selection, then run a classical BFT protocol
inside the committee. Subsequent designs (Solida, Hybrid
Consensus, Algorand, modern PoS rollups) refined this idea.

## System and threat model

- **Network.** Partially synchronous within the committee
  (PBFT-compatible); bounded delay outside.
- **Failures.** A standard PBFT-style assumption inside the
  committee: at most `f < n/3` Byzantine; outside, computational
  honest-majority (PoW) for committee membership.
- **Cryptography.** SHA-256 PoW for committee membership;
  Schnorr threshold signatures (CoSi tree-based aggregate
  signing) for PBFT.
- **Goal.** Bitcoin-level throughput and Byzantine-finality
  blocks.

## Theory

### Two block types

ByzCoin uses key blocks and microblocks similar to Bitcoin-NG
(module 0088), but extends the idea:

- *Key blocks.* Carry PoW, advance the committee. Each new key
  block evicts the oldest committee member and admits the new
  miner.
- *Microblocks.* Signed by a *committee threshold*, not just
  the leader. Microblocks have BFT finality.

### Committee selection by PoW window

The committee is the set of the last `w` key-block miners (e.g.,
`w = 144` for ~24 hours of Bitcoin mining). Each new key block
advances the window by one: oldest member leaves, new miner
joins.

Under honest-majority of mining power, the committee inherits a
honest majority with high probability provided `w` is large
enough.

### CoSi: scalable threshold signatures

ByzCoin uses *collective signing* (CoSi, Syta et al. 2016):
a tree-based protocol that aggregates Schnorr signatures from
all committee members in `O(log n)` rounds. The output is a
single Schnorr signature with `n - f` size proof, verifiable in
constant time.

CoSi enables ByzCoin to scale to ~thousand-member committees, far
beyond standard PBFT (~100 members).

### Round protocol

For each microblock:

1. *Leader proposal.* Current leader (rotating) broadcasts a
   block to the committee.
2. *CoSi prepare.* Tree-aggregated signatures, all members
   sign-or-abstain.
3. *CoSi commit.* Second round: members sign the prepare
   threshold-signature output.
4. *Microblock published.* Block + 2 CoSi signatures.

Equivalent to PBFT prepare/commit but with tree aggregation in
each phase.

### Comparison with related designs

| protocol         | committee selection | finality        | tx throughput  | year |
| ---------------- | ------------------- | --------------- | -------------- | ---- |
| Bitcoin          | none (full network) | probabilistic   | ~7 tx/s        | 2008 |
| Bitcoin-NG       | leader (PoW)        | probabilistic   | ~thousands     | 2016 |
| ByzCoin          | committee (PoW)     | BFT (immediate) | ~thousands     | 2016 |
| Solida           | committee (PoW)     | BFT             | ~hundreds      | 2017 |
| Hybrid Consensus | committee (PoW)     | BFT             | ~hundreds      | 2017 |
| Algorand         | committee (VRF/PoS) | BFT (immediate) | ~thousands     | 2019 |
| Cosmos           | fixed BFT validators | BFT (immediate) | ~thousands     | 2018 |

ByzCoin is the bridge between fully open PoW and committee BFT.
Subsequent work (Algorand) replaced PoW with VRF-based PoS for
committee selection.

### Properties

- *Strong consistency.* Microblocks have immediate BFT
  finality; no probabilistic confirmation.
- *High throughput.* CoSi aggregate signatures keep verification
  cost linear in committee size, not quadratic like vanilla
  PBFT.
- *Permissionless.* Anyone with hashing power can join the
  committee window.
- *Backward-compatible.* ByzCoin chains are also valid Bitcoin
  longest-chain entries.

### Limitations

- *Committee responsiveness.* If `f >= n/3` of recent miners go
  offline, the committee stalls. PoW-driven committee rotation
  is too slow to recover quickly.
- *Adversarial mining.* An attacker who briefly controls > 1/3
  of recent mining power can attack BFT safety inside the
  committee, even if they hold < 1/2 of total hashing.
- *CoSi tree dependence.* Tree-based aggregation is fragile to
  membership changes; ByzCoin uses CoSi only after the
  committee is stable.

## Practice

ByzCoin was deployed as a research prototype by the EPFL DEDIS
group. It has not been adopted by major chains, but its key
ideas are reflected in:

- *Algorand.* Cryptographic-sortition committee + BFT.
- *Diem (Libra).* Permissioned BFT committee.
- *Aptos / Sui.* PoS committee + leader-based BFT.
- *Cosmos / Polkadot.* Validator sets with BFT consensus.

## Verifiability and circuit encoding

**tag: `partial`.**

ByzCoin circuits encode three things: PoW key-block validity
(SHA-256), CoSi threshold signatures (Schnorr aggregate), and
the committee window. Schnorr signatures over a SNARK-friendly
curve are tractable; SHA-256 PoW is the bottleneck. Approximate
costs (per microblock): SHA-256 PoW = ~30k constraints
(once per epoch); CoSi verification = ~100 constraints (single
aggregate). A SNARK-light-client for ByzCoin is roughly
comparable to a Bitcoin SNARK light client + signature checks.

## Known attacks and limitations

- *Stalling.* If many recent miners go offline simultaneously,
  the committee cannot reach 2f+1 votes.
- *Brief majority captures.* A short-term miner with 1/3 of
  the window's hashing power can break safety.
- *CoSi tree disruption.* Network failures during tree-based
  signing can stall a round.

## References

- Kogias, Jovanovic, Gailly, Khoffi, Gasser, Ford,
  "Enhancing Bitcoin Security and Performance with Strong
  Consistency via Collective Signing", USENIX Security 2016.
- Syta, Tamas, Visher, Wolinsky, Jovanovic, Gasser, Gailly,
  Khoffi, Ford, "Keeping Authorities Honest or Bust with
  Decentralized Witness Cosigning", S&P 2016 (CoSi).

## Implementation notes

The crate provides a `Committee` struct that maintains a
sliding window of recent key-block miners and exposes
`is_member`, `size`, and committee `rotate`. Tests verify the
sliding window logic.

See also [`HISTORY.md`](../HISTORY.md), section "2014 to 2017".
