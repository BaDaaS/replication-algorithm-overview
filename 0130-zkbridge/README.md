# 0130: zkBridge

## Historical context

Tiancheng Xie, Jiaheng Zhang, Zerui Cheng, Fan Zhang, Yupeng
Zhang, Yongzheng Jia, Dan Boneh, and Dawn Song published
"zkBridge: Trustless Cross-chain Bridges Made Practical" at
CCS 2022. zkBridge is the canonical example of using zk-SNARKs
to verify *another chain's consensus* on this chain.

The motivation: cross-chain bridges are notoriously insecure
(Ronin Bridge $625M hack, Wormhole $325M hack, Nomad $190M
hack). Most bridges rely on a *trusted committee* (multi-sig
or BFT validators) which is a single point of failure.

zkBridge replaces the trusted committee with a *zk-SNARK*: the
relayer proves in zero-knowledge that "this block is finalised
on the source chain"; the destination chain verifies the SNARK
on-chain.

## System and threat model

- **Network.** Inter-chain bridges between any two chains.
- **Failures.** No trust assumption beyond cryptographic
  soundness of the SNARK and the source chain's consensus.
- **Cryptography.** Pairing-based zk-SNARK (Groth16 or PLONK).
- **Goal.** Trust-minimised cross-chain message-passing.

## Theory

### Light-client SNARK

Each block on the source chain has a header. The header
includes:

- Block hash.
- Parent hash.
- State root.
- Validator-set commitments.
- Aggregate signature (BLS or other).

A light client must verify the chain by checking signatures and
state-root proofs. zkBridge encodes this verification in a
SNARK circuit: the prover runs the verification logic, the
verifier (destination chain) checks the SNARK.

### Two phases

1. *Off-chain proving.* Relayers run the SNARK prover on the
   source chain's recent blocks.
2. *On-chain verification.* The destination chain verifies the
   SNARK proof in a smart contract.

### deVirgo accelerated proving

zkBridge introduced *deVirgo* (distributed Virgo): a
distributed prover system that speeds up SNARK generation by
parallel computation across multiple machines. Reduces
proof time from hours to minutes for typical chain headers.

### Comparison: bridge designs

| design               | trust assumption       | latency      | year |
| -------------------- | ---------------------- | ------------ | ---- |
| Multi-sig            | trusted m-of-n signers | seconds      | 2018 |
| Optimistic           | challenge period       | days         | 2020 |
| Light-client trust   | source chain validators | minutes     | 2021 |
| zkBridge             | SNARK soundness        | minutes      | 2022 |

zkBridge eliminates trust in human signers and the challenge
period; only relies on cryptographic assumptions.

### Properties

- *Trustless.* No trusted intermediaries.
- *Succinct verification.* On-chain verification is fast.
- *Universal.* Works between any two chains.
- *Pluggable.* Different SNARK systems for different chains.

### Limitations

- *Proving cost.* Generating proofs is expensive (CPU + memory).
- *Source-chain consensus complexity.* Proving SHA-256-based
  consensus (Bitcoin) is much harder than BLS-based.
- *Trusted setup.* Some SNARK systems require trusted setup.

### Subsequent work

- *Telepathy* (Succinct Labs). Production zkBridge for Ethereum.
- *Polymer.* zk-IBC for Cosmos.
- *Tendermint zkBridge.* Various academic attempts.
- *Mina to others.* Mina's recursive proofs as bridge basis.

## Practice

- *Telepathy* by Succinct Labs. Production Ethereum zk-bridge.
- *Polymer Labs.* Cosmos zk-IBC.
- *Galactica.* zk privacy-preserving bridges.
- Used by L2-to-L1 bridges, cross-chain messaging.

### Production-implementation notes

- *Proving infrastructure.* GPU-accelerated SNARK provers
  (cuBLAS, custom kernels).
- *Verifier contract.* Solidity precompile for BN254 or
  BLS12-381 verification.
- *Trusted setup.* Most SNARKs use ceremony-based trusted
  setup; PLONK with universal setup avoids per-circuit
  ceremony.

## Verifiability and circuit encoding

**tag: `deployed`.**

zkBridge *is* the verifiability paradigm. It encodes the
source chain's consensus in a SNARK circuit and verifies on
the destination chain.

Cost depends on the source chain's signature scheme:

- BLS-based (Ethereum, Cosmos with BLS): ~10^4 to 10^6
  constraints per epoch.
- ECDSA-based (Bitcoin, Cosmos default): ~10^6+ constraints
  per signature.

## Known attacks and limitations

- *Trusted setup.* If poisoned, all proofs are forgeable.
- *Implementation bugs.* SNARK circuits are notoriously easy
  to get wrong; audit required.
- *Source-chain consensus failure.* zkBridge inherits source
  chain's safety assumption.

## References

- Xie, Zhang, Cheng, Zhang, Zhang, Jia, Boneh, Song,
  "zkBridge: Trustless Cross-chain Bridges Made Practical",
  CCS 2022.
- Succinct Labs, "Telepathy: Bringing Trustless Verification to
  Smart Contracts", 2023.

## Implementation notes

The crate provides a stub `BridgeProof` and a
`verify_bridge_proof` function. Tests verify the API.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to 2024".
