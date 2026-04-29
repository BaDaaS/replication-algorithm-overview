# 0097: Prism

## Historical context

Vivek Bagaria, Sreeram Kannan, David Tse, Pramod Viswanath, and
Xuechao Wang published "Prism: Deconstructing the Blockchain
to Approach Physical Limits" at CCS 2019 (preprint 2018, arXiv
1810.08092). Prism takes the blockchain decoupling idea further
than Bitcoin-NG (module 0088) or PHANTOM (module 0096) by
splitting Nakamoto's monolithic block into *three* independent
roles, each running as its own parallel chain:

1. *Proposer chain.* Slow, low-throughput, leader-electing chain
   producing one block at a time.
2. *Voter chains.* `m` parallel voting chains; each voter chain
   votes on the proposer chain's blocks.
3. *Transaction blocks.* Independent blocks containing
   transactions, attached to a proposer block once confirmed.

The result: each role is optimised independently. Throughput
approaches network bandwidth limits (~10000 tx/s in
prototypes); confirmation latency approaches network propagation
delay (~seconds); security matches Nakamoto.

Prism is one of the most influential PoW research papers post-
Bitcoin: it shows the throughput/latency/security trade-off can
be relaxed by separating concerns, anticipating the modular
blockchain thesis (module 0152).

## System and threat model

- **Network.** Bounded-delay (PSS).
- **Failures.** Computational honest-majority hashing.
- **Cryptography.** SHA-256 PoW per role.
- **Goal.** Approach the physical limits: throughput up to
  network bandwidth, latency down to propagation delay, while
  preserving Nakamoto security.

## Theory

### Three block types, three roles

| role             | function                          | rate       | reward |
| ---------------- | --------------------------------- | ---------- | ------ |
| Proposer block   | Leader election, content ordering | low (~1/min) | yes  |
| Voter block (m chains) | Confirmation voting          | medium     | small  |
| Transaction block | Carries transactions             | high       | yes    |

Each block type uses a separate hash target (e.g., 1/256, 8/256,
247/256). A miner finds *some* hash; the bits of the hash
determine which block type they mined.

### Why this gives high throughput

Transaction blocks are produced at the bandwidth-saturating
rate. They are not on the critical path for finality; they are
referenced by proposer blocks once they are stable.

Proposer blocks happen at standard Nakamoto rate; they only
order content (which previously-mined transaction blocks they
include).

Voter blocks happen at medium rate; they vote on proposer
blocks. Multiple parallel voter chains aggregate votes from
different miners.

### Confirmation latency

A proposer block is confirmed when:

- Each of the `m` voter chains has voted on it past depth `k`.
- The `1 - exp(-Theta(m * k))` confidence is met.

For `m = 1000` voter chains, each vote needs only depth `k = 6`
or so for high confidence. Aggregate confirmation latency
becomes ~tens of seconds.

### Security theorem (Bagaria et al. 2019)

Under the bounded-delay PSS model, with honest hashing fraction
`alpha > 1/2 + epsilon` and parameters `m, k` calibrated to the
network:

- *Common prefix.* Proposer-chain prefix stable up to `k * m`
  voter-block depth.
- *Chain growth.* Proposer-chain growth proportional to honest
  hash share.
- *Throughput.* Transaction-block rate up to `f_tx`, where
  `f_tx` is calibrated to network bandwidth.

### Comparison: Bitcoin, Bitcoin-NG, GHOSTDAG, Prism

| protocol     | throughput | latency | structure       | year |
| ------------ | ---------- | ------- | --------------- | ---- |
| Bitcoin      | ~7 tx/s    | ~1 hour | chain           | 2008 |
| GHOST/Etheth | ~15 tx/s   | minutes | tree            | 2013 |
| Bitcoin-NG   | thousands  | ~1 hour (key) | key + micros | 2016 |
| GHOSTDAG     | thousands  | seconds | DAG             | 2020 |
| Prism        | tens of thousands | seconds | parallel chains | 2018 |

Prism's main advantage is its *near-optimality*: throughput
within a constant factor of network bandwidth, latency within
a constant factor of propagation delay.

### Properties

- *High throughput.* Bandwidth-bound, not propagation-bound.
- *Low latency.* Tens of seconds for high-confidence finality.
- *Nakamoto-equivalent security.* `alpha > 1/2 + epsilon`
  suffices.
- *Modular.* Each role tuned independently.

### Subsequent influence

- *OHIE* (Yu et al. 2020, module 0098). Multiple parallel
  Nakamoto chains.
- *Cordial Miners* (module 0074). DAG-BFT with similar
  decoupling.
- *Modular blockchain thesis.* Celestia, EigenLayer, et al.
  separate consensus from execution and data availability.

## Practice

- *Prism prototype* deployed by the authors achieving ~10000
  tx/s on a 1 Gbit/s network.
- No major chain has adopted Prism verbatim; the protocol is
  considered too complex relative to alternatives like
  GHOSTDAG.

### Production-implementation notes

- The number of voter chains `m` is a critical parameter:
  larger `m` means faster confirmation but more bookkeeping.
  Practical `m`: 100-1000.
- Hash-target selection: the bits-of-hash mapping must be
  unbiased; standard Bitcoin sha256(sha256(header)) suffices.
- Proposer-block content selection: a proposer block references
  recent transaction blocks; ordering between transaction
  blocks within a proposer block can be canonical (by hash) or
  arbitrary (proposer's choice).

### Mining algorithm (proof-of-work function)

Prism uses *one* PoW function (double-SHA-256 in the paper's
prototype) shared across all three block types. The trick: a
single mining attempt produces a 256-bit hash, which is then
*sorted into one of the three block roles* by examining
specific bit ranges:

- Bits 0-7 below proposer-target threshold => proposer block.
- Bits 0-15 below voter-target threshold => voter block.
- Bits 0-23 below tx-target threshold => transaction block.

Targets are calibrated so block / voter / tx fractions match
the protocol's expected ratios. This *unified mining* design
avoids requiring miners to pick a role: one hash effort
contributes to whatever role its output happens to satisfy.

The Prism paper does not propose a custom hash; double-SHA-256
inherits Bitcoin's well-understood security analysis and
existing ASIC ecosystem.

## Verifiability and circuit encoding

**tag: `partial`.**

Prism circuits encode SHA-256 PoW for each of the three block
types, plus the voting-chain aggregation. Per-block SHA-256
costs are equivalent to Bitcoin (~30k constraints per hash). The
voting aggregation adds Merkle-membership proofs and depth
checks.

A SNARK light client for Prism is more complex than for
Bitcoin: it must verify the proposer chain *and* a sufficient
fraction of voter chains. Circuit cost is roughly `m`x Bitcoin's
verifier.

## Known attacks and limitations

- *Voter-chain DoS.* An attacker who specialises in voter blocks
  could try to bias votes; Prism's analysis bounds this
  probabilistically.
- *Proposer-block censorship.* A proposer can omit valid
  transaction blocks; subsequent proposers must include them or
  forfeit reward.
- *Parameter mis-tuning.* Wrong `m, k` breaks safety or kills
  throughput.

## References

- Bagaria, Kannan, Tse, Viswanath, Wang, "Prism: Deconstructing
  the Blockchain to Approach Physical Limits", CCS 2019.

## Implementation notes

The crate provides three block types (`ProposerBlock`,
`VoterBlock`, `TxBlock`) and a `Ledger` that stores them. Tests
verify each type can be appended and counted independently.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
