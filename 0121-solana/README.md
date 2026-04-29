# 0121: Solana

## Historical context

Anatoly Yakovenko published "Solana: A new architecture for a
high performance blockchain" (white paper, 2018). Solana
mainnet launched March 2020 with a unique consensus design
combining:

- *Proof-of-History (PoH).* A *verifiable delay function* (VDF)
  chain providing a global timestamp for transactions.
- *Tower BFT.* A PBFT-derivative voting protocol over PoH
  timestamps; modified to handle Solana's high block rate.

PoH's role is unusual: it is *not* consensus per se, but
a cryptographic clock. Validators agree on the order of events
(via PoH) before agreeing on which events are canonical (via
Tower BFT). This separation enables Solana's headline
throughput of ~50000+ tx/s.

## System and threat model

- **Network.** Bounded delay (PSS).
- **Failures.** Byzantine `f < n/3`.
- **Cryptography.** SHA-256 for PoH, Ed25519 signatures.
- **Goal.** High-throughput PoS with deterministic finality.

## Theory

### Proof-of-History

PoH is a sequential SHA-256 hash chain: `h_(i+1) =
sha256(h_i)`. Each step takes a fixed time on standard
hardware; the chain is *non-parallelisable* (a VDF). After `T`
steps, you can prove "at least `T` units of time elapsed"
because no one can compute the chain faster.

Validators *insert* transactions into the PoH chain at
specific positions. The position serves as a global
timestamp.

### Tower BFT voting

Tower BFT is PBFT with two optimisations:

1. *PoH-driven slot times.* Slots are determined by PoH, not
   physical time. All validators see the same slot ordering.
2. *Lockout doubling.* When a validator votes on a block,
   they commit to that block for `2^k` slots (lockout), where
   `k` is the depth in the chain. The lockout doubles per
   confirmation, giving an exponential commitment to the chain.

A block is *finalised* when 2/3 of stake-weighted lockouts
expire favouring it.

### Comparison: Solana vs Ethereum Gasper

| property              | Solana          | Ethereum Gasper |
| --------------------- | --------------- | --------------- |
| timestamping          | PoH (VDF)       | local clocks    |
| consensus             | Tower BFT       | LMD-GHOST + FFG |
| finality              | Tower BFT lockout | FFG checkpoints |
| block time            | 0.4 sec          | 12 sec         |
| validator count       | ~1500           | ~1M            |
| throughput            | ~50000 tx/s     | ~15 tx/s       |
| network requirement   | high bandwidth  | moderate       |

### Properties

- *Sub-second block times* via PoH.
- *High throughput* (~50000+ tx/s in benchmarks).
- *Deterministic finality* via Tower BFT lockouts.
- *Hardware-demanding.* High bandwidth (1Gbps+), large RAM,
  fast CPU.

### Limitations

- *Centralisation pressure.* High-end hardware requirement
  limits validator participation.
- *Network outages.* Solana has experienced multiple network
  halts (2021, 2022, 2024) due to high transaction volume
  spikes.
- *Stake concentration.* Largest validators control significant
  fraction of voting power.

### Subsequent work

- *Firedancer.* Independent Solana validator implementation
  by Jump Crypto for diversity.
- *QUIC transport.* Used in Solana for high-throughput
  message delivery.
- *Stake-weighted QoS.* Network-level prioritisation by stake.

## Practice

- *Solana mainnet beta.* Production since March 2020.
- ~1500 validators.
- Block time: 400ms.
- Throughput: ~50000 tx/s peak; ~3000-5000 tx/s typical.
- Used for high-frequency DeFi, NFTs, gaming.

### Production-implementation notes

- *PoH chain.* Each validator has its own PoH stream;
  transactions are inserted at specific PoH positions.
- *Vote distribution.* Validators submit votes as transactions
  on-chain.
- *Slashing.* Active for double-sign; less aggressive for
  downtime.

## Verifiability and circuit encoding

**tag: `partial`.**

PoH is SHA-256-based, sharing the same SNARK-unfriendliness as
Bitcoin. Tower BFT's vote aggregation uses Ed25519 signatures,
also not SNARK-friendly.

A SNARK light client for Solana would face high constraint
counts; practical bridges to Solana (Wormhole, Pyth) rely on
trust-minimised committees rather than SNARKs.

## Known attacks and limitations

- *Network DoS.* High-volume tx spikes have caused multiple
  outages.
- *Hardware centralisation.* Effective validator set smaller
  than nominal due to resource cost.
- *Stake-grinding.* Partially mitigated by VRF-style leader
  selection.

## References

- Yakovenko, "Solana: A new architecture for a high performance
  blockchain", 2018.
- Solana Labs, "Tower BFT", 2020.
- Pippenger, "On the evaluation of powers", 1976
  (VDF complexity background).

## Implementation notes

The crate provides a `PohChain` (sequential hash) and a
`TowerVote` lockout tracker. Tests verify the PoH chain
extends correctly and the lockout doubles per depth.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
