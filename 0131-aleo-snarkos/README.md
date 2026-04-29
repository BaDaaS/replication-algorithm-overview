# 0131: Aleo / snarkOS

## Historical context

Aleo Systems launched mainnet in October 2024 with a
*privacy-preserving* SNARK-based blockchain. Aleo's
distinguishing features:

- *Programmable SNARKs.* Smart contracts execute as zk-SNARK
  proofs; only the proof is on-chain, not the inputs.
- *AleoBFT consensus.* A DAG-BFT consensus (modelled after
  Bullshark, module 0073) for ordering SNARK proofs.
- *Marlin SNARK.* Aleo uses Marlin (a universal-trusted-setup
  PLONK variant) for its core proof system.

Aleo combines privacy (transaction inputs hidden in zero-
knowledge) with programmable smart contracts (Leo language),
enabling a new class of applications: private DeFi, identity,
and more.

## System and threat model

- **Network.** Bounded-delay PSS.
- **Failures.** Byzantine `f < n/3`.
- **Cryptography.** Marlin SNARK, BLS12-377 aggregate signatures.
- **Goal.** Privacy-preserving smart contracts with deterministic
  finality.

## Theory

### Programmable SNARKs

Each transaction in Aleo:

1. *Off-chain execution.* Compiled Leo program executes locally;
   inputs are private.
2. *SNARK generation.* The program execution is wrapped in a
   Marlin SNARK proof.
3. *On-chain verification.* The validator network verifies the
   SNARK; only the public outputs (e.g., commitments) are
   published.

### AleoBFT consensus

AleoBFT is a Bullshark-style DAG-BFT:

1. Validators broadcast certified batches (Narwhal mempool).
2. Wave anchors are committed deterministically.
3. Certificates ordered topologically into the canonical chain.

This is a production deployment of DAG-BFT (cf. modules
0072-0079), specifically designed for SNARK-heavy workloads.

### Snark verification on-chain

Each AleoBFT block carries:

- Tx batch (each tx is a SNARK proof).
- Aggregated SNARK verification.
- BLS validator signatures.

Validators verify each transaction's SNARK before voting.

### Comparison: Aleo vs Mina vs Polygon zkEVM

| property             | Aleo                | Mina               | Polygon zkEVM     |
| -------------------- | ------------------- | ------------------ | ----------------- |
| privacy              | yes (programmable)  | partial (snapps)   | no                |
| smart contracts      | Leo (custom)        | snarky (custom)    | Solidity          |
| chain proof          | per-tx SNARK        | recursive chain    | per-block rollup  |
| consensus            | AleoBFT (DAG)       | Samasika (Praos)   | upstream Ethereum |
| SNARK system         | Marlin              | Pickles            | various           |
| year                 | 2024                | 2021               | 2023              |

### Properties

- *Privacy by default.*
- *Programmable.*
- *Production DAG-BFT.*
- *Universal trusted setup* (Marlin).

### Limitations

- *Proving cost.* Each transaction requires a SNARK proof
  generation (high CPU).
- *Validator hardware.* SNARK verification scales with
  transaction count.
- *Application complexity.* Leo language has steeper learning
  curve than Solidity.

### Subsequent work

- *Aleo improvement proposals.*
- *zkVM standardisation.* Cross-protocol zkVMs.
- *snarkVM evolution.* Ongoing optimisations.

## Practice

- *Aleo mainnet.* Production since October 2024.
- ~10-50 validators (initially).
- Block time: a few seconds.
- Privacy-preserving transactions.

## Verifiability and circuit encoding

**tag: `deployed`.**

Aleo's entire execution model is SNARK-based; verifiability is
the design centerpiece. Cross-chain bridges to Aleo can use
SNARK proofs natively.

## Known attacks and limitations

- *SNARK proof generation.* Slow; hardware-bound.
- *Trusted-setup risk.* Marlin uses universal setup; one
  ceremony covers all programs.
- *Application bugs.* Leo program errors can leak private data.

## References

- Aleo Systems, "Aleo: A Privacy-Preserving Blockchain", 2022.
- snarkVM specification, 2023.

## Implementation notes

The crate provides a `SnarkTx` stub and a `verify_tx` function
for the AleoBFT verification step. Tests verify the API.

See also [`HISTORY.md`](../HISTORY.md), section "2024 to 2026".
