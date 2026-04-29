# 0106: Casper FFG

## Historical context

Vitalik Buterin and Virgil Griffith published "Casper the
Friendly Finality Gadget" (arXiv 1710.09437, 2017). FFG is a
*finality overlay* for an underlying chain protocol (originally
Ethereum's GHOST PoW chain): the underlying chain decides block
ordering, and FFG provides PBFT-style deterministic finality
on *checkpoints* (every 32 blocks, an *epoch boundary*).

The "friendly" in the name reflects two properties: FFG can be
added on top of an existing chain without invasive changes,
and validators only need to vote on infrequent checkpoints
(low overhead).

FFG was deployed in Ethereum's *Beacon Chain* (December 2020)
and remained the finality protocol after the Merge (September
2022). Combined with LMD-GHOST (module 0086), it forms
*Gasper* (module 0142).

## System and threat model

- **Network.** Partially synchronous (PBFT-compatible).
- **Failures.** Byzantine; finality requires `f < n/3` of
  bonded stake.
- **Cryptography.** BLS aggregate signatures over checkpoint
  votes.
- **Goal.** Deterministic finality on top of a chain protocol.

## Theory

### Checkpoints and epochs

The underlying chain produces blocks; every `E` blocks (e.g.,
`E = 32` in Ethereum) is a *checkpoint*. Validators vote on
*links* between adjacent checkpoints: a link `(A, B)` means the
voter accepts that `B` is the descendant of `A`.

### Justification and finalisation

A checkpoint `B` is *justified* if a 2/3 stake-weighted
super-majority votes for the link `(A, B)` from any justified
ancestor `A`. The genesis is justified by definition.

A checkpoint `A` is *finalised* if it is justified *and* a
direct child `B` is also justified (the "1+1" rule).

Equivalently: two consecutive justified checkpoints implies the
first is finalised.

### Slashing conditions

Validators must avoid two slashable conditions:

1. *Double-vote.* Voting for two different links at the same
   target height.
2. *Surround-vote.* Voting for a link `(A, B)` and another link
   `(C, D)` where `(C, D)` surrounds `(A, B)`: i.e.,
   `height(C) < height(A) < height(B) < height(D)`.

Slashing destroys the validator's deposit (~3% to ~100%
depending on conditions; 1-2% in production for honest mistakes).

### Theorem (Buterin-Griffith 2017, informal)

Under partial synchrony with `f < n/3` Byzantine stake: FFG
provides *plausibility* (no two conflicting checkpoints
finalise) and *liveness* (eventually some checkpoint
finalises) in the GST + PBFT-style model.

### Plausibility theorem

If two conflicting checkpoints `A` and `A'` are both finalised
(at the same height), then at least 1/3 of bonded stake has
violated a slashable condition.

This is the *accountable safety* property: safety violations
have provable culprits, who can be slashed.

### Comparison: FFG vs PBFT vs Tendermint

| property              | PBFT          | Tendermint    | Casper FFG     |
| --------------------- | ------------- | ------------- | -------------- |
| committee             | fixed         | fixed         | bonded stake   |
| finality              | every block   | every block   | every epoch    |
| invasive integration  | high          | high          | overlay        |
| accountable safety    | partial       | partial       | yes (slashing) |
| underlying chain      | n/a           | own           | external (GHOST) |
| latency               | seconds       | seconds       | ~6 minutes     |
| year                  | 1999          | 2014          | 2017           |

FFG's main innovation is the overlay design: an existing chain
gets finality without a major rewrite.

### Properties

- *Deterministic finality* via 2/3 super-majority.
- *Accountable safety* (slashable conditions).
- *Overlay design* (compatible with any chain).
- *Low overhead* (votes only on epoch boundaries).

### Subsequent work

- *Casper CBC* (Zamfir 2017). Different finality semantics,
  same slashing principle.
- *Gasper* (Buterin et al. 2020, module 0142). FFG + LMD-GHOST
  combined.
- *Tendermint and HotStuff* (modules 0055, 0056). Direct chain
  protocols with similar slashing.

## Practice

- *Ethereum Beacon Chain* (December 2020 onward). FFG provides
  finality on the proof-of-stake chain.
- *Ethereum mainnet* (post-Merge, September 2022 onward). FFG
  is the finality gadget for the unified chain.

### Production-implementation notes

- *Epoch length.* 32 blocks (Ethereum); ~6.4 minutes per epoch.
- *Validator count.* ~1 million as of 2024 (each with 32 ETH
  stake).
- *BLS aggregate signatures.* Required to scale to 1M
  validators; aggregate is verified in constant time.
- *Slashing.* Effective slashing fraction 1-3% of stake for
  honest mistakes; up to 100% for coordinated attacks.

## Verifiability and circuit encoding

**tag: `friendly`.**

Casper FFG circuits encode BLS aggregate-signature verification
plus the justification/finalisation rules. BLS over BLS12-381
is reasonably SNARK-friendly (~10^4 to 10^6 constraints per
aggregate verification). FFG is a target for SNARK light
clients (e.g., zkBridge, Succinct Labs' Telepathy).

The slashing-condition predicates are simple arithmetic
comparisons (well-suited for SNARKs).

## Known attacks and limitations

- *Long-range attacks.* If validators leak old keys, a chain
  can be rewritten beyond the slashing window. Mitigated by
  *weak subjectivity*: clients trust a recent finalised
  checkpoint.
- *Inactivity leak.* If `> 1/3` validators are offline, no
  finality. Ethereum's inactivity leak gradually reduces stake
  of offline validators to recover finality.
- *Validator collusion.* `>= 1/3` colluding stake can stall
  finality (but not break safety).

## References

- Buterin, Griffith, "Casper the Friendly Finality Gadget",
  arXiv 1710.09437, 2017.
- Buterin et al., "Combining GHOST and Casper", arXiv
  2003.03052, 2020 (Gasper).
- Ethereum Foundation, "The Merge" specification, 2022.

## Implementation notes

The crate provides `Checkpoint` and `Vote` types and a
`Justifications` tracker that records justified checkpoints.
Tests verify that a checkpoint becomes justified after 2/3
votes and finalised after a justified child is added.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
