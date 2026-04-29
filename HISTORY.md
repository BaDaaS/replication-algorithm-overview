# A Narrative History of Consensus

This file is the timeline counterpart to the modular course. It is
written to be read end-to-end. Modules in the course expand each
episode below into a self-contained study unit.

## Pre-1980: atomic commit and the two generals

The earliest replication problems arose from databases. The
two-phase commit protocol (2PC), credited in folklore to the early
1970s System R team and formalised by Jim Gray in 1978, gave a
practical solution to atomic commitment but was known to block when
the coordinator failed. Lampson and Sturgis explored crash recovery
and stable storage in the late 1970s. The Two Generals problem,
first stated by Akkoyunlu, Ekanadham, and Huber in 1975, exposed
the impossibility of agreement over a lossy channel and would later
be cited as the philosophical seed for the FLP impossibility result.

## 1980 to 1985: the impossibility era

Pease, Shostak, and Lamport published "Reaching Agreement in the
Presence of Faults" (1980), introducing the synchronous Byzantine
agreement problem and proving an `n > 3f` lower bound. Lamport,
Shostak, and Pease followed in 1982 with "The Byzantine Generals
Problem", giving an oral-message protocol for `n > 3f` rounds and a
written-message variant tolerating `f < n` with signatures.
Dolev and Strong (1983) showed a matching `f + 1` round lower bound
for any deterministic synchronous Byzantine agreement protocol with
authentication.

The high point of impossibility came in 1985. Fischer, Lynch, and
Paterson proved that no deterministic asynchronous protocol can
solve consensus even with a single crash failure. The same year,
Dolev and Reischuk gave the `Omega(n * f)` lower bound on message
complexity for Byzantine agreement.

## 1986 to 1999: Paxos and the rise of practical SMR

Oki and Liskov introduced Viewstamped Replication in 1988 as part
of their replicated transaction system. Lamport's "The Part-Time
Parliament" was submitted in 1989 and finally published in 1998
after a famously long review cycle; it became known as Paxos.
"Paxos Made Simple" (2001) and the multi-Paxos and fast-Paxos
variants followed. Dwork, Lynch, and Stockmeyer (1988) introduced
the partial-synchrony model that all practical BFT protocols
adopted, and Chandra and Toueg (1996) characterised the weakest
failure detector for consensus.

In 1999, Castro and Liskov published Practical Byzantine Fault
Tolerance (PBFT). It was the first BFT replication protocol with
production-ready performance and became the template for two
decades of follow-up work.

## 2000 to 2008: practical BFT, ZAB, the cloud era

The 2000s saw a wave of BFT engineering. Q/U, HQ, Zyzzyva, and
Aardvark refined the PBFT design space along the latency, fault-
tolerance, and robustness axes. Hybrid BFT designs (A2M, TrInc,
MinBFT) used trusted hardware to relax the `n > 3f` bound to
`n > 2f`. ZooKeeper Atomic Broadcast (ZAB), published in 2008 and
formalised in 2011, anchored Apache ZooKeeper and inspired Kafka.
Chain Replication (van Renesse and Schneider, 2004) and CRAQ
(Terrace and Freedman, 2009) became reference designs for storage
systems. Spanner (2012) deployed multi-Paxos at planet scale with
TrueTime.

## 2009 to 2014: the Nakamoto detour

Bitcoin's 2008 whitepaper repurposed proof-of-work and longest-chain
heuristics to solve a new problem: open-membership consensus among
mutually distrusting parties without any prior identity registry.
The protocol was widely deployed before its theoretical foundations
were settled. Garay, Kiayias, and Leonardos (2015, 2017) gave the
first rigorous analysis (the Bitcoin backbone) and Pass, Seeman, and
Shelat (2017) extended it to partial synchrony. Sompolinsky and
Zohar's GHOST (2015) and Eyal and Sirer's selfish mining (2014)
made clear that the longest-chain rule was both more subtle and
less robust than initially thought.

## 2015 to 2019: HotStuff, Algorand, Ouroboros, Casper

This window produced four protocol families that still dominate the
landscape:

- **HotStuff and the streamlined-BFT family.** Yin et al. (2019)
  reframed BFT around a chained pipeline with linear authenticator
  complexity per view and optimistic responsiveness, simplifying
  the analysis and unifying view changes with normal-case
  operation. Tendermint (Buchman 2014, Buchman-Kwon-Milosevic 2018)
  had pioneered the streamlined approach earlier in production.
- **Algorand.** Chen and Micali (2017) and Gilad et al. (2017)
  combined a verifiable random function for cryptographic
  sortition with a fast Byzantine agreement protocol over
  small committees, achieving fast finality with sub-quadratic
  communication.
- **Ouroboros.** Kiayias et al. (2017) gave the first proof-of-stake
  protocol with formal security in the UC framework. Praos (2018)
  and Genesis (2018) added security against adaptive adversaries
  and against bootstrapping attacks. Crypsinous and Chronos
  followed.
- **Casper FFG.** Buterin and Griffith (2017) introduced the
  finality gadget that, combined with a fork-choice rule, became
  the basis for Ethereum's eventual proof-of-stake design.

## 2020 to 2023: DAG-based BFT and async fallbacks

The DAG-based wave grew from the observation that decoupling data
dissemination from agreement allows much higher throughput.
DAG-Rider (2021), Narwhal-Tusk and Narwhal-Bullshark (2022), and
Cordial Miners (2023) made the approach practical. Ethereum
deployed Gasper (LMD-GHOST plus Casper FFG) in production with the
Merge in 2022. Polkadot deployed BABE plus GRANDPA. Cosmos chains
ran Tendermint and then CometBFT.

## 2024 to 2026: Mysticeti and finality redesign

Mysticeti (2024) collapsed the DAG and consensus rounds for sub-
second finality. Mahi-Mahi, Sailfish, Shoal++, Autobahn, and BBCA-
chain followed in rapid succession, each pushing on different
points of the DAG-BFT design surface (commit latency, anchor
density, Byzantine resilience, partial synchrony). HotStuff-2
(Malkhi-Nayak 2023) refined the original HotStuff to remove the
two-chain confirmation lag. Ethereum's roadmap turned to single-
slot or three-slot finality and proposer-builder separation, while
Cardano announced Leios and Peras as its next-generation throughput
and finality layers.

By 2026, the field had three productive frontiers:

- **Performance frontier.** DAG-based protocols, threshold
  cryptography for fairness and front-running resistance, and
  PBS-style separation between block production and validation.
- **Composition frontier.** Restaking and shared security
  (EigenLayer and follow-ups), cross-chain consensus and light
  clients, and finality-gadget redesigns that compose linear-
  authenticator BFT with eventual consistency layers.
- **Verifiability frontier.** A separate line of work, which had
  been growing steadily since the publication of recursive SNARK
  schemes (Halo in 2019, Halo 2 and Nova in 2021, ProtoStar and
  HyperNova in 2023), made the consensus protocol itself the
  subject of a succinct proof. Mina shipped Ouroboros Samasika under
  the Pickles recursion in 2021, becoming the first deployed system
  in which a light client checks a constant-size proof of the
  full chain history. Aleo extended the idea with snarkOS and a
  SNARK-native execution layer. zk-rollup sequencers (Aztec, Scroll,
  Linea, Starknet, Polygon zkEVM) embedded a small BFT inside a
  validity proof that anchors L1. zk-bridges (zkBridge, Telepathy,
  Polyhedra) compiled production consensus protocols (Tendermint,
  Casper FFG, GRANDPA) into circuits to give light clients
  succinct cross-chain finality. The umbrella term that the course
  adopts for this line is **verifiable replication algorithm**: a
  state-machine replication protocol whose execution can be
  succinctly proved and verified in time polylogarithmic in the
  chain length and the validator-set size. Part XIII develops the
  theory and practice of verifiable replication explicitly.

## What this course covers

The course expands each episode above into a sequence of modules.
Every protocol named in this file (and many that are not) gets a
module with the four pillars: theory, practice, formalisation
aspects, and verifiability and circuit encoding. See the [course
outline](README.md#course-outline) and the [generation
prompt](PROMPT.md) for the full plan.
