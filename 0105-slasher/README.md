# 0105: Slasher

## Historical context

Vitalik Buterin published "Slasher: A Punitive Proof-of-Stake
Algorithm" on the Ethereum blog in 2014. Slasher was Buterin's
first published proof-of-stake design, intended as a successor
to Bitcoin's PoW. It introduced the *slashing* concept that
later became central to Casper FFG (module 0123) and most
modern PoS protocols.

The core idea: stakers deposit collateral and sign blocks; if
they sign two conflicting blocks (equivocate), anyone can
submit a *slashing transaction* that destroys their deposit.
This ties stakers' financial incentive to honest behaviour,
addressing the *nothing-at-stake* problem in naive PoS.

Slasher itself was never deployed in production. Its successor,
*Slasher 2.0* (2014) improved on it; both were superseded by
Casper FFG / Casper CBC. The slashing mechanism, however, is
a permanent contribution.

## System and threat model

- **Network.** Bounded delay.
- **Failures.** Byzantine; stake fraction `f < 1/3`.
- **Cryptography.** Signatures over committed blocks.
- **Goal.** Permissionless PoS with disincentivised
  equivocation.

## Theory

### Nothing-at-stake problem

In naive PoS, a staker can sign multiple competing chain
heads at no cost, since signing is free. This breaks the
honest-majority assumption: stakers will sign every chain
they see to maximise expected reward, leading to permanent
forks.

Bitcoin's PoW is *automatically* free of this problem: a miner
must spend hashing power to produce a block, so they can only
support one chain at a time.

### Slashing solves nothing-at-stake

Slasher requires every staker to deposit collateral. If they
sign two conflicting blocks (or sign blocks at equivocating
heights), anyone can submit a *slashing transaction* containing
both signatures; the protocol verifies the signatures and
destroys the staker's deposit.

This makes signing two competing chains *costly* (loss of
deposit), restoring the security argument.

### Two slashable conditions

1. *Double-vote.* Signing two blocks at the same height.
2. *Surround-vote.* (Casper FFG, module 0123) Signing a
   sequence of blocks that surrounds a previously-signed
   sequence.

Slasher 1.0 covered only double-vote; Casper FFG added
surround-vote.

### Comparison: Slasher vs PoW vs Casper FFG

| property            | PoW                  | Slasher              | Casper FFG     |
| ------------------- | -------------------- | -------------------- | -------------- |
| anti-equivocation   | physical (hash work) | financial (slashing) | financial (FFG) |
| nothing-at-stake    | free                 | financial penalty    | financial penalty + finality |
| finality            | probabilistic        | probabilistic        | deterministic (FFG checkpoint) |
| deposit lock period | n/a                  | weeks-months         | months          |

### Subsequent work

- *Slasher 2.0* (Buterin 2014). Tweaks to deposit/penalty
  structure.
- *Casper FFG* (Buterin-Griffith 2017, module 0123). Combines
  slashing with deterministic finality.
- *Casper CBC* (Zamfir 2017). Different finality model; same
  slashing principle.
- *Modern PoS chains* (Cardano, Tezos, Cosmos, Polkadot,
  Algorand). All use slashing.

## Practice

Slasher itself was not deployed; its conceptual contributions
shaped subsequent PoS chains. Slashing is now a standard
feature in:

- *Ethereum* (since the Merge, 2022). Slashing for double-
  vote, surround-vote, and inactivity.
- *Cosmos* (Tendermint). Slashing for double-sign and downtime.
- *Polkadot.* Slashing for invalid block production.
- *Cardano.* Less aggressive (mostly via stake-pool
  reputation).

### Production-implementation notes

- *Whistleblower reward.* Slashing transactions typically pay a
  bounty to the submitter, ensuring rapid detection.
- *Slashing fraction.* Often only ~1-3% of deposit (not full)
  to balance security vs lock-in pressure.
- *Inactivity leak.* Some protocols (Ethereum) gradually slash
  inactive validators to recover liveness during long
  partitions.

## Verifiability and circuit encoding

**tag: `partial`.**

Slashing circuits encode signature-pair verification: given two
signed messages by the same staker at conflicting heights,
verify both signatures and the conflict condition. Cost
dominated by signature verification (~100-1000 constraints per
signature on SNARK-friendly curves).

## Known attacks and limitations

- *Long-range attacks.* If an attacker buys old keys, they can
  rewrite history; slashing only applies if validators are still
  bonded. Mitigation: weak subjectivity checkpoints.
- *Posterior corruption.* A staker's key compromised after
  unbonding can sign old blocks; slashing has expired.
  Mitigation: forward-secure signatures (Praos).
- *Validator collusion.* If `> 1/3` stake colludes, slashing
  cannot be enforced.

## References

- Buterin, "Slasher: A Punitive Proof-of-Stake Algorithm",
  Ethereum blog, 2014.
- Buterin, "Slasher Ghost, and Other Developments in Proof of
  Stake", Ethereum blog, 2014.

## Implementation notes

The crate provides a `Slashable` enum with `DoubleVote` and
`SurroundVote` variants, and a `detect` function that takes a
list of signed messages and returns slashable evidence. Tests
verify the double-vote detection.

See also [`HISTORY.md`](../HISTORY.md), section "2014 to 2017".
