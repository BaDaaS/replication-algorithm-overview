# 0083: Bitcoin / Nakamoto Consensus

## Historical context

Satoshi Nakamoto's 2008 whitepaper "Bitcoin: A Peer-to-Peer
Electronic Cash System" introduced the longest-chain
consensus rule, also called *Nakamoto consensus*. It was
the first practical *open-membership* (permissionless)
consensus protocol: any participant can join, propose
blocks, and validate the chain without prior identity
registration.

The protocol is structurally simple but theoretically novel:
it replaces the BFT quorum with *proof of work* (a costly
computation that gates block production), and it replaces
the agreement vote with the *longest-chain rule* (the chain
with the most accumulated work is the canonical one).

Pre-Nakamoto BFT (PBFT, Rampart) required known validator
identities and direct communication. Nakamoto consensus
made consensus possible without these assumptions, opening
the design space for permissionless blockchains.

## System and threat model

- **Network.** Asynchronous gossip, but the security analysis
  assumes a *partially synchronous* model with a known
  network-delay bound `Delta` (Pass-Shi-Seeman 2017).
- **Failures.** A computational majority of honest hashing
  power; up to `1/2` (or `1/3` under stricter analyses) of
  hashing power can be Byzantine.
- **Cryptography.** SHA-256 hashes (collision-resistant
  random oracle); ECDSA signatures (secp256k1).
- **Goal.** Eventual probabilistic agreement on a single
  growing chain.

## Theory

### Block structure

Each block contains:

- Previous-block hash.
- Merkle root of transactions.
- Timestamp.
- Nonce (the proof-of-work witness).
- Difficulty target.

A block is *valid* iff `sha256(sha256(block-header)) <
difficulty-target`.

### The longest-chain rule

Each node maintains the chain whose total accumulated work
is greatest (in practice: longest chain since difficulty is
nearly constant). New blocks extending this chain are
accepted; competing forks are reorganised away.

### Theorem (Nakamoto consensus security)

Under the partially synchronous model with `Delta` bound and
honest hashing fraction `alpha > 1/2 + epsilon`:

- *Common prefix.* For any two honest nodes' chains at time
  `t`, the chains agree up to depth `k` blocks, with `k`
  growing in `t`. Probability of disagreement decays
  exponentially in `k`.
- *Chain growth.* The honest chain grows by at least
  `(1 - epsilon) * alpha * t * mining_rate` blocks per unit
  time.
- *Chain quality.* At least `alpha - epsilon` fraction of
  blocks in any sufficiently long chain are honest.

These three properties together imply linearisable SMR with
*probabilistic finality* (depth `k` blocks deep are
permanent with probability `1 - exp(-k)`).

The analysis was made rigorous by Garay-Kiayias-Leonardos
2015 (the *Bitcoin backbone*; module 0084) and refined in
Pass-Shi-Seeman 2017.

### Differences from BFT consensus

| property                | PBFT-family BFT  | Bitcoin / Nakamoto |
| ----------------------- | ---------------- | ------------------ |
| validator set           | known            | open membership    |
| Sybil resistance        | identity         | proof-of-work      |
| communication pattern   | direct (point-to-point) | gossip       |
| finality                | deterministic    | probabilistic      |
| commit latency          | seconds          | ~10 minutes per block, ~1 hour for finality |
| throughput              | thousands tx/s   | ~7 tx/s (Bitcoin)  |
| fault tolerance         | f < n/3          | hash power > 1/2   |
| energy cost             | low              | very high          |
| protocol complexity     | high             | low                |

The trade-off: Nakamoto consensus is much simpler but
spends enormous energy on PoW. BFT-family protocols are
energy-efficient but require identity registration.

### Why "longest" rather than "first"?

A faster block-propagation rule (e.g., always accept the
first valid block heard) would be vulnerable to Sybil
attacks: an adversary could flood the network with
slightly-different valid blocks. The longest-chain rule
ties block acceptance to *cumulative work*, which a Sybil
adversary cannot fake without spending the corresponding
hashing power.

### Selfish mining and other attacks

- *Selfish mining* (Eyal-Sirer 2014, module 0087): a miner
  with > 1/4 hash power can boost relative profit by
  withholding blocks; not a safety failure but an incentive
  failure.
- *51% attacks*: a majority hashing adversary can rewrite
  history. Has happened on smaller PoW chains (Ethereum
  Classic 2019, 2020).
- *Eclipse attacks* (Heilman et al. 2015): an adversary
  controlling a victim's network connections can show them
  a divergent chain.

## Practice

- *Bitcoin mainnet.* Production since 2009.
- *Litecoin, Dogecoin, Bitcoin Cash, and many others.*
  Variants of Nakamoto consensus.
- *Ethereum (until 2022).* Pre-Merge ran a GHOST variant
  (module 0086).

### Mining algorithm (proof-of-work function)

Bitcoin's PoW puzzle is `sha256(sha256(header)) <
difficulty_target`: the *double-SHA-256* hash of the
80-byte block header must be numerically smaller than the
current target. SHA-256 was chosen for three reasons: NIST
standardisation (auditability), efficient ASIC implementation
(making it a stable substrate), and well-understood collision
resistance. Hashing the SHA-256 output a second time
("double-SHA-256") was Nakamoto's belt-and-braces precaution
against length-extension attacks on the Merkle-Damgard
construction.

Other Nakamoto-style chains use different hash functions,
each chosen for a specific design goal:

| chain                | hash function     | design goal                            |
| -------------------- | ----------------- | -------------------------------------- |
| Bitcoin              | double-SHA-256    | efficient ASIC; hardware specialisation |
| Bitcoin Cash, BSV    | double-SHA-256    | inherit Bitcoin substrate               |
| Litecoin             | Scrypt            | memory-hardness vs ASIC (early goal)    |
| Dogecoin             | Scrypt            | merge-mined with Litecoin               |
| Monero               | RandomX           | CPU-friendly; ASIC-resistant by design  |
| Zcash                | Equihash          | memory-hard; egalitarian mining         |
| Ravencoin            | KAWPOW            | GPU-only; ASIC-resistant                |
| Ethereum (pre-Merge) | Ethash            | DAG-based memory-hard (module 0086)     |
| Ethereum Classic     | Etchash           | Ethash variant, larger DAG              |
| Conflux              | Octopus           | Ethash-derived (module 0099)            |
| Kaspa                | kHeavyHash        | matrix-mul; GPU-friendly (module 0096)  |
| Grin, Beam           | Cuckoo Cycle      | graph problem; bandwidth-hard           |
| Handshake            | BLAKE2b + SHA3-256 | naming-system-specific                  |
| Siacoin              | Blake2b           | storage-chain-specific                  |
| Decred               | BLAKE-256         | hybrid PoW/PoS substrate                |

ASIC-resistance is the most common motivation for deviating
from SHA-256: Scrypt (initial), RandomX, KAWPOW, and Cuckoo
Cycle were all designed to make custom-silicon mining
uneconomical. In practice, every "ASIC-resistant" function
has eventually been ASIC-mined; RandomX (Monero) is the
closest to holding the line via deliberate CPU-friendly
opcodes that resist FPGA/ASIC speedup.

### Why Nakamoto's design was a breakthrough

Pre-2008 BFT could not work in open membership: identity
spoofing (Sybil attack) made any quorum-based scheme
trivially defeatable. Nakamoto's insight was that *cost*
(hash power) could substitute for identity. Once
hash-power was tied to mining cost, Sybil attacks became
expensive rather than free, and consensus became viable
without an authority.

### Production-implementation notes

- *Difficulty adjustment.* Bitcoin recalibrates difficulty
  every 2016 blocks (~2 weeks) to maintain ~10-minute
  block intervals.
- *Coinbase rewards.* Block proposer earns subsidy + fees.
- *Halving.* Subsidy halves every 210000 blocks (~4
  years), eventually reaching zero (around 2140).
- *Block size limit.* 1 MB historically; SegWit and Taproot
  effectively raised this.

## Verifiability and circuit encoding

**tag: `partial`.**

A SNARK proof of Bitcoin's longest-chain rule encodes:

- Block-header chain (each header references the previous).
- Per-block PoW verification: SHA-256 below target.
- Total-work computation.

SHA-256 is *not* SNARK-friendly: ~30k constraints per hash,
2 hashes per block (double-SHA256). For a 1000-block
chain: ~6 * 10^7 constraints.

zk-bridges to Bitcoin (zkBridge 2022, Galactica) use
Pickles or Halo 2 with SHA-256 sub-circuits; per-block
verifier cost is ~6 * 10^4 constraints (amortised) with
recursive aggregation.

A "verifiable Bitcoin light client" remains an active
research area; full-node verification cost is currently
prohibitive for L1 contracts.

## Known attacks and limitations

- *51% attack.* Dominant computational majority can rewrite
  history. Requires sustained ~50%+ hash power.
- *Selfish mining.* Profitable above ~25% hash power.
- *Eclipse attack.* Network-level adversary can isolate a
  victim.
- *Front-running and MEV.* Miners can reorder transactions
  for profit.

## Implementation notes

The crate provides a minimal Bitcoin-style block: header,
parent hash, nonce, transactions. Tests verify a chain of 3
blocks satisfies the longest-chain rule (each subsequent
block extends the previous).

A real PoW miner (computing valid SHA-256 nonces) is
intentionally omitted; the simulation focuses on chain
structure rather than mining.

## References

- Nakamoto, "Bitcoin: A Peer-to-Peer Electronic Cash
  System", 2008.
- Garay, Kiayias, Leonardos, "The Bitcoin Backbone
  Protocol", Eurocrypt 2015.
- Pass, Seeman, Shelat, "Analysis of the Blockchain Protocol
  in Asynchronous Networks", Eurocrypt 2017.

See also [`HISTORY.md`](../HISTORY.md), section "2009 to
2014".
