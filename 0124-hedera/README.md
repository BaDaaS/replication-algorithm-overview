# 0124: Hedera

## Historical context

Hedera Hashgraph is the production deployment of the
*Hashgraph* consensus algorithm (Baird, module 0069) by
Hedera Hashgraph LLC (founded 2018 by Mance Harmon and Leemon
Baird). Hedera mainnet launched in 2019 and operates with a
*permissioned* governing council of ~30 enterprise validators
(IBM, Google, Boeing, Tata, and others).

Hedera is the first production-scale deployment of an
asynchronous DAG-BFT consensus. Its differentiating claims:

- *Asynchronous Byzantine fault tolerance* (gossip-about-gossip).
- *Bounded latency* (~3-5 seconds finality).
- *High throughput* (~10000 tx/s sustainable).
- *Patented algorithm* (Hashgraph; Swirlds Inc.).

The patented status has limited Hashgraph adoption outside
Hedera itself; the consensus design is otherwise an important
example of asynchronous DAG-BFT in production.

## System and threat model

- **Network.** Asynchronous (no timing assumption for safety).
- **Failures.** Byzantine `f < n/3`.
- **Cryptography.** Ed25519 signatures.
- **Goal.** Asynchronous BFT with permissioned validator set.

## Theory

### Hashgraph (recap)

Each node maintains a *hashgraph*: a DAG of events. Events are
gossiped between nodes; each event records the sender,
the parent (own previous event), and the *other-parent* (the
remote sender's latest event).

*Famous witnesses* are events that "see" 2/3 of the gossip
in their virtual round. Once a witness is famous, it provides
a *deterministic ordering* of all events in earlier rounds.

### Production specifics

Hedera adds:

- *Permissioned governing council.* ~30 enterprise validators,
  rotating; geographically distributed.
- *Stake-weighted voting.* Each validator weight proportional
  to staked HBAR.
- *Service split.* Cryptocurrency, smart contracts, file
  storage, and consensus services as separate APIs.
- *Throughput sharding.* Plans for permissionless validator
  participation post-2024.

### Comparison: Hedera vs Cosmos vs Aptos

| property            | Hedera          | Cosmos         | Aptos           |
| ------------------- | --------------- | -------------- | --------------- |
| consensus           | Hashgraph       | CometBFT       | AptosBFT        |
| network model       | async           | partial sync   | partial sync    |
| validator count     | ~30 (council)   | ~150           | ~150            |
| permissioned        | yes (council)   | no             | no              |
| latency             | 3-5 sec         | 6 sec          | 0.5-1 sec       |
| patented            | yes (Swirlds)   | no             | no              |
| year                | 2019            | 2019           | 2022            |

### Properties

- *Async safety* (no fork ever).
- *Bounded latency.*
- *DAG-BFT throughput.*
- *Patented* (limits adoption).

### Limitations

- *Permissioned validator set.* Not fully decentralised.
- *Patent.* Hashgraph algorithm is patented; Hedera operators
  use it under license.
- *Council governance.* Council determines protocol upgrades.

### Subsequent work

- *Hedera Improvement Proposals (HIPs).* Protocol governance.
- *Validator decentralisation.* Plans for permissionless
  participation.
- *Modern DAG-BFT* (Narwhal-Bullshark, Mysticeti) inherits
  ideas without patent restrictions.

## Practice

- *Hedera mainnet.* Production since 2019.
- ~30 council validators.
- Throughput: ~10000 tx/s sustained.
- Latency: 3-5 seconds finality.
- Used for enterprise applications, NFTs, supply-chain.

## Verifiability and circuit encoding

**tag: `partial`.**

Ed25519 signatures are not SNARK-friendly; Hashgraph DAG
verification is also non-trivial. Hedera does not currently
support light-client SNARK proofs.

## Known attacks and limitations

- *Council collusion.* If `> 1/3` of council validators
  collude, safety can fail.
- *Patent risk.* Implementations require Swirlds licensing.
- *Centralised governance.* Hedera Council controls upgrades.

## References

- Baird, "The Swirlds Hashgraph Consensus Algorithm: Fair, Fast,
  Byzantine Fault Tolerance", Swirlds, 2016.
- Hedera Foundation, "Hedera Whitepaper", 2018.

## Implementation notes

The crate provides a stub `Council` validator-set type. Tests
verify the size limit.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
