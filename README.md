# Replication and Consensus Algorithms: A Graduate Course

A comprehensive, hands-on graduate course covering the theory,
practice, and formalisation aspects of replication and consensus
algorithms, from the impossibility results of the 1980s to the
DAG-based and proof-of-stake protocols of the 2020s.

## Audience

Graduate students in computer science and mathematics, with comfort
in:

- Discrete probability and asymptotic analysis.
- Modal and temporal logic at the level of Pnueli 1977 and
  Manna-Pnueli (LTL `G`, `F`, `X`, `U`; the Alpern-Schneider
  safety/liveness decomposition).
- Cryptographic security games (existential unforgeability, IND-CPA,
  random-oracle and standard models).
- Distributed-systems vocabulary (processes, channels, schedulers,
  failures).
- Reading research papers and proof sketches.

## How to Use

Each module is a directory `NNNN-protocol-name/` containing:

- `README.md`: lecture notes structured into Historical context,
  System and threat model, Theory, Practice, Formalisation aspects,
  Known attacks, Implementation notes, References.
- `Cargo.toml`, `src/lib.rs`, `src/sim.rs` or `tests/`: a Rust
  reference implementation skeleton with simulator-driven property
  tests.
- `Exercises.md`: 4 to 8 exercises (theory, practice, formalisation).
- `Solutions.md` or `solutions/`: reference solutions.

Modules are sequential. Take them in order on a first pass. Once
through, revisit by family.

## Four Pillars

Every module gives equal weight to:

1. **Theory.** Formal system model, definitions, theorem statements,
   proofs or proof sketches, complexity bounds, lower bounds.
2. **Practice.** How real implementations diverge from the paper:
   pipelining, signature aggregation, view-change shortcuts, gossip
   layers, mempool separation, batching, threshold encryption,
   timeouts, leader rotation, configuration changes, snapshotting.
   Concrete codebase pointers to CometBFT, Aptos-core, Sui, Solana,
   libp2p, Geth, Prysm, Lighthouse, Cardano-node, Polkadot-sdk,
   Hyperledger Fabric.
3. **Formalisation aspects.** What the protocol's state, transition
   relation, adversary, and security properties look like as
   mathematical objects suitable for a proof assistant. Suggested
   types, invariants, theorems, and reuse of existing Lean +
   Mathlib + cslib abstractions. No Lean code in this course.
4. **Verifiability and circuit encoding.** Whether the protocol's
   transitions, fork-choice rule, and finality predicate can be
   encoded efficiently in a SNARK or STARK circuit, and how
   production systems (Mina via Pickles, Aleo, zk-rollup sequencer
   consensus, zk-bridges) realise that encoding. Each module
   carries a verifiability tag: `na`, `friendly`, `partial`, or
   `deployed`.

The phrase **verifiable replication algorithm** is reserved here
for a state-machine replication protocol whose state-transition
relation, fork-choice rule, and finality predicate admit a succinct
proof of correct execution that a light client can verify in time
polylogarithmic in the chain length and the validator-set size.
Part XIII is dedicated to this notion.

## Course Outline

The course is organised into 14 parts. See [`PROMPT.md`](PROMPT.md)
for the generation specification and the full module list.

| Part  | Theme            | Title                                          |
| ----- | ---------------- | ---------------------------------------------- |
| I     | Foundations      | Foundations and impossibilities                |
| II    | Failure & rand.  | Failure detectors and randomised consensus     |
| III   | Crash SMR        | Crash-fault state-machine replication          |
| IV    | Classical BFT    | Classical Byzantine fault tolerance (1994-14)  |
| V     | Streamlined BFT  | Streamlined and pipelined BFT (HotStuff fam.)  |
| VI    | Async BFT        | Asynchronous BFT (HoneyBadger family)          |
| VII   | DAG BFT          | DAG-based BFT (Aleph through Mysticeti+)       |
| VIII  | PoW              | Nakamoto-style and proof-of-work               |
| IX    | FBA & Avalanche  | Federated Byzantine agreement and Avalanche    |
| X     | PoS gadgets      | Proof of stake: foundations and gadgets        |
| XI    | Ouroboros        | Ouroboros family                               |
| XII   | Production       | Production blockchain consensus                |
| XIII  | Verifiable       | Verifiable replication and SNARK-friendly cons |
| XIV   | Capstones        | Capstones, meta-theory, and open problems      |

See [`HISTORY.md`](HISTORY.md) for the narrative timeline and
[`lore/`](lore/) for shorter pieces (etymology, anecdotes,
side-quests).

## Prerequisites

- Rust toolchain (1.92 or later) via `rustup`. Edition 2024 is used
  throughout.
- A unix-like shell with `make`.
- Optional: `gh` for GitHub interactions.

## Building

```sh
make help          # list targets
make build         # debug build
make test          # full test suite
make check         # format check, lint, and tests
make doc-open      # browse the Rust docs
```

The shared simulator lives at `crates/sim/`. Each module crate
depends on it. Module crates are named `replication-NNNN-name`.

## Implementation Notes

- All implementations are deliberately small and pedagogical, not
  production-grade. They are written to map clearly to the paper's
  notation and to the future Lean formalisation.
- Tests use the deterministic simulator with a pluggable adversary
  (drop, delay, reorder, partition, equivocate). Property tests
  assert the safety, validity, and termination conditions stated in
  the original papers.
- Cryptographic primitives are stubbed where the protocol's
  correctness does not depend on them; otherwise, approved crates
  from `~/.claude/rules/dependencies.md` are used.

## Formalisation Aspects

The course is written so that each module's theory section can be
formalised in Lean 4 with Mathlib and cslib. Each module's
"Formalisation aspects" section identifies:

- The state and message types.
- The transition relation as a labelled transition system or I/O
  automaton.
- The adversary as a typed scheduler.
- The safety and liveness theorems with explicit hypotheses.
- The cryptographic assumptions stated as adversary advantages.
- Suggested reuse of cslib's `LTS`, `FLTS`, `InferenceSystem`,
  `HasFresh`, and Mathlib's algebraic and probabilistic
  infrastructure.

The actual Lean formalisation is a separate project planned to
follow this course. The course itself contains no Lean code.

## License

Apache-2.0. See [LICENSE](LICENSE).

## References

The complete bibliography lives in [`references.bib`](references.bib).
Each module's `README.md` lists its primary citations inline.
