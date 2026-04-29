# 0117: Mina / Samasika

## Historical context

Joseph Bonneau, Izaak Meckler, Vanishree Rao, and Evan Shapiro
published "Mina: Decentralized Cryptocurrency at Scale" (Mina
white paper, 2020). Mina (formerly Coda) is the first
production blockchain with a *constant-size* state: the entire
chain is summarised by a single recursive zk-SNARK proof of
size ~22 KB, regardless of chain length.

Samasika is Mina's consensus protocol: a Praos-style proof-of-
stake chain (with VRF leader election and stake-weighted
slot lottery) plus a *Genesis-style chain-density tie-breaker*
(module 0112), all encoded in a recursive Pickles SNARK
(Mina's custom proof system). Each block carries a SNARK
proof attesting that the entire chain history is valid,
including stake distribution, VRF outputs, signatures, and
the Genesis density rule.

Mina mainnet launched March 2021. The succinct-state property
makes Mina the canonical example of *verifiable consensus*:
any device, even a phone, can verify the entire chain in
milliseconds.

## System and threat model

- **Network.** Bounded-delay PSS.
- **Failures.** Adaptive Byzantine `< 1/2` honest stake.
- **Cryptography.** Pickles SNARK (recursive Plonk-style proof
  system on the Pasta cycle of curves); Schnorr signatures on
  Pasta.
- **Goal.** Constant-size verifiable PoS chain.

## Theory

### Recursive SNARK

Each block carries a *transition proof*: a SNARK that proves
"there exists a valid predecessor block + transactions that
extends the chain to this state". The SNARK includes a
verifier circuit for the *previous* block's SNARK, making the
proof recursive.

Result: a single 22 KB proof attests the entire chain's
validity, regardless of chain depth.

### Samasika consensus

Samasika is the Praos + Genesis combination encoded in the
SNARK:

- *Praos* slot-leader VRF lottery.
- *Genesis* chain-density tie-breaker.
- *Schnorr* signatures on Pasta curves (SNARK-friendly).

Each block-producer evaluates VRF on their slot+private key;
if eligible, they produce a block + recursive SNARK proof.

### Density rule in SNARK

The Genesis chain-density predicate (module 0112) is encoded
in the SNARK: given two competing forks, a verifier can prove
in zero-knowledge that one has higher density.

### Properties

- *Constant-size proof.* ~22 KB regardless of chain length.
- *Trustless light clients.* Verify chain in milliseconds.
- *Praos-equivalent security.* CP / CG / CQ from Genesis
  encoded in the SNARK.
- *Adaptive corruption resistance* (forward-secure VRF + KES
  in SNARK).

### Comparison: succinct PoS

| protocol  | proof size  | cycle of curves      | year |
| --------- | ----------- | -------------------- | ---- |
| Mithril   | ~kB         | BLS12-381            | 2022 |
| Mina      | ~22 KB      | Pasta (Pallas/Vesta) | 2021 |
| zkBridge  | ~MB         | BLS12-381            | 2022 |
| Aleo      | varies      | snarkVM curves       | 2024 |

Mina's recursive Pickles is the only production system
achieving constant-size full-chain proofs.

### Implementation: Pickles + Pasta

- *Pickles.* Recursive Plonk-style proof system on the Pasta
  cycle.
- *Pasta cycle.* Pallas and Vesta curves; their scalar fields
  are each other's base fields, enabling efficient recursion.
- *Snarky.* OCaml DSL for writing SNARK circuits.

### Subsequent influence

- *Polygon zkEVM.* Recursive SNARKs for L2 rollup.
- *Risc Zero.* General-purpose recursive SNARKs.
- *Aleo.* Privacy-preserving SNARK chain.

## Practice

- *Mina mainnet.* Production since March 2021.
- ~1500 block producers.
- Block time 3 minutes; constant-size proof at every block.
- Used for tokenisation, identity, voting.

### Production-implementation notes

- *Block production.* Each producer maintains the most recent
  Pickles proof; the chain proof is updated incrementally.
- *Light clients.* Mina's mobile app verifies the entire chain
  in milliseconds via the constant-size proof.
- *Stake distribution.* Encoded as Merkle accumulator;
  membership proofs are part of the chain SNARK.

## Verifiability and circuit encoding

**tag: `deployed`.**

Mina is *the* deployed example of verifiable consensus. The
chain protocol itself is encoded in the SNARK; every block
carries a proof that the entire history is valid.

This is in stark contrast to Bitcoin and Ethereum: Bitcoin
SNARK light clients are at the research stage; Ethereum's
Beacon Chain has SNARK proofs of state but not consensus;
Mina has both.

## Known attacks and limitations

- *SNARK trusted setup.* Pickles uses no trusted setup.
- *Stake centralisation.* Same as all PoS.
- *Adversarial committee.* Mitigated by Praos's stake-weighted
  lottery.
- *Pickles soundness.* Relies on Plonk and Pasta curve security.

## References

- Bonneau, Meckler, Rao, Shapiro, "Mina: Decentralized
  Cryptocurrency at Scale", 2020.
- Bowe, Chiesa, Hopwood, Hughes, Vesely, "Recursive Proof
  Composition without a Trusted Setup", 2019.
- Mina Foundation, "Samasika Consensus Specification", 2021.

## Implementation notes

The crate provides a stub `RecursiveProof` and a
`verify_chain` function that takes a block-history and
returns whether a (placeholder) proof would validate. Tests
verify the API surface.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to 2024".
