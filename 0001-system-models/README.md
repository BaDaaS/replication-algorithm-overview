# 0001: System Models -- Synchrony, Partial Synchrony, Asynchrony

## Historical context

The 1980s settled the question "what does it mean for a distributed
system to be timed?" three times.

- **Synchronous model.** Pease, Shostak, and Lamport (1980) and
  Lamport, Shostak, and Pease (1982) assumed that messages between
  honest processes are delivered within a known bound `D` of time
  units, and that local clocks tick at a known rate. Under this
  model, deterministic Byzantine agreement is possible iff `n > 3f`
  for `f` Byzantine faults.
- **Asynchronous model.** Fischer, Lynch, and Paterson (1985)
  showed that no deterministic algorithm can solve consensus in the
  asynchronous model (no bound on message delay, no synchronised
  clocks) even with one crash failure. We dedicate module 0003 to
  this result.
- **Partially synchronous model.** Dwork, Lynch, and Stockmeyer
  (1988) introduced the missing middle: the system is asynchronous
  for an arbitrary, finite period and then becomes synchronous
  forever. They showed that consensus is solvable in this model
  with `n > 3f` Byzantine faults (or `n > 2f` for crash failures),
  matching the synchronous bound while making liveness conditional
  on eventual timing stability.

The DLS partial-synchrony model is the one that essentially every
practical BFT protocol from PBFT onwards assumes for liveness. This
module formalises the three models and gives the standard reductions
between them.

## System and threat model

The model parameters are time, delivery, and clocks; failure models
are taken up in module 0002.

- **Time.** A global, but inaccessible-to-processes, real-valued
  clock `t in R`. We frequently coarsen to a discrete tick model
  `t in N` without loss of generality.
- **Local clocks.** Each process `i` has a clock `C_i : R -> R`.
  The synchronous model assumes `|C_i(t) - t| < epsilon` for some
  small `epsilon`; the partially synchronous model assumes either
  that `epsilon` exists but is unknown, or that drift is bounded
  only after a global stabilisation time (GST).
- **Channels.** Each pair of processes `i, j` has an FIFO,
  authenticated channel that may delay or lose messages.

## Theory

### Definition (synchronous system)

A system is *synchronous* if there exist constants `D, Phi > 0`
known to all processes such that:

- (Timely delivery.) Every message sent at time `t` between honest
  processes is delivered by time `t + D`.
- (Bounded clock drift.) For every honest process `i` and times
  `t1 < t2`, `(t2 - t1) / Phi <= C_i(t2) - C_i(t1) <= Phi (t2 - t1)`.

### Definition (asynchronous system)

A system is *asynchronous* if no upper bound on message delivery
or clock drift exists. Messages are eventually delivered (the
channel never permanently drops a message), but no schedule is
guaranteed.

### Definition (partially synchronous, DLS variants)

There are two equivalent definitions in DLS:

- **Variant A (eventual bounds).** Constants `D, Phi` exist but are
  unknown to processes; the synchrony predicate holds at all times.
- **Variant B (eventual synchrony).** Constants `D, Phi` are known
  to processes, but a *global stabilisation time* `T_GST in N` (an
  arbitrary, unknown finite time) exists such that the synchrony
  predicate holds for all `t >= T_GST`.

### Theorem (DLS variant equivalence)

Variants A and B yield the same set of solvable problems. The
reduction from B to A makes `D, Phi` "unknown" by erasing them and
forcing the algorithm to discover them; the reduction from A to B
fixes `T_GST = 0` and uses a doubling trick to discover bounds.

*Proof sketch.* DLS Theorem 4.1; the doubling trick is to treat a
candidate bound `D_k = 2^k` as the round timeout, and incrementing
`k` whenever a round fails to terminate. Eventually `D_k > D` and
the algorithm makes progress. QED (sketch).

### Theorem (DLS resilience under partial synchrony)

For any failure type, the resilience bound under partial synchrony
matches the synchronous bound:

- Crash failures: `f < n/2`.
- Omission failures: `f < n/2`.
- Authenticated Byzantine (signed messages): `f < n/2`.
- Unauthenticated Byzantine: `f < n/3`.

*Proof.* DLS Theorems 5.1 to 5.7. The key construction is *round
timeouts that grow*; the safety argument is the same as in the
synchronous case once the round bounds are large enough.

### Failure scope versus timing scope

It is a common confusion to conflate failure model and timing model.
Both restrict the adversary, but along orthogonal axes:

- The failure model restricts which behaviour the adversary may
  impose on individual processes (crash, omit, deviate
  arbitrarily).
- The timing model restricts which schedules of message delivery
  and clock drift the adversary may impose on the network.

A protocol is described by a pair (failure model, timing model).
Module 0002 develops failure models in detail; this module fixes
the timing axis.

### Local versus global timeouts

Synchronous protocols may use timeouts as part of the protocol
logic ("if no message has arrived by round `r * D`, conclude the
sender is faulty"). Partially synchronous protocols may use
timeouts only for liveness, not for safety: a missed timeout
triggers a view change, not an inconsistent decision. This is the
DLS principle of *separating safety from liveness*.

## Practice

### Which production protocols assume which model

| Protocol family   | Timing assumption       | Source            |
| ----------------- | ----------------------- | ----------------- |
| Paxos / Multi-Paxos | Partial synchrony     | Lamport 1998      |
| Raft              | Partial synchrony       | Ongaro-Ousterhout |
| PBFT              | Partial synchrony       | Castro-Liskov 99  |
| Tendermint        | Partial synchrony       | Buchman 2014      |
| HotStuff(-2)      | Partial synchrony       | Yin et al. 2019   |
| Bullshark         | Partial synchrony       | Spiegelman 2022   |
| HoneyBadger BFT   | Asynchrony              | Miller 2016       |
| Dumbo             | Asynchrony              | Guo 2020          |
| Mysticeti         | Partial synchrony       | Babel 2024        |
| Bitcoin           | Partial synchrony[*]    | Pass-Shi 2017     |
| Algorand          | Partial synchrony       | Chen-Micali 2017  |
| Ouroboros Praos   | Semi-synchrony          | David 2018        |

[*] Bitcoin's safety is information-theoretic in synchronous models
and probabilistic in partially synchronous ones; the original
whitepaper assumed effective synchrony and was retroactively
re-analysed by Pass-Shi-Seeman 2017 and the Bitcoin backbone
papers.

### How production stacks instantiate the model

- **Local clocks.** Most production stacks use `gettimeofday(2)` or
  `clock_gettime(CLOCK_MONOTONIC)`. Spanner uses TrueTime, which
  exposes an explicit confidence interval `[earliest, latest]`.
  TrueTime is a *bounded-uncertainty* clock model and is closer to
  synchronous than partial synchrony.
- **Round timeouts.** Tendermint and HotStuff implementations
  expose three configurable timeouts: `propose`, `prevote`,
  `precommit`. Bull-shark and Mysticeti use a "round timeout"
  measured in DAG layers rather than wall time.
- **Eventual synchrony in the wild.** GST is never reached
  algorithmically; it is asserted operationally. Most operational
  postmortems of consensus halts (e.g. Cosmos Hub vote-extension
  bugs, Solana periodic reboots) trace back to a refusal of the
  network to enter the "good period" assumed by the protocol.

### Cross-cutting concerns

- **Adaptive adversaries** can exploit timing windows to corrupt
  the next leader; partial synchrony alone does not protect
  against this. The Single-Secret Leader Election work (BEHG 2020)
  closes this gap by hiding the next leader's identity until they
  speak.
- **Network calibration.** Some protocols (e.g. AWS Aurora,
  Google's Spanner) tightly control the network so that effective
  synchrony holds; others (open blockchains, IoT consensus) cannot
  rely on calibration and lean on partial synchrony with large
  timeouts.

## Formalisation aspects

### Pseudo-Lean signatures

```text
-- ASCII-only pseudo-Lean.

def Time := Nat                    -- discrete logical time

structure SyncBounds where
  D    : Nat                       -- max delivery delay
  Phi  : Nat                       -- clock drift bound

inductive TimingModel where
  | synchronous (b : SyncBounds)
  | partially_synchronous_eventual_bounds
  | partially_synchronous_eventual_stab (gst_known : Bool)
  | asynchronous
```

### Schedules and adversary types

A *schedule* is a relation `S : Time -> Multiset Envelope`. A
schedule is *D-timely after `t0`* if for every envelope `e in S(t)`
sent at time `t_send`, `t - t_send <= D` whenever `t >= t0 + D`.

The DLS partial-synchrony predicate becomes:

```text
def IsPartiallySynchronous (S : Time -> Envelope -> Prop) : Prop :=
  exists D : Nat, exists T : Time,
    forall t : Time, t >= T ->
    forall e : Envelope, S t e -> e.delivery_time - e.send_time <= D
```

### Why this matters for proof assistants

Synchrony assumptions are quantifier-rich. Lean's `Nat`-based
arithmetic and Mathlib's `Filter.atTop` are well-suited. The cslib
`Cslib.Foundations.LTS` machinery generalises to `LTS` indexed by
time; a partially synchronous LTS has an `Eventually` modality on
the synchrony predicate, expressible with Mathlib's
`Filter.Eventually`. The principal proof obligation is
*round-induction*: prove that the protocol's progress argument
goes through whenever the local round advances past the (finite,
unknown) GST.

### Temporal logic as the right specification language

The natural language for these specifications is *linear temporal
logic* (LTL), introduced to computer science by Pnueli's 1977 FOCS
paper "The Temporal Logic of Programs" [`pnueli1977`]. LTL adds to
classical logic the modalities `G phi` ("always `phi`"), `F phi`
("eventually `phi`"), `X phi` ("`phi` at the next step"), and
`phi U psi` ("`phi` holds until `psi`"). The DLS Variant B
synchrony predicate is precisely an `F` modality on a global
predicate over the schedule: `F G TimelyDelivery(D)`. Manna and
Pnueli's "Temporal Verification of Reactive Systems"
[`mannapnueli1995`] is the canonical reference for LTL-based
distributed-system specifications. Lamport's TLA+
[`lamport2002specifying`] is a different but compatible
formalisation, and the Tendermint and Casper FFG specifications
in TLA+ are documented in their respective public repositories.

Throughout this course, when we state a liveness property as
"eventually every honest replica decides", we mean it in the LTL
sense: `F (forall i, decided_i)`. When we state a safety property
as "no two honest replicas decide differently", we mean
`G (forall i j, decided_i /\ decided_j -> decision_i = decision_j)`.
The decomposition of correctness into "safety + liveness", a
distinction first made rigorous in Alpern-Schneider 1985 and
generalised in Manna-Pnueli, is the standard interface between the
prose statement of a theorem and its formal counterpart. We follow
this convention in every module from Part II onwards.

## Verifiability and circuit encoding

**Tag: `na`.**

Timing models are not the kind of thing that one encodes in a
SNARK circuit. They are *side conditions* on the schedule of the
network, not on any computational object. Verifiable replication
algorithms (Part XIII) typically assume partial synchrony at the
consensus layer and prove only the state-transition relation in
circuit; the synchrony assumption flows in as a non-circuit
hypothesis on the validator-set behaviour.

This separation is intentional. The *whole point* of partial
synchrony is that safety holds even before GST; only liveness
depends on the synchrony predicate. A SNARK proof of a chain
prefix is a safety-only object: it commits the verifier to "if the
proven prefix exists, it is consistent with the protocol", not to
"the chain made progress in real time". Verifiable replication
therefore aligns naturally with partial synchrony: SNARKs prove
the safety closure, the network provides liveness, and a halt is
visible (the chain stops growing) but not a soundness violation.

A subtler verifiability angle: timing-dependent slashing
conditions (e.g. Ethereum's "proposer is too late" slashing) need
the verifier to know `now`. In production, this is supplied by an
on-chain clock (the L1 block height). Module 0049 (Gasper) and
Part XIII develop this further.

## Known attacks and limitations

- **Eclipse attacks** can keep a victim's effective timing
  arbitrarily worse than the rest of the network's, simulating an
  asynchronous-only environment for the victim. PBFT and HotStuff
  remain safe but the victim never makes progress.
- **Network-level adversaries** (BGP hijacks, ISP-level traffic
  manipulation) can delay GST indefinitely. The protocol does not
  liveness-recover until the adversary releases the network.
- **Local clock skew** beyond `Phi` lets a process trigger leader
  rotation prematurely, hammering the network with view changes.
  Production systems guard with NTP and refuse to participate when
  local skew exceeds a threshold.

## Implementation notes

The crate provides three adversary implementations, one per timing
model. They are compatible with the simulator from `crates/sim/`
and are the building blocks reused by every later module.

- `SynchronousAdversary { delay: Time }` delivers every message
  exactly `delay` ticks after sending. Models the synchronous
  case with `D = delay`.
- `AsynchronousAdversary { max_delay: Time, rng_seed }` delays
  each message by a uniformly random number of ticks in
  `[1, max_delay]`. Pure asynchrony is realised by setting
  `max_delay = u64::MAX`; bounded asynchrony by smaller values.
- `PartiallySynchronousAdversary { gst, sync_delay, async_max }`
  delays messages by up to `async_max` ticks before `gst` and by
  exactly `sync_delay` ticks afterwards. Models DLS Variant B
  with a known `T_GST`.

The tests pair each adversary with a simple counter SMR (reused
from module 0000) and observe:

- Under `SynchronousAdversary`, all replicas terminate quickly.
- Under `AsynchronousAdversary` with no bound, there exist seeds
  such that processes never deliver in any bounded interval.
- Under `PartiallySynchronousAdversary`, before GST the system may
  not terminate; after GST it terminates within bounded time.

## References

- Pease, Shostak, and Lamport, "Reaching Agreement in the Presence
  of Faults", JACM 1980. [`psl1980`].
- Lamport, Shostak, and Pease, "The Byzantine Generals Problem",
  TOPLAS 1982. [`lsp1982`].
- Fischer, Lynch, Paterson, "Impossibility of Distributed Consensus
  with One Faulty Process", JACM 1985. [`flp1985`].
- Dwork, Lynch, Stockmeyer, "Consensus in the Presence of Partial
  Synchrony", JACM 1988. [`dls1988`].
- Pass, Seeman, Shelat, "Analysis of the Blockchain Protocol in
  Asynchronous Networks", Eurocrypt 2017.
- Pnueli, "The Temporal Logic of Programs", FOCS 1977.
  [`pnueli1977`]. Foundational reference for LTL.
- Manna and Pnueli, "Temporal Verification of Reactive Systems:
  Safety", Springer 1995. [`mannapnueli1995`].
- Lamport, "Specifying Systems: The TLA+ Language and Tools",
  Addison-Wesley 2002. [`lamport2002specifying`].
- Alpern and Schneider, "Defining Liveness", Information Processing
  Letters 1985 (the canonical safety/liveness decomposition).

See also [`HISTORY.md`](../HISTORY.md), section "1980 to 1985" and
"1986 to 1999".
