# 0128: Hyperledger Fabric

## Historical context

Elli Androulaki and 28 co-authors published "Hyperledger
Fabric: A Distributed Operating System for Permissioned
Blockchains" at EuroSys 2018. Fabric is the flagship
*permissioned* blockchain platform, originally developed at
IBM and now under the Linux Foundation's Hyperledger umbrella.

Fabric's distinguishing feature is its *execute-order-validate*
architecture, which inverts the traditional *order-execute*
model used by most blockchains:

1. *Execute.* Endorsing peers run the transaction locally and
   produce a "read-write set" + endorsement signature.
2. *Order.* The ordering service (a separate consensus layer)
   sequences endorsed transactions into blocks.
3. *Validate.* All peers re-execute and validate transactions
   against current state, applying valid ones.

Fabric's ordering service is *pluggable*: production deployments
have used Kafka, Solo (development), and Raft (current default).
Future work may introduce SmartBFT for Byzantine ordering.

## System and threat model

- **Network.** Bounded delay PSS within a permissioned consortium.
- **Failures.** Crash-faults in current default (Raft);
  Byzantine in optional SmartBFT.
- **Cryptography.** ECDSA signatures, Membership Service
  Provider (MSP) for identity.
- **Goal.** Production permissioned BFT with privacy and
  flexibility.

## Theory

### Execute-Order-Validate

Traditional blockchains: *order-then-execute*. Order
transactions, then every peer executes them. Determinism is
mandatory (otherwise peers diverge).

Fabric inverts: *execute-then-order*. Endorsing peers execute
on their local view, then the ordering service sequences. This
allows non-deterministic chaincode (e.g., interacting with
external services) as long as endorsement consensus is reached.

### Endorsement policies

Each chaincode declares an *endorsement policy*: which peers
must endorse a transaction. E.g., "any 2 of 3 banks". This
is a fine-grained access control.

### Channels and privacy

Fabric supports *channels*: independent ledgers among subsets
of peers. Each channel has its own state and transactions;
non-channel peers cannot see channel data.

### Comparison: Fabric vs Cosmos vs Diem

| property              | Fabric           | Cosmos          | Diem (defunct)  |
| --------------------- | ---------------- | --------------- | --------------- |
| permissioned          | yes              | open            | yes (founded)   |
| consensus             | Raft (default)   | CometBFT        | DiemBFT (HS)    |
| privacy mechanism     | channels + MSP   | none            | none            |
| transaction ordering  | execute-order    | order-execute   | order-execute   |
| smart contracts       | chaincode        | CosmWasm/EVM    | Move            |
| year                  | 2017             | 2019            | 2019-2022       |

### Properties

- *Permissioned.* Membership controlled by MSP.
- *Privacy.* Channel-based data segregation.
- *Pluggable consensus.* Raft default; SmartBFT available.
- *Endorsement policies.* Fine-grained access control.

### Limitations

- *Permissioned only.* No open membership.
- *Crash-fault default.* Default Raft tolerates only crashes;
  Byzantine requires SmartBFT.
- *Performance overhead.* Three-phase EOV adds latency.

### Subsequent work

- *SmartBFT* (IBM Research). Byzantine ordering service for
  Fabric.
- *MIR-BFT* and others. Multi-leader BFT for higher throughput.
- *Fabric private chaincode.* SGX/TEE-based privacy.

## Practice

- *IBM Food Trust, IBM Blockchain Platform.* Production Fabric
  deployments for supply-chain, trade finance.
- *Linux Foundation Hyperledger.* Fabric is one of several
  Hyperledger frameworks (Besu, Iroha, Sawtooth).
- ~tens of organisations per consortium typical.

### Production-implementation notes

- *Raft default.* Single-orderer or 5-orderer Raft cluster.
- *MSP.* Membership Service Provider for identity (X.509-style).
- *Channels.* Each channel has own block stream.

## Verifiability and circuit encoding

**tag: `na`.**

Fabric is permissioned; SNARK light clients are not the
intended use case. Inter-organisation auditing relies on
shared ledger access rather than zero-knowledge proofs.

## Known attacks and limitations

- *Permissioned trust.* Consortium members have full access.
- *Channel-based privacy.* Peer must be channel member to see
  data.
- *Endorsement collusion.* If endorsement policy permits a
  small set of peers, they can collude.

## References

- Androulaki et al., "Hyperledger Fabric: A Distributed
  Operating System for Permissioned Blockchains", EuroSys 2018.
- Hyperledger Foundation, "Fabric documentation", 2017 onward.

## Implementation notes

The crate provides an `EndorsementPolicy` evaluator: given a
set of endorsing peer signatures, returns whether the policy
is satisfied. Tests verify simple m-of-n cases.

See also [`HISTORY.md`](../HISTORY.md), section "2014 to 2017".
