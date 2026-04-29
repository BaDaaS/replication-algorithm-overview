# Course Generation Prompt: Replication and Consensus Algorithms

## Role

Act as a graduate-level instructor teaching a year-long course in
distributed consensus to students with strong backgrounds in
mathematics and computer science. Assume the audience is comfortable
with:

- Discrete probability, game theory basics, and asymptotic analysis.
- Modal and temporal logic at the level of Pnueli 1977
  ("The Temporal Logic of Programs", FOCS) and Manna-Pnueli
  ("Temporal Verification of Reactive Systems"); familiarity with
  the LTL modalities `G`, `F`, `X`, `U` and with the
  Alpern-Schneider safety/liveness decomposition.
- Cryptographic security games (existential unforgeability, IND-CPA,
  random-oracle and standard models).
- Concurrent and distributed systems vocabulary (processes, channels,
  failures, schedulers, adversaries).
- Reading research papers and proof sketches.

Do not condescend, but do not assume background outside the above.
When a result has a clean proof, include it. When the proof is too
long, give the proof sketch and cite the section in the paper.

## Goal

Build a complete, self-contained graduate course at
`/Users/soc/codes/badaas/replication-algorithm-overview/`. The course
covers the theory and practice of every significant consensus or
state-machine-replication protocol from the early impossibility
results through the protocols published in 2024 to 2026. The course
mirrors the layout of `/Users/soc/codes/badaas/lean4-courses/`:
sequential numbered directories, each a self-contained module with
lecture notes, exercises, solutions, and a Rust reference
implementation skeleton.

The eventual goal of the broader research programme is to formalise
these protocols in Lean 4, but the Lean formalisation is not part of
this course. The course must, however, present the theory in a form
that is ready for later formalisation: precise types, named
theorems, hypotheses and conclusions stated explicitly, and proofs
broken into lemmas with stated invariants.

## Four pillars per module

Every module must give equal weight to four pillars:

1. **Theory.** Formal system model, definitions, theorem statements,
   proofs or proof sketches, complexity bounds, lower bounds, and
   reductions. This is the part written for a future Lean
   formalisation.
2. **Practice.** How real implementations differ from the paper:
   pipelining, signature aggregation, view-change shortcuts, gossip
   layers, mempool separation, batching, threshold encryption,
   timeouts, leader rotation, configuration changes, snapshotting,
   monitoring. Reference real codebases (CometBFT, Aptos-core, Sui,
   Solana, libp2p, Geth, Prysm, Lighthouse, Cardano-node,
   Polkadot-sdk, Hyperledger Fabric ordering) and document the
   tweaks they apply.
3. **Formalisation aspects.** What the protocol's state, transition
   relation, adversary, and security properties look like as
   mathematical objects suitable for a proof assistant. List the
   types, the invariants, the safety theorem, the liveness theorem,
   the resilience bound, and any cryptographic assumptions as
   explicit hypotheses. Do not write Lean code. Do write
   pseudo-mathematical signatures (in plain ASCII) that map cleanly
   to Lean later.
4. **Verifiability and circuit encoding.** Whether the protocol's
   transition function, fork-choice rule, and security checks can
   be encoded efficiently in a SNARK or STARK circuit. Identify the
   primitives the protocol needs that are not SNARK-friendly out of
   the box (Merkle hashes, signatures, VRFs, threshold crypto,
   randomness beacons) and the SNARK-friendly substitutes used in
   practice (Poseidon, Rescue, Griffin, Anemoi, Schnorr over Pasta,
   BLS over BLS12-381, in-circuit VRFs, threshold-BLS DKG circuits).
   Note known production deployments that recursively prove protocol
   execution (Mina via Pickles + Halo 2, Aleo via snarkVM,
   zk-rollup sequencer consensus, zk-bridges and light clients such
   as zkBridge, Succinct's Telepathy, Polyhedra). Tag the module
   with a verifiability rating:
   - `na`: not applicable (foundational result, no protocol to encode);
   - `friendly`: the protocol's structure admits an efficient
     SNARK encoding with only standard substitutions;
   - `partial`: parts of the protocol can be proved (e.g. the
     fork-choice rule but not the network layer);
   - `deployed`: a production system already publishes recursive
     proofs of the protocol's execution.

The phrase **verifiable replication algorithm** is reserved in this
course for a state-machine replication protocol whose state-
transition relation, fork-choice rule, and finality predicate admit
a succinct (SNARK or STARK) proof of correct execution that a light
client can verify in time polylogarithmic in the chain length and
the validator-set size. Part XIII develops this notion in depth.

## Format

### Top-level files

- `README.md`: course overview, prerequisites, navigation, and
  recommended reading order. Includes a graph of dependencies between
  modules.
- `HISTORY.md`: 5 to 10 page narrative timeline (see below).
- `references.bib`: every cited paper and survey.
- `Cargo.toml`: workspace declaration listing every module crate and
  the shared simulator crate.
- `Makefile`: standard targets (`help`, `build`, `test`, `lint`,
  `format`, `check`). Wrap lines at 80 chars. Use tabs for recipes.
- `.editorconfig`, `.gitignore`, `.prettierrc.json`, `rustfmt.toml`,
  `clippy.toml`.
- `crates/sim/`: shared deterministic network simulator (process
  abstraction, message bus, virtual clock, network adversary capable
  of dropping, delaying, reordering, partitioning, and equivocating).
- `lore/`: short pieces with historical anecdotes, etymology, and
  side-quests; each lore file is self-contained and linked from the
  relevant module READMEs (mirroring `lean4-courses/lore/`).

### Per-module directories

Each module is `NNNN-protocol-name/` with:

- `README.md`: lecture notes, structured as below.
- `Cargo.toml`: a library crate `replication-NNNN-protocol-name`.
- `src/lib.rs`: minimal protocol skeleton (public types, message
  enums, a step function or actor loop). Faithful to the paper's
  abstraction, not production-grade.
- `src/sim.rs` or `tests/`: simulator-driven property tests for the
  safety, validity, and termination claims under the protocol's
  stated assumptions.
- `Exercises.md`: 4 to 8 exercises, increasing in difficulty.
  Include theory exercises (prove a lemma, establish a bound),
  practice exercises (extend the simulator, add a tweak), and
  formalisation-readiness exercises (write the safety invariant as a
  predicate, identify the proof obligations).
- `Solutions.md` and/or a `solutions/` directory with reference code.

### `README.md` per module structure

```markdown
# NNNN: Protocol Name

## Historical context

When was this proposed, by whom, in response to what prior work, and
why it mattered. 2 to 4 paragraphs.

## System and threat model

- Synchrony: synchronous, partially synchronous (DLS 1988), or
  asynchronous.
- Failure model: crash-stop, crash-recover, omission, Byzantine, or
  Byzantine with equivocation, adaptive vs static corruption.
- Network: reliable, FIFO, authenticated, public-key infrastructure,
  random oracle if applicable.
- Cryptographic assumptions: hash collision resistance, signature
  unforgeability, threshold signatures, VRFs, zero-knowledge proofs.
- Resilience bound: n, f, and the tolerated ratio.

## Theory

- Formal definition of the consensus problem this protocol solves
  (agreement, validity, integrity, termination), with the precise
  variant: weak vs strong validity, eventual vs probabilistic
  termination, etc.
- Protocol description in pseudo-code, ASCII only, with line-numbered
  steps.
- Statement of safety theorem with hypotheses and conclusion.
- Proof or proof sketch (with citation to the paper's section).
- Statement of liveness theorem (or weaker progress condition) with
  hypotheses (synchrony assumption, leader assumption, fairness).
- Proof or proof sketch.
- Round complexity, message complexity, authenticator complexity,
  and where the bound matches a known lower bound.

## Practice

- Real-world implementations and the tweaks they apply.
- Diagram or table of differences between the paper and at least one
  production codebase.
- Performance numbers if published (TPS, finality latency, recovery
  time, signature size).
- Operational concerns: configuration changes, snapshotting,
  upgrades, equivocation slashing, MEV, censorship resistance.
- Known production incidents or postmortems with citations.

## Formalisation aspects

- State type, message type, transition relation as a labelled
  transition system or I/O automaton, in ASCII pseudo-mathematics.
- Invariants, indexed by phase or round, that imply safety.
- The cryptographic assumption stated as a hypothesis on the
  adversary's success probability, not as a primitive.
- Adversary model as a typed scheduler.
- Suggested formalisation route in Lean 4 + Mathlib + cslib (which
  existing abstractions to reuse: LTS, FLTS, InferenceSystem,
  HasFresh, etc.). No Lean code.
- Open formalisation problems and pointers to existing verified
  consensus work (IronFleet, Verdi, Velisarios, Bythos, the
  Tendermint Coq spec).

## Verifiability and circuit encoding

- Tag: `na`, `friendly`, `partial`, or `deployed` (defined above).
- Which non-arithmetic operations the protocol uses (collision-
  resistant hashes, EdDSA / ECDSA / BLS signatures, VRFs, sortition,
  randomness beacons, network gadgets) and the SNARK-friendly
  substitutes that production systems pick (Poseidon / Rescue /
  Griffin / Anemoi for hashing, Schnorr over Pasta or BLS over
  BLS12-381 / 377 for signatures, in-circuit VRF constructions).
- Estimated proving cost per block or per round, in number of
  constraints, when published numbers exist.
- Whether recursive composition (Halo, Halo 2, Nova, ProtoStar,
  HyperNova, Mariposa, Pickles) is needed for constant-size light-
  client proofs, and how it is used.
- Production examples and deployment status: Mina (Pickles plus
  Ouroboros Samasika), Aleo (snarkOS), zk-rollup sequencer
  consensus, zkBridges (zkBridge, Telepathy, Polyhedra Expander).
- Open problems that the formal model would need to clarify before a
  faithful circuit encoding is possible.

## Known attacks and limitations

What can go wrong, scope boundaries, and what later protocols fixed.

## Implementation notes

Pointers from `src/lib.rs` types to paper notation: which struct is
the paper's state machine, which function is the paper's procedure,
where the proof's invariants would attach.

## References

- Primary paper(s) with link.
- Follow-up work, surveys, and known formalisations.
- Related lore file(s) in `lore/`.
- Implementation references (specific files in production
  codebases).
```

## Course Outline

The course is organised into 14 parts. Module numbers are guidelines;
the generator may merge or split modules, but every protocol listed
below must appear as a module with its own crate, README,
exercises, and solutions. Use four-digit module numbers.

### Part I. Foundations and impossibilities

Audience: builds the language used for the rest of the course.

Modules:

- Introduction to state machine replication and the consensus
  problem (Schneider 1990).
- System models: synchrony, partial synchrony, asynchrony
  (Dwork-Lynch-Stockmeyer 1988).
- Failure models: crash, omission, Byzantine, mobile, rational
  (Cristian, Aguilera, Gafni, Eyal-Sirer).
- The Two Generals problem (Akkoyunlu-Ekanadham-Huber 1975).
- Byzantine Generals (Lamport-Shostak-Pease 1982; Pease-Shostak-Lamport
  1980).
- FLP impossibility (Fischer-Lynch-Paterson 1985), with full proof.
- Dolev-Strong round-complexity lower bound (1983).
- Dolev-Reischuk message-complexity lower bound (1985).
- CAP and PACELC (Brewer 2000, Gilbert-Lynch 2002, Abadi 2012).
- Reliable, causal, and atomic broadcast; equivalence to consensus
  (Hadzilacos-Toueg 1993).
- Quorum systems and intersection (Malkhi-Reiter 1998, Vukolic 2012).
- Cryptographic prerequisites: hashes, signatures, threshold
  signatures, BLS aggregation, VRFs, common coins.

### Part II. Failure detectors and randomised consensus

- Chandra-Toueg failure detectors and the weakest detector (1996).
- Ben-Or randomised async consensus (1983).
- Rabin's randomised Byzantine agreement (1983).
- Bracha's async Byzantine broadcast and ABA (1984, 1987).
- Cachin-Kursawe-Petzold-Shoup (CKPS) async BFT (2001).
- Mostefaoui-Moumen-Raynal signature-free async ABA (2014, 2015).
- Common-coin constructions from threshold signatures and from VRFs.

### Part III. Crash-fault state-machine replication

Cover each of:

- Two-phase commit and three-phase commit.
- Viewstamped Replication (Oki-Liskov 1988).
- Viewstamped Replication Revisited (Liskov-Cowling 2012).
- Paxos: Synod and Part-Time Parliament (Lamport 1989, 1998).
- Paxos Made Simple (Lamport 2001).
- Multi-Paxos.
- Cheap Paxos (Lamport-Massa 2004).
- Fast Paxos (Lamport 2006).
- Generalized Paxos (Lamport 2005).
- Mencius (Mao-Junqueira-Marzullo 2008).
- EPaxos (Moraru-Andersen-Kaminsky 2013).
- Stoppable Paxos and Vertical Paxos.
- Disk Paxos (Gafni-Lamport 2003).
- Flexible Paxos (Howard-Malkhi-Spiegelman 2016).
- Compartmentalized Paxos (Whittaker et al. 2021).
- Raft (Ongaro-Ousterhout 2014).
- ZAB (Junqueira-Reed-Serafini 2011).
- Chain Replication (van Renesse-Schneider 2004).
- CRAQ (Terrace-Freedman 2009).
- Spanner-style replication and TrueTime (Corbett et al. 2012).
- CASPaxos and Atlas (geo-distributed Paxos).

### Part IV. Classical Byzantine fault tolerance (1994 to 2014)

- Rampart (Reiter 1994).
- SecureRing (Kihlstrom 1998).
- PBFT (Castro-Liskov 1999, 2002), with the safety and liveness
  proofs in detail.
- Q/U (Abd-El-Malek et al. 2005).
- HQ Replication (Cowling et al. 2006).
- Zyzzyva and Zyzzyva5 (Kotla-Alvisi-Dahlin 2007, 2008).
- Aardvark robust BFT (Clement et al. 2009).
- UpRight (Clement et al. 2009).
- Spinning (Veronese et al. 2009).
- Prime (Amir et al. 2010).
- Steward (Amir et al. 2010, hierarchical BFT).
- A2M and TrInc trusted hardware (Chun 2007, Levin 2009).
- MinBFT and MinZyzzyva (Veronese et al. 2013).
- BFT2F (Li-Mazieres 2007).
- BFT-SMaRt (Bessani-Sousa-Alchieri 2014).

### Part V. Streamlined and pipelined BFT (2014 to 2024)

- Tendermint (Buchman 2014, Buchman-Kwon-Milosevic 2018) and
  CometBFT.
- HotStuff: basic and chained (Yin et al. 2019), with the linearity,
  responsiveness, and optimistic responsiveness arguments.
- LibraBFT and DiemBFT v1 to v4 (2019 to 2021).
- Wendy.
- Jolteon (Gelashvili et al. 2021).
- Fast-HotStuff (Jalalzai et al. 2020).
- HotStuff-2 (Malkhi-Nayak 2023).
- Streamlet (Chan-Pass-Shi 2020).
- Pala and Pili (Chan 2018, 2019).
- Sync HotStuff (Abraham et al. 2019).
- SBFT (Gueta et al. 2019, IBM).
- Ditto (asynchronous fallback for HotStuff, 2021).

### Part VI. Asynchronous BFT

- HoneyBadger BFT (Miller et al. 2016).
- BEAT family (Duan-Reiter-Zhang 2018).
- Dumbo and Dumbo2 (Guo et al. 2020).
- Speeding Dumbo (Guo et al. 2022).
- Asynchronous fallbacks: Ditto, Bullshark async fallback.

### Part VII. DAG-based BFT

- HashGraph (Baird 2016).
- Aleph (Gagol et al. 2019).
- DAG-Rider (Keidar-Kokoris-Kogias-Naor-Spiegelman 2021).
- Narwhal and Tusk (Danezis et al. 2022).
- Bullshark (Spiegelman-Giridharan-Sonnino-Kokoris-Kogias 2022).
- Cordial Miners (Keidar et al. 2023).
- Shoal (Spiegelman et al. 2023).
- Mysticeti (Babel et al. 2024).
- Mysticeti-FPC (fast path commit).
- Mahi-Mahi (2024).
- Sailfish (2024).
- Shoal++ (2024).
- Autobahn (Giridharan et al. 2024).
- BBCA-chain and Starfish (2024 and later).
- Motorway and follow-ups (2025).

### Part VIII. Nakamoto-style and proof-of-work

- Bitcoin (Nakamoto 2008), with the longest-chain rule and the 51%
  argument made precise.
- The Bitcoin backbone protocol (Garay-Kiayias-Leonardos 2015,
  2017).
- Sleepy consensus (Pass-Shi 2017).
- GHOST (Sompolinsky-Zohar 2015).
- Bitcoin-NG (Eyal et al. 2016).
- Inclusive blockchain protocols (Lewenberg et al. 2015).
- Selfish mining (Eyal-Sirer 2014).
- ByzCoin (Kogias et al. 2016).
- Solida (Abraham et al. 2016).
- Hybrid Consensus (Pass-Shi 2017).
- Thunderella (Pass-Shi 2018).
- FruitChains (Pass-Shi 2017).
- SPECTRE (Sompolinsky-Lewenberg-Zohar 2016).
- PHANTOM and GhostDAG (Sompolinsky-Wyborski-Zohar 2018, 2020).
- Prism (Bagaria et al. 2019).
- OHIE (Yu et al. 2020).
- Conflux (Li et al. 2020).

### Part IX. Federated Byzantine agreement and Avalanche

- Stellar Consensus Protocol (Mazieres 2016) and federated voting.
- Ripple consensus protocol (Schwartz-Youngs-Britto 2014, with later
  analyses).
- Avalanche, Snowball, Snowman, Frosty (Rocket et al. 2018, 2019,
  2024).

### Part X. Proof of stake: foundations and gadgets

- Long-range attacks, nothing-at-stake, stake grinding.
- Slasher (Buterin 2014).
- Casper FFG (Buterin-Griffith 2017).
- Casper CBC (Zamfir et al. 2018).
- Algorand (Micali 2016, Chen-Micali 2017, Gilad et al. 2017).
- Snow White (Daian-Pass-Shi 2019).

### Part XI. Ouroboros family

- Ouroboros Classic (Kiayias-Russell-David-Oliynykov 2017).
- Ouroboros Praos (David-Gazi-Kiayias-Russell 2018).
- Ouroboros Genesis (Badertscher-Gazi-Kiayias-Russell-Zikas 2018).
- Ouroboros Crypsinous (Kerber-Kiayias-Kohlweiss-Zikas 2019).
- Ouroboros Chronos (Badertscher et al. 2019).
- Ouroboros Leios (recent throughput-oriented variant).
- Ouroboros Peras (recent fast-finality variant).
- Mina Samasika.

### Part XII. Production blockchain consensus

For each, dissect what the production stack actually runs, including
the signature scheme, leader selection, mempool, finality gadget,
and any divergence from the paper:

- Ethereum: Gasper (LMD-GHOST + Casper FFG, Buterin et al. 2020),
  proposer-builder separation, MEV-Boost, inclusion lists, and
  3-slot or single-slot finality proposals.
- Cosmos and CometBFT: Tendermint plus ABCI++, vote extensions.
- Polkadot: BABE block production, GRANDPA finality (Stewart-Kokoris-
  Kogias 2020), BEEFY bridge finality.
- Cardano: Praos in production, Genesis as bootstrap, Leios and
  Peras roadmap.
- Tezos: Emmy, Emmy+, Tenderbake.
- Solana: TowerBFT, Proof of History, Frankendancer, Firedancer
  consensus path.
- Aptos: DiemBFTv4, Quorum Store, Shoal, Shoal++.
- Sui: Bullshark then Mysticeti/Mysticeti-FPC, Mahi-Mahi roadmap.
- Hedera: HashGraph in production.
- Algorand: deployed protocol.
- Avalanche subnets and primary network.
- Near: Doomslug and Nightshade.
- Internet Computer: Threshold Relay and NNS.
- Filecoin: Expected Consensus and Tipset Finality.
- Hyperledger Fabric ordering services (Solo, Kafka, Raft, BFT).
- Diem heritage and what survived in Aptos and Sui.

### Part XIII. Verifiable replication and SNARK-friendly consensus

This part develops the notion of a **verifiable replication
algorithm** introduced in the four-pillar definition: a state-
machine replication protocol whose execution can be checked by a
succinct proof. Each module covers theory, practice, formalisation
aspects, and the explicit circuit encoding question. Cover each of:

- SNARK-friendly cryptographic primitives: Poseidon, Rescue-Prime,
  Griffin, Anemoi, MiMC, Reinforced Concrete; SNARK-friendly
  signatures (Schnorr over Pasta, EdDSA over Edwards curves, BLS
  over BLS12-381 and BLS12-377); in-circuit VRFs.
- Recursive proof composition: PCD and IVC, Halo, Halo 2, Nova,
  Sangria, ProtoStar, HyperNova, Mariposa, Pickles. Use as a stage
  for constant-size light-client proofs.
- Verifiable randomness in circuit: VDFs, threshold-BLS randomness
  beacons (drand-style), and how they enter the consensus circuit.
- Mina: Ouroboros Samasika and the Pickles recursion. The first
  production verifiable replication algorithm in the sense above.
- Aleo: snarkOS, AleoBFT, and the snarkVM execution model.
- Verifiable light clients and zk-bridges: zkBridge (Xie et al.
  2022), Succinct's Telepathy and SP1, Polyhedra's Expander, and
  Electron Labs.
- zk-rollup sequencer consensus: Aztec, Scroll, Linea, Starknet,
  Polygon zkEVM. The interface between the sequencer's BFT and the
  validity proof on L1.
- Validity-proof-anchored consensus: Cosmos zk light clients,
  Polkadot's BEEFY plus zk-light-clients, snarkified Tendermint and
  GRANDPA.
- Threshold cryptography in circuit: distributed key generation,
  proactive secret sharing, threshold signatures, and how they
  enable single-secret leader election with succinct verifiability.
- The verifiability frontier: where a protocol's encoding is open
  research (HotStuff in a circuit, DAG-BFT in a circuit, fully
  in-circuit slashing for Casper FFG).
- Cross-cutting analysis: a comparison table classifying every
  protocol from Parts I to XII by its verifiability tag, the
  primitives it relies on, and the open work needed to lift it to
  `deployed`.

### Part XIV. Capstones, meta-theory, and open problems

- Communication complexity of consensus, lower bounds and matching
  upper bounds (Dolev-Reischuk and beyond).
- Adaptive vs static adversaries; secure leader election and
  single-secret leader election.
- Restaking, shared security, and pooled validation (EigenLayer
  whitepaper, follow-ups).
- Cross-chain consensus, IBC, light clients, BEEFY-style bridges.
- A survey of formal-verification work: IronFleet (PBFT, Multi-
  Paxos), Verdi-Raft, Velisarios, Bythos, the Tendermint Coq spec,
  TLA+ Paxos, what is missing for Lean 4.
- A capstone project: build a small SMR end to end in Rust,
  combining a finality gadget, a DAG-based mempool, and a fork-choice
  rule, and write up the safety and liveness arguments in
  formalisation-ready form.

## Implementation constraints

- Language: Rust 2021 (or later).
- Workspace at the repo root; every module is a crate; one shared
  simulator crate at `crates/sim/`.
- Use only dependencies from the approved list in
  `~/.claude/rules/dependencies.md`. If a new dependency is needed
  (for example `ed25519-dalek`, `sha2`, `blake3`, `proptest`,
  `quickcheck`), it must already be approved or be flagged in the
  module README for explicit user approval before adding.
- Avoid production async runtimes unless a module specifically
  teaches an async protocol; prefer the deterministic simulator.
- Lint with `clippy -- -D warnings` and format with `rustfmt`.
- `make check` at repo root must pass: format check, clippy, full
  test suite.

## Pedagogical constraints

- Each `README.md` is self-contained: a graduate student picking up
  cold can follow it, with cross-references to earlier modules where
  appropriate.
- Distinguish results in the original paper from later refinements
  or folklore. When a result is folklore, say so.
- Pseudo-code and pseudo-mathematics in READMEs use ASCII only. No
  Unicode arrows, smart quotes, or em dashes. Use `->`, `<=`, `>=`,
  `!=`, `<->` and plain `"`.
- Neutral tone. No marketing language, no urgency, no hyperbole.
  Soften absolute claims (see `~/.claude/rules/writing-core.md`).
- Cite primary sources for every non-trivial claim.
- Every safety, liveness, and resilience claim must reference the
  theorem or section number in the cited paper.
- Each module ends with two to four open problems or research
  directions, suitable for a graduate student looking for a project.

## History and lore

- A top-level `HISTORY.md` gives a 5 to 10 page narrative timeline:
  - Pre-1980: early ideas (atomic commit, two-phase commit, two
    generals).
  - 1980 to 1985: Byzantine generals, FLP, Dolev-Strong, the
    impossibility era.
  - 1986 to 1999: Paxos, Viewstamped Replication, randomised async
    consensus, PBFT.
  - 2000 to 2008: practical BFT, ZAB, the cloud SMR era, trusted
    hardware.
  - 2009 to 2014: Bitcoin, the Nakamoto detour, GHOST.
  - 2015 to 2019: HotStuff, Algorand, Ouroboros, Casper, Tendermint
    in production.
  - 2020 to 2023: DAG-based BFT (Aleph, Narwhal, Bullshark), Gasper
    in production, async fallbacks.
  - 2024 to 2026: Mysticeti family, Sailfish, Shoal++, Autobahn,
    Mahi-Mahi, Motorway; restaking and shared security; finality-
    gadget redesigns.
- The `lore/` directory holds shorter pieces: the Paxon parliament
  and Lamport's narrative choices, the Bitcoin-NG vs Bitcoin
  politics, why HotStuff and its descendants take espionage names,
  what the BFT-SMaRt project taught about modularity, the
  Diem-to-Aptos-and-Sui split, and so on.

## Deliverables checklist

- [ ] Repository scaffolding and Cargo workspace.
- [ ] Top-level `README.md`, `HISTORY.md`, `references.bib`,
      `Makefile`, `Cargo.toml`, lint and format configs.
- [ ] Shared `crates/sim/` simulator and adversary library.
- [ ] One module directory per protocol named in the outline above,
      each with `README.md`, crate skeleton, exercises, and
      solutions.
- [ ] All four pillars (Theory, Practice, Formalisation aspects,
      Verifiability and circuit encoding) addressed in every module.
- [ ] `make check` green at the repo root after each part is
      complete.
- [ ] `lore/` populated alongside relevant modules.

## Process

1. Bootstrap: write the top-level files, the simulator crate, and
   the empty workspace. Confirm `make build` works.
2. Implement Part I in full; pause and ask the user to review.
3. Continue through the parts in order. After each part, pause for
   review.
4. For each module:
   - Read the primary paper(s) and the most-cited follow-up.
   - Draft the README's Theory, Practice, and Formalisation aspects
     sections before writing any code.
   - Implement the type and message definitions.
   - Implement the round or step logic.
   - Write the simulator-driven property tests.
   - Write exercises and solutions.
   - Run `make check` from the repo root.
5. When uncertain about scope, an unknown protocol detail, or an
   unapproved dependency, stop and ask. Do not invent.
6. When a 2024 to 2026 protocol's primary source is a preprint or a
   talk, cite it as such and flag in the README that the
   formalisation aspects are tentative pending a peer-reviewed
   version.

## Style reminders

- ASCII only. No em dashes, no en dashes, no smart quotes, no
  Unicode arrows, no decorative bullets.
- Wrap prose at 80 columns where the format permits.
- Neutral tone, citations for every non-trivial claim, soften
  absolute statements.
- No AI co-author lines in commit messages or generated content.
- Distinguish theorem, lemma, proposition, corollary, conjecture,
  observation; do not call observations theorems.

End of prompt.
