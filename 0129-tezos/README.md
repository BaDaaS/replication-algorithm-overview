# 0129: Tezos

## Historical context

Tezos (founded by Arthur and Kathleen Breitman, mainnet
2018) is one of the earliest production proof-of-stake
chains. Its consensus has gone through multiple iterations:

- *Nakamoto-style PoS* (2018-2021). Probabilistic PoS chain.
- *Emmy+* (2019-2021). Improved Nakamoto-PoS with fairness
  fixes.
- *Tenderbake* (2021 onward). Tendermint-derivative BFT with
  deterministic finality, formal proof in Coq.

Tezos is unique among production chains for having its
consensus protocol *formally proven* in Coq (Tenderbake's
proofs were published by Allombert, Bourgoin, Goubault,
Kihn, Mensi, and Yung).

## System and threat model

- **Network.** Bounded-delay PSS.
- **Failures.** Byzantine `f < n/3` of staked tez.
- **Cryptography.** Ed25519, Schnorr, BLS aggregate
  signatures.
- **Goal.** Production PoS with formally-proven safety.

## Theory

### Liquid Proof of Stake

Tezos uses *Liquid Proof-of-Stake* (LPoS): users *delegate*
their stake to *bakers* (block producers). Delegation is
liquid: users can change delegate at any time without
unbonding period.

### Tenderbake (BFT layer)

Tenderbake is a Tendermint-derived BFT protocol with two key
differences:

1. *Two-block finality.* A block is finalised when the next
   block is committed (one extra round of acknowledgement).
2. *Coq-formalised safety.* Algorithm proven correct in Coq.

### Round structure

Per round (15 seconds default):

1. *Propose.* Round leader proposes a block.
2. *Preendorse.* `2/3` validators sign preendorsement.
3. *Endorse.* `2/3` validators sign endorsement.
4. *Commit + finality.* Block is committed; the next block's
   commitment finalises this one.

### Self-amendment

Tezos has *on-chain governance*: protocol upgrades are
proposed, voted on, and enacted by the chain itself. This is
unique among major blockchains.

### Comparison: Tezos vs Cosmos vs Polkadot

| property              | Tezos             | Cosmos        | Polkadot     |
| --------------------- | ----------------- | ------------- | ------------ |
| consensus             | Tenderbake (HS-2) | CometBFT      | BABE+GRANDPA |
| stake model           | LPoS              | bonded PoS    | NPoS         |
| formal proof          | Coq (Tenderbake)  | partial       | partial      |
| governance            | on-chain          | on-chain      | on-chain     |
| year                  | 2018-2021         | 2019          | 2020         |

### Properties

- *Formally-proven Tenderbake* in Coq.
- *Liquid delegation* (no unbonding period).
- *On-chain self-amendment.*
- *PoS with slashing.*

### Limitations

- *Validator count.* ~400-500 active bakers.
- *Self-amendment risk.* Bad upgrades can be voted in.
- *Liquid delegation latency.* Stake snapshot is delayed by
  several cycles.

### Subsequent work

- *Tenderbake formal verification.* Coq proofs by IRIF/Nomadic
  Labs.
- *Etherlink.* EVM rollup on Tezos.
- *Smart rollups.* On-chain optimistic rollups.

## Practice

- *Tezos mainnet.* Production since 2018.
- ~400 active bakers.
- Block time: 15 seconds.
- ~2 block finality (~30 seconds).
- Used for DeFi, NFTs, social applications.

### Production-implementation notes

- *OCaml node implementation.* Tezos's primary client is
  written in OCaml.
- *Multiple clients.* Octez (OCaml), Tezos Mavryk, others.
- *Slashing.* Active for double-sign and double-baking.

## Verifiability and circuit encoding

**tag: `friendly`.**

Tezos uses Ed25519 for blocks (not SNARK-friendly) but BLS for
some signature aggregations. SNARK light clients for Tezos are
in research; Tenderbake's formal-verification work makes
verification semantics clean.

## Known attacks and limitations

- *Self-amendment governance.* Voters can be apathetic;
  participation may be low.
- *Validator clustering.* Largest bakers dominate.
- *Consensus stalls.* Rare but require manual recovery.

## References

- Tezos Foundation, "Tezos Whitepaper", 2014.
- Allombert, Bourgoin, Goubault, Kihn, Mensi, Yung,
  "Tenderbake: A Solution to the Consensus Problem", 2021.
- Nomadic Labs, "Coq formalisation of Tenderbake", 2022.

## Implementation notes

The crate provides a `TenderbakeRound` state with preendorse
and endorse vote counts; status() returns Pending, Preendorsed,
or Committed at the appropriate quorum.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
