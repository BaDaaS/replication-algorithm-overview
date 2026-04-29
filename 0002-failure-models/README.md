# 0002: Failure Models

## Historical context

A failure model is a constraint on what behaviour the adversary
may impose on individual processes. The taxonomy crystallised in
Cristian's 1991 IEEE Computer survey and was refined by
Hadzilacos-Toueg in 1993 (the "Fault-Tolerant Broadcasts" chapter).
Aguilera-Chen-Toueg (2000) added the crash-recovery model that
real systems require, and the BAR fault-tolerance line
(Aiyer-Alvisi-Clement-Dahlin-Martin-Porth 2005) introduced
rational deviation as a first-class category. Eyal-Sirer (2014)
showed that rational deviations matter even in protocols that are
"safe" against Byzantine adversaries: a rational miner can boost
its revenue by selfish mining without violating any safety
property of Bitcoin.

## System and threat model

This module supplies the failure axis that pairs with the timing
axis from module 0001. Concrete protocols specify a pair `(F, T)`
of (failure model, timing model).

## Theory

### Definition (failure model hierarchy)

Let `n` be the number of processes and `f` be the maximum number
of processes the adversary may corrupt. We list the standard
failure types from least to most powerful adversary.

- **Crash-stop.** A faulty process executes the protocol correctly
  up to some point and then halts. It does not recover.
- **Crash-recovery.** A faulty process may halt and later
  resume; on resumption it may have lost any in-RAM state. Stable
  storage may be assumed to survive.
- **Send-omission.** A faulty process executes correctly except
  that some subset of its outgoing messages is silently dropped.
- **Receive-omission.** Similar but for incoming messages.
- **General omission.** Both send-omission and receive-omission.
- **Authenticated Byzantine.** A faulty process behaves
  arbitrarily, but cannot forge signatures of honest processes.
- **Unauthenticated Byzantine.** Arbitrary behaviour with no
  signature guarantees; equivalent to assuming all messages are
  unauthenticated.
- **Mobile failure.** The set of corrupted processes changes over
  time, but at any point at most `f` are corrupt
  (Ostrovsky-Yung 1991).
- **Adaptive corruption.** The adversary chooses which `f`
  processes to corrupt as the protocol unfolds, learning their
  state on corruption. Distinct from mobile in that adaptive
  corruption is monotone (once corrupted, forever corrupted).
- **Static corruption.** The corrupted set is fixed in advance.
- **Rational (BAR).** Processes split into Byzantine, Altruistic,
  Rational. Rational processes deviate iff deviation is utility-
  maximising. Aiyer et al. 2005.

The hierarchy refines: any protocol resilient against a more
powerful adversary is resilient against a weaker one.

### Theorem (resilience lower bounds)

| Failure model        | Synchronous | Partial synchrony | Asynchronous     |
| -------------------- | ----------- | ----------------- | ---------------- |
| Crash-stop           | `f < n`     | `f < n/2`         | `f < n/2` (rand) |
| Authenticated Byz.   | `f < n`     | `f < n/2`         | `f < n/2` (rand) |
| Unauthenticated Byz. | `f < n/3`   | `f < n/3`         | `f < n/3` (rand) |

*Sources:*

- Synchronous Byzantine `f < n/3` is Pease-Shostak-Lamport 1980
  (Theorem 2). Authenticated `f < n` is the same paper, Theorem 3.
- Partial synchrony bounds are DLS 1988 Theorems 5.1 to 5.7.
- Asynchronous bounds (`f < n/3` Byzantine and `f < n/2` crash) are
  for *randomised* protocols; deterministic protocols are
  impossible by FLP. Sources: Bracha-Toueg 1985, Ben-Or 1983.

### Lemma (fail-stop is detectable)

In the fail-stop model, every honest process can correctly identify
the set of crashed processes after a bounded number of rounds (in
synchrony) or eventually (in partial synchrony). This is the
strongest possible "failure detector" and underwrites Schneider's
1990 SMR construction.

### Reductions

- Crash-stop reduces to authenticated Byzantine: a crashed process
  is just a Byzantine process that emits no messages. Hence any
  protocol tolerating `f` Byzantine faults tolerates `f` crashes.
- Crash-recovery reduces to crash-stop only if stable storage is
  reliable. With no stable storage, crash-recovery is *not* a
  refinement of crash-stop because a recovering process may take
  decisions inconsistent with its pre-crash state.
- Adaptive Byzantine reduces to mobile Byzantine on a strictly
  shorter time horizon.

### Failure detectors (preview)

The Chandra-Toueg failure detector classes (`P, S, OmegaP, Omega`)
parameterise the *information* available to honest processes about
who has failed. We develop them in module 0014.

## Practice

### How real systems pick a failure model

| System         | Failure model            | Resilience |
| -------------- | ------------------------ | ---------- |
| etcd, ZooKeeper| Crash-stop (with disk)   | `f < n/2`  |
| CockroachDB    | Crash-recovery + disk    | `f < n/2`  |
| Spanner        | Crash-recovery + TrueTime| `f < n/2`  |
| CometBFT       | Unauthenticated Byzantine| `f < n/3`  |
| Aptos / Sui    | Authenticated Byzantine  | `f < n/3`  |
| Bitcoin        | Rational + Byzantine     | computational majority |
| Algorand       | Adaptive Byzantine + sig | `f < n/3`  |
| Cardano        | Adaptive + sig (Praos)   | `f < n/2`  |

### Crash-recovery is the realistic baseline

Production replicas almost always assume crash-recovery, even when
the consensus protocol is described in crash-stop terms.
Implementations:

- Persist the current view, the last accepted proposal, and the
  log to disk before responding.
- Replay the persisted log on restart.
- Implement careful "promise" semantics so that replays do not
  violate safety.

The Raft paper specifies persistence requirements explicitly
(`currentTerm`, `votedFor`, `log[]`); the Paxos literature is more
implicit.

### Byzantine in practice

True Byzantine behaviour in production usually arises from
implementation bugs (a buggy validator equivocates because of a
client mismatch), not from malice. Slashing protocols (Casper FFG,
GRANDPA, Tendermint's evidence module) treat the bug and the
malicious node identically, which is operationally appropriate.

### Rational deviation

Selfish mining (Eyal-Sirer 2014): a rational miner can withhold
blocks to win more than its fair share of rewards. Bitcoin's
safety is preserved (no double-spend), but the protocol's
*incentive* compatibility is not: miners with less than 50% hash
power can profit by deviating. Subsequent work (Sapirshtein-Sompolinsky
-Zohar 2016) refined the threshold and Eyal 2015 showed the result
in continuous time.

## Formalisation aspects

### Pseudo-Lean: an enum of failure modes per process

```text
inductive FailureMode where
  | honest
  | crashStop  (crash_at : Time)
  | omission   (drop_pred : Time -> Envelope -> Bool)
  | byzantine  (script : Time -> ProcessState -> Action)
  | mobile     (corrupt_at : Time -> Set NodeId)
```

### Resilience as an invariant

```text
def NumCorruptedAt (mode : NodeId -> FailureMode) (t : Time)
    : Nat :=
  -- count NodeIds whose mode is non-honest at time t
  sorry

def Resilient
    (mode : NodeId -> FailureMode) (n f : Nat) : Prop :=
  forall t : Time, NumCorruptedAt mode t <= f
```

The resilience theorem of each protocol then takes the form

```text
theorem protocol_safety
    (mode : NodeId -> FailureMode)
    (h : Resilient mode n f)
    (h_n : 3 * f < n)        -- e.g. unauthenticated Byzantine
    : Safety := by
  sorry
```

### Cryptographic assumption as a hypothesis

For authenticated Byzantine, the formalisation must include:

```text
def UnforgeableSignatures
    (sigma : SignatureScheme) : Prop :=
  forall (A : Adversary),
    Pr [A wins forgery game with sigma] < negligible
```

The protocol's safety theorem then has `UnforgeableSignatures sigma`
as a hypothesis. This pattern is the exact analogue of how
cryptographic theorems read in Crypto Made Simpler (Aaronson) or in
Mathlib's `MeasureTheory` infrastructure.

## Verifiability and circuit encoding

**Tag: `na`.**

Failure models are predicates on traces, not protocols. The notion
of a *verifiable* failure model arises only in conjunction with a
specific protocol. Two patterns appear in production:

- **Slashing as a circuit-checkable predicate.** Casper FFG
  defines double-vote and surround-vote as predicates over signed
  messages. These predicates are SNARK-friendly when the signature
  scheme is (BLS over BLS12-381 in Ethereum, Schnorr over Pasta in
  Mina). Light clients can produce a proof of a slashable offence;
  the L1 verifier accepts the proof.
- **Verifiable failure detectors.** The Chandra-Toueg detector
  classes give "this process is suspected to have failed" outputs.
  In a verifiable setting, suspicion can be supported by signed
  evidence (the suspect's missed heartbeat is committed to a
  validator-set signature). This pattern shows up in zk-bridges
  with light-client headers.

We tag this module `na` because the abstract failure taxonomy is
not itself a protocol. The verifiability discussion proper lives
in the modules that pair a failure model with a concrete protocol.

## Known attacks and limitations

- **Modelling gaps.** Real failures often span the boundary
  between two categories: a crashing node may briefly send a
  half-formed message before crashing (a "fail-noisy" failure).
  The protocol designer must either widen the failure model to
  cover this or argue that the half-message cannot cause harm.
- **Adaptive vs static gap.** Cryptographic schemes secure against
  static corruption may fail against adaptive corruption (the
  classical example is the BLS threshold signature without
  proactive refresh). Algorand and Ouroboros Praos are designed to
  resist adaptive corruption via VRFs and ephemeral keys.
- **Rational deviations against verifiability.** A verifiable
  protocol still requires the validator set to be incentive-
  compatible. A rational set of validators can collude
  asymptotically without violating the protocol's safety predicate
  (e.g. all signing the same equivocation in turn). Module 0049
  (Gasper) discusses this in the Ethereum context.

## Implementation notes

The crate provides three failure adversaries layered over the
timing adversaries from module 0001:

- `CrashStopAdversary { crash_at: BTreeMap<NodeId, Time> }`. For
  each crashed node, drop all outgoing messages from time
  `crash_at[node]` onwards. Honest deliveries to the node still
  arrive (the node's local state is never observed externally
  again).
- `OmissionAdversary`. Drops outgoing messages with a configurable
  per-node probability.
- `EquivocatingAdversary` (Byzantine, simple). For each Byzantine
  node, when it broadcasts a message to a set of recipients, the
  adversary may swap in different (`equivocated`) messages for a
  designated subset.

The `EquivocatingAdversary` is intentionally protocol-agnostic and
operates on opaque message types. Per-protocol Byzantine behaviour
(specific malicious votes, leader-equivocation, withholding) is
the responsibility of each protocol module.

## References

- Cristian, "Understanding Fault-Tolerant Distributed Systems",
  CACM 1991.
- Hadzilacos and Toueg, "Fault-Tolerant Broadcasts and Related
  Problems", in Distributed Systems (2nd ed.), Addison-Wesley
  1993. [`ht1993`].
- Aguilera, Chen, Toueg, "Failure Detection and Consensus in the
  Crash-Recovery Model", Distributed Computing 2000.
- Bracha and Toueg, "Asynchronous Consensus and Broadcast
  Protocols", JACM 1985.
- Aiyer, Alvisi, Clement, Dahlin, Martin, Porth, "BAR Fault
  Tolerance for Cooperative Services", SOSP 2005.
- Eyal and Sirer, "Majority is not Enough: Bitcoin Mining is
  Vulnerable", FC 2014.
- Ostrovsky and Yung, "How to Withstand Mobile Virus Attacks",
  PODC 1991.

See also [`HISTORY.md`](../HISTORY.md), section "1986 to 1999"
and "2009 to 2014".
