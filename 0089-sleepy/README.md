# 0089: Sleepy Consensus

## Historical context

Rafael Pass and Elaine Shi published "The Sleepy Model of
Consensus" at Asiacrypt 2017, formalising a fundamental
property of permissionless protocols: tolerating *unbounded
intermittent crash failures* among participants.

In the standard BFT model (e.g., PBFT, module 0042), all `n`
nodes must be online and responsive. If a node sleeps (crashes
without intent to recover), it is counted as a fault. A
protocol with `f < n/3` tolerates only `f` such sleeping nodes.

In open networks (Bitcoin, Ethereum), participants come and go
freely. At any moment, only a fraction of the registered
participants is online. Pass-Shi formalise this via the *sleepy
adversary*: an adversary may put any subset of nodes to sleep
and wake them later, with the only constraint being that
*online honest* nodes outnumber *online corrupted* nodes at
every time.

## System and threat model

- **Network.** Bounded-delay (PSS-style).
- **Failures.** A *sleepy* adversary that may put nodes to sleep
  (silent, non-responsive) and wake them later. At every time,
  the online honest fraction `alpha_online > 1/2 + epsilon`.
- **Cryptography.** Each registered node has a public-key
  identity (PoS-style) or hashing power (PoW-style).
- **Goal.** Robust SMR despite unbounded sleep faults.

### Why sleepy is different from crash-stop

In crash-stop (Lamport-Fischer-Pease, module 0006), a faulty
node never recovers. Sleepy nodes recover and rejoin. In
sleepy:

- The total registered population is fixed (PoS-stake-weighted
  or PoW hashing-power-weighted).
- At any time, an arbitrary subset is "online".
- Sleeping nodes do nothing; they neither send nor receive
  messages.
- Awakened nodes catch up by receiving the chain.

The Sleepy theorem: a protocol is sleepy-secure iff online
honest > online corrupted at every time.

## Theory

### Sleepy theorem (Pass-Shi 2017)

A randomised round-based PoS-style protocol exists that
satisfies CP / CG / CQ in the bounded-delay sleepy model
provided online honest > online corrupted at every time.

Proof technique: encode each round as a randomised slot
proposer; if a chosen proposer is online and honest, they
extend the chain; if corrupted, they may equivocate; if
sleeping, the slot is empty. The ratio of honest:corrupted
proposers (filtered by online status) drives the chain growth
analysis, mirroring the GKL-PSS framework.

### Sleepy in PoS protocols

Sleepy consensus is the foundational model for proof-of-stake
protocols. The Ouroboros family (modules 0125 to 0131) and
Algorand (module 0143) build on the sleepy abstraction:
participants register stake, but at any time only a subset is
online.

### Comparison: BFT, PoW Nakamoto, and Sleepy

| property                | BFT (PBFT)    | PoW Nakamoto  | Sleepy       |
| ----------------------- | ------------- | ------------- | ------------ |
| node availability       | always online | always online | intermittent |
| identity                | known         | none          | registered   |
| Sybil resistance        | identity      | hash power    | stake / PoW  |
| fault threshold         | `f < n/3`     | hash > 1/2    | online honest > online corrupted |
| protocol type           | deterministic | longest chain | longest chain (slot-based) |
| supports rotation       | yes           | yes           | yes          |
| applies to PoS          | partially     | no            | natural fit  |
| applies to PoW          | partially     | natural fit   | natural fit  |
| latency                 | seconds       | minutes       | seconds-minutes |
| analysis maturity       | mature        | mature        | mature       |

Sleepy is a *meta-model*: it characterises which threat models
are tolerable, not a specific protocol. Concrete sleepy
protocols include:

- *Sleepy* (Pass-Shi 2017): the original construction.
- *Snow White* (Daian-Pass-Shi 2019, module 0124): a
  PoS protocol designed in the sleepy model.
- *Ouroboros Praos* (module 0126): sleepy-secure PoS with
  forward-secure signatures.
- *Algorand* (Chen-Micali 2019, module 0143): sleepy-secure
  PoS with VRF-based committee selection.

### Sleepy adversary classes

Pass-Shi distinguish three adversary models:

1. *Static sleepy.* The adversary commits to a sleep schedule
   in advance.
2. *Adaptive sleepy.* The adversary may sleep nodes after
   seeing their participation.
3. *Strongly adaptive sleepy.* The adversary may corrupt nodes
   after seeing their messages and erase their messages
   (simulating that the message was never sent).

Each is strictly weaker (i.e., harder to defeat). Sleepy 2017
addresses static and adaptive; strongly adaptive is the
subject of subsequent work (e.g., Pass-Shi 2018, Thunderella).

## Practice

Sleepy is implicitly assumed by every PoS chain in production:
Cardano, Ethereum (post-Merge), Algorand, Cosmos, Polkadot,
Tezos, Aptos, Sui all tolerate sleep faults.

In practice, "sleep" includes:

- *Network-disconnected nodes.* Common during ISP outages.
- *Software failures.* Crashed validator processes.
- *Maintenance windows.* Scheduled downtime.
- *Voluntary inactivity.* Validators who drop out before formal
  unregistration.

### Production-implementation notes

- Inactivity slashing (Ethereum) penalises sleep but does not
  terminate the protocol. Pass-Shi's framework gives the
  theoretical justification.
- VRF-based proposer selection (Algorand, Cardano Praos) is
  designed to be sleep-tolerant: if a chosen proposer is
  asleep, the slot is simply empty.

## Verifiability and circuit encoding

**tag: `partial`.**

Sleepy is a property of the protocol model; verifiability is
inherited from the underlying chain protocol (e.g., Ouroboros
Praos circuits, Mina Samasika circuits). The sleepy condition
itself does not appear in circuits; it is a pre-condition for
the safety/liveness analysis.

## Known attacks and limitations

- *Long-range attacks.* In PoS, an adversary who held majority
  stake at some past time can rewrite history. Sleepy does not
  address this; weak subjectivity or finality gadgets (Casper
  FFG) do.
- *Posterior corruption.* A sleeping node's signature key may
  later be acquired by an adversary. Forward-secure signatures
  (Praos) mitigate.
- *Online honest minority.* If more honest than corrupted nodes
  sleep, the sleepy condition fails locally and safety can be
  violated.

## References

- Pass, Shi, "The Sleepy Model of Consensus", Asiacrypt 2017.
- Pass, Shi, "FruitChains: A Fair Blockchain", PODC 2017
  (introduces fairness in the sleepy model).
- Daian, Pass, Shi, "Snow White: Robustly Reconfigurable
  Consensus and Applications to Provably Secure Proof of
  Stake", FC 2019.

## Implementation notes

The crate provides a simple `SleepyState` snapshot tracking
which nodes are online and which are honest, plus a predicate
`sleepy_secure` checking whether the online-honest > online-
corrupt invariant holds. Tests verify the predicate on
contrived populations.

See also [`HISTORY.md`](../HISTORY.md), section "2014 to 2017".
