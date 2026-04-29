# 0005: FLP Impossibility

## Historical context

Fischer, Lynch, and Paterson published "Impossibility of Distributed
Consensus with One Faulty Process" in JACM 1985 [`flp1985`]. The
result is the central theoretical "no-go" of the consensus
literature: no deterministic protocol can solve binary consensus
in an asynchronous system that may experience even a *single*
crash failure.

The result is doubly surprising. The asynchronous model permits
unbounded message delay but assumes eventual delivery and reliable
channels. The single-failure model is the weakest possible (one
crash). And yet *no* deterministic algorithm can guarantee
termination. The proof is short, elegant, and entirely topological
in spirit; we give it in full.

The FLP impossibility motivates every avenue this course covers:
randomised consensus (Ben-Or, Rabin, HoneyBadger), failure
detectors (Chandra-Toueg), partial synchrony (DLS, PBFT,
HotStuff), and proof-of-work / proof-of-stake. They are all ways
of escaping the asynchronous deterministic regime.

## System and threat model

- **Network.** Asynchronous: no upper bound on message delay or
  process speed; reliable channels (every sent message is
  eventually delivered exactly once).
- **Failures.** Up to one *crash-stop* failure.
- **Inputs and outputs.** Each process has an input bit
  `b_i in {0, 1}` and is required to produce an output bit
  `c_i in {0, 1}`.
- **Protocol.** A deterministic state machine taking the local
  state and an incoming message to a new state and outgoing
  messages.

The protocol must satisfy:

- *(Agreement.)* All non-crashed processes output the same bit.
- *(Validity.)* If all input bits are equal to `b`, then no
  process outputs `1 - b`.
- *(Termination.)* Every non-crashed process outputs a bit in a
  finite number of steps.

## Theory

### Definitions

A *configuration* `C` of an asynchronous system is the global
state: per-process local states plus the multiset of in-flight
messages.

A *step* applies a single available action: receive a delivered
message at some process and update its local state (possibly
sending out new messages).

A configuration `C` is *0-valent* if every reachable extension of
`C` decides 0; *1-valent* if every reachable extension decides 1;
*univalent* if 0-valent or 1-valent; *bivalent* otherwise.

### Lemma 1 (initial bivalence)

There exists an initial configuration that is bivalent.

*Proof.* Let `C(b_1, ..., b_n)` denote the initial configuration
with input vector `b = (b_1, ..., b_n)`. By validity:

- `C(0, 0, ..., 0)` is 0-valent.
- `C(1, 1, ..., 1)` is 1-valent.

The Hamming distance between these two configurations is `n`.
Walk from one to the other by flipping one input bit at a time,
producing a sequence `C_0 = C(0,...,0), C_1, ..., C_n =
C(1,...,1)`. Adjacent configurations `C_k, C_{k+1}` differ in one
input bit. By validity, the first `C_k` that is not 0-valent
exists (since `C_n` is 1-valent, and `C_0` is 0-valent). Suppose
this `C_k` is 1-valent. Then `C_{k-1}` is 0-valent and `C_k` is
1-valent, differing in one input bit. The crashed-process scenario
in which the differing process crashes immediately gives the same
extension from both, but they decide differently. Contradiction.

So there is a bivalent `C_k`. QED.

### Lemma 2 (bivalent extension)

From any bivalent configuration `C`, for any pending message `m`
deliverable at some process `p`, there exists a finite sequence
of steps (not delivering `m`) followed by the delivery of `m`,
ending in a bivalent configuration `C'`.

*Proof.* Suppose, for contradiction, that delivering `m` at any
extension `C_e` of `C` (without delivering `m` in between) yields
a univalent configuration. Define `D_e = step_m(C_e)` (the result
of delivering `m` at `C_e`). The set of `D_e` is partitioned into
0-valent and 1-valent.

Because `C` is bivalent, both 0-valent and 1-valent `D_e` exist.
By the connectivity of step-extensions, there exist neighbouring
configurations `C_e_1, C_e_2 = step_e(C_e_1)` (i.e. `C_e_2` is
one step away from `C_e_1`) such that `D_{e_1}` is 0-valent and
`D_{e_2}` is 1-valent.

Three sub-cases on the distinguishing step `e` (delivery of some
message `m'` at process `p'`):

- **(a) `p' != p`, `m' != m`.** Then `step_m` and `step_{m'}`
  commute (they act on different processes and different
  messages). So `step_{m'}(D_{e_1}) = D_{e_2}` and
  `step_m(C_{e_2}) = step_m(step_{m'}(C_{e_1})) = D_{e_2}`. But
  `step_m(C_{e_1}) = D_{e_1}` is 0-valent, so a 0-valent
  configuration steps to the 1-valent `D_{e_2}`, contradicting
  univalence.

- **(b) `p' = p`, `m' != m`.** Now `m` and `m'` are both
  destined for `p`. Consider an extension that crashes `p`
  immediately and continues without `p`. Call this configuration
  `C_e^*`. From `C_e^*`, both `D_{e_1}` and `D_{e_2}` extend to
  the same configuration (they only differ in `p`'s behaviour,
  which is now crashed). Hence the extensions agree, but
  `D_{e_1}` is 0-valent and `D_{e_2}` is 1-valent: contradiction.

- **(c) `p' = p`, `m' = m`.** This is the case where `m'` is `m`
  itself; the distinguishing step *is* the delivery of `m`. Then
  `D_{e_1} = step_m(C_{e_1})` and `D_{e_2} = step_m(C_{e_2}) =
  step_m(step_m(C_{e_1}))` which is not well-defined (each
  message is delivered at most once). So this case does not
  occur with reliable channels.

In all valid sub-cases we reach a contradiction. Therefore
delivering `m` from some extension of `C` results in a bivalent
configuration. QED.

### Theorem (FLP impossibility)

There is no deterministic protocol that solves binary consensus
in an asynchronous system with at most one crash failure.

*Proof.* By Lemma 1, choose a bivalent initial configuration
`C_0`. We construct an infinite execution that never decides.

By Lemma 2, for any pending message `m`, we can extend `C_0` to a
bivalent configuration `C_1` in which `m` has just been
delivered. Iterate: at step `i`, pick the *oldest* pending
message `m_i` (FIFO across the global queue). Apply Lemma 2 to
get a bivalent configuration `C_{i+1}` in which `m_i` has been
delivered.

The resulting execution:

- Delivers every message exactly once (FIFO admin guarantees no
  starvation).
- Is non-faulty (no process crashes, every message is delivered).
- Visits only bivalent configurations, so no configuration is
  decided.

Termination would require eventual univalence; bivalence-
preserving extension shows it never happens. QED.

### Sources

The argument above is the textbook treatment, due originally to
FLP 1985 and refined by Lynch's "Distributed Algorithms" 1996.
The bivalence/univalence terminology is FLP's. Reflecting the
elegance of the proof, FLP won the 2001 Dijkstra Prize.

### Workarounds (preview)

The FLP impossibility leaves four escape hatches:

1. **Randomisation.** Ben-Or 1983 and Rabin 1983 give protocols
   that terminate with probability 1 in expected `O(2^n)` rounds
   (Ben-Or) or `O(1)` rounds (Rabin, with a common coin).
   Module 0015.
2. **Partial synchrony.** DLS 1988 provides a way out by adding
   eventual synchrony, the basis of all production BFT.
   Module 0001.
3. **Failure detectors.** Chandra-Toueg 1996 expose information
   about failures sufficient to break determinism without
   randomisation. Module 0014.
4. **Eventual or weakly-consistent agreement.** CRDT-style
   protocols sidestep the problem by relaxing termination.

### Strengthenings

- *Other strong asynchronous problems.* Atomic snapshots, set
  agreement, renaming all admit similar impossibilities. The
  unifying theme is a topological condition: solvability
  corresponds to the protocol's *configuration complex* having
  certain connectivity (Herlihy-Shavit 1999).
- *Strict asynchrony with no failures.* Even with zero failures,
  if the model is "wait-free" (each process must act on its own
  pace), some problems remain unsolvable. FLP itself does not
  apply (zero failures + agreement is solvable trivially), but
  set-agreement type problems do.

## Practice

### What FLP says about real systems

FLP is operationally a statement about *guarantees*, not about
typical behaviour. In practice, asynchronous protocols *can*
solve consensus: Bitcoin's blockchain, eventually consistent
databases, and gossip protocols all converge in the absence of
adversarial scheduling. What FLP rules out is the *guarantee* of
termination in *every* asynchronous schedule with one failure.

The standard production response:

- **Liveness via timeouts.** Protocols add timeout-based view
  changes for liveness. Safety holds always; liveness holds when
  the network is partially synchronous (DLS GST).
- **Liveness via randomisation.** HoneyBadger BFT (module 0070)
  uses a common coin to reach termination in expected constant
  rounds, with no synchrony assumption.
- **Liveness via PoW / PoS difficulty adjustment.** Bitcoin's
  difficulty adjustment is a randomised escape from FLP that
  works because the proof-of-work step is not a deterministic
  message-passing protocol.

### A practical FLP scenario: stuck consensus halts

Operational reports of consensus halts (Cosmos chains, early
Tendermint, Solana reboots) usually trace back to *de facto*
asynchrony: the network's effective delay exceeded the
view-change timeout, so the protocol fell into a regime where
liveness was not guaranteed. FLP says this is the best one can
do: we cannot have safety, liveness, asynchrony, and one
failure all simultaneously.

## Formalisation aspects

### Pseudo-Lean structure

```text
structure AsyncSystem (n : Nat) where
  state    : Type
  init     : Vec Bool n -> state
  step     : state -> Message -> state * List Message
  decide   : state -> Option Bool

inductive Configuration (n : Nat) where
  -- per-process states + multiset of in-flight messages
  mk (states : Fin n -> AsyncSystem.state)
     (network : Multiset Message) : Configuration n

def Reachable (C C' : Configuration n) : Prop :=
  -- transitive closure of "step at some pending message"
  sorry

def IsZeroValent (C : Configuration n) : Prop :=
  forall (C' : Configuration n),
    Reachable C C' /\ Decided C' -> DecidedValue C' = 0

def IsBivalent (C : Configuration n) : Prop :=
  exists (C0 C1 : Configuration n),
    Reachable C C0 /\ Decided C0 /\ DecidedValue C0 = 0 /\
    Reachable C C1 /\ Decided C1 /\ DecidedValue C1 = 1

theorem flp_impossibility (n : Nat) (h_n : n >= 2) :
    forall (sys : AsyncSystem n),
      not (
        forall (input : Vec Bool n) (sched : Schedule),
          AgreementHolds sys input sched /\
          ValidityHolds sys input sched /\
          TerminationHolds sys input sched
      ) := by
  intro sys
  intro hAll
  -- 1. by Lemma 1 there is a bivalent C_0
  obtain <<C_0, hBivalent_0>> := initial_bivalence sys
  -- 2. by Lemma 2, build an infinite bivalent execution
  -- 3. termination is violated
  sorry
```

### What is hard about a Lean formalisation

The proof is *higher-order* in a subtle way: Lemma 2's case
analysis depends on which step `e` distinguishes the 0-valent and
1-valent extensions, which is a property of the *protocol*. The
proof in Lean would parametrise the entire argument over the
protocol's transition function and use Mathlib's `Finset` /
`List` / `Multiset` machinery for the configuration space. The
co-inductive construction of the infinite execution would use
Mathlib's `Stream'`.

CSLib's `LTS` machinery is less directly useful here than for
positive results: FLP is a theorem *about every LTS*, not within
one. The formal counterpart would be a higher-order theorem
quantifying over the LTS's transition relation.

There is published precedent: the Coq formalisation of FLP by
Bisping et al. (2016) and the TLA+ specification by Lamport
(1990) demonstrate the result is mechanisable, though neither
has been ported to Lean 4 yet. This is an open problem in the
broader cslib effort.

## Verifiability and circuit encoding

**Tag: `na`.**

FLP is a non-existence theorem; there is no protocol to encode.
The closest verifiable counterpart is *negative*: a SNARK proof
of "this protocol terminated" is, on its own, vacuous if the
protocol is asynchronous-deterministic with one failure (FLP
guarantees the existence of non-terminating runs).

Production verifiable replication therefore *commits* to a
synchrony assumption (partial synchrony) at the consensus layer
and proves only the state-transition relation, not the network
schedule. This is the right architectural choice in light of
FLP: the SNARK certifies the protocol's safety closure, the
network supplies the liveness assumption, and the verifier
treats both as separately checkable.

A subtler verifiability angle: a randomised consensus protocol
(HoneyBadger BFT) circumvents FLP via a common coin. To make
the *coin* verifiable in circuit, one needs SNARK-friendly
threshold cryptography (e.g. threshold-BLS) and a randomness
beacon (drand-style). Module 0070 (HoneyBadger) and Part XIII
develop this.

## Known attacks and limitations

- *FLP is tight.* The model assumptions (asynchrony, single
  crash, deterministic protocol) are each individually
  necessary; relaxing any of them admits a protocol.
- *Randomised relaxations.* Ben-Or terminates with probability
  1 but expected exponential time; Rabin terminates in expected
  constant time but requires a common coin.
- *Production confusion.* Engineers sometimes claim "we have a
  production async consensus protocol" when they mean
  "partially synchronous"; the distinction is not pedantic, it
  is the dividing line between a protocol that meets FLP's
  hypothesis and one that does not.

## Implementation notes

The crate provides a *constructive demonstration* of FLP: the
simulator runs a toy two-process consensus with a custom
adversary that delivers messages in an order that, by
inspection, keeps the system bivalent for many rounds. This is
not a proof of FLP (that lives in the README); it is an
empirical illustration of how the adversary's freedom over
schedule is the source of impossibility.

A second test runs the same protocol under `NoOpAdversary`
(a synchronous schedule) and shows that termination is
straightforward when the schedule is benign. The contrast
visualises FLP's hypothesis: it is the worst-case *adversarial*
schedule that breaks the protocol, not typical behaviour.

## References

- Fischer, Lynch, Paterson, "Impossibility of Distributed
  Consensus with One Faulty Process", JACM 1985. [`flp1985`].
- Ben-Or, "Another Advantage of Free Choice: Completely
  Asynchronous Agreement Protocols", PODC 1983.
- Rabin, "Randomized Byzantine Generals", FOCS 1983.
- Chandra and Toueg, "Unreliable Failure Detectors for Reliable
  Distributed Systems", JACM 1996. [`ct1996`].
- Dwork, Lynch, Stockmeyer, "Consensus in the Presence of Partial
  Synchrony", JACM 1988. [`dls1988`].
- Bisping et al., "Mechanical Verification of a Constructive Proof
  for FLP", ITP 2016 (Coq formalisation).
- Lynch, "Distributed Algorithms", Morgan Kaufmann 1996,
  Chapter 21.

See also [`HISTORY.md`](../HISTORY.md), section "1980 to 1985".
