# 0012: Chandra-Toueg Failure Detectors

## Historical context

Chandra and Toueg's "Unreliable Failure Detectors for Reliable
Distributed Systems" (JACM 1996, [`ct1996`]) introduced the
*failure detector* abstraction: a per-process oracle that
outputs a set of *suspected* processes. The oracle is
parameterised by *completeness* (every actually-faulty process
is eventually suspected) and *accuracy* (no honest process is
permanently suspected).

The detector classes are organised in a 4x2 grid:

| Class | Completeness  | Accuracy            |
| ----- | ------------- | ------------------- |
| `P`   | Strong        | Strong (perfect)    |
| `S`   | Strong        | Eventually strong   |
| `OmegaP` (`<>P`) | Strong | Eventually weak  |
| `OmegaS` (`<>S`) | Strong | Eventually weak  |

The weakest detector that solves consensus in asynchronous
crash-stop is `Omega` (eventually elects a permanent leader);
this is Chandra-Hadzilacos-Toueg 1996, "The Weakest Failure
Detector for Solving Consensus".

The result restored solvability where FLP had ruled it out:
adding even a weak failure-detection oracle is sufficient to
escape the asynchronous deterministic regime.

## System and threat model

- **Network.** Asynchronous (no upper bound on delay).
- **Failures.** Crash-stop only (initially; Byzantine
  generalisations exist but are subtler).
- **Failure detector.** A local module at each process that
  outputs `suspected_i ⊆ Pi` at each step. The detector is
  *unreliable*: it may suspect honest processes and miss faulty
  ones, but the protocol relies only on the abstract
  completeness/accuracy guarantees.

## Theory

### Definition (completeness)

- *Strong completeness.* Eventually every faulty process is
  permanently suspected by every honest process.
- *Weak completeness.* Eventually every faulty process is
  permanently suspected by at least one honest process.

### Definition (accuracy)

- *Strong accuracy.* No honest process is ever suspected.
- *Weak accuracy.* Some honest process is never suspected.
- *Eventual strong accuracy.* Eventually no honest process is
  suspected by any honest process.
- *Eventual weak accuracy.* Eventually some honest process is
  not suspected by any honest process.

### Eight classes, four equivalence classes

The 4x2 grid yields 8 classes; some are equivalent:

- `P` (perfect): strong completeness + strong accuracy.
- `Q` (quasi-perfect): weak completeness + strong accuracy. Equivalent to `P`.
- `S` (strong): strong completeness + eventual strong accuracy.
- `W` (weak): weak completeness + eventual weak accuracy.
- `OmegaP` (`<>P`): strong completeness + eventual strong accuracy.
- `OmegaQ`: weak completeness + eventual strong accuracy. Equivalent to `<>P`.
- `OmegaS` (`<>S`): strong completeness + eventual weak accuracy.
- `OmegaW`: weak completeness + eventual weak accuracy. Equivalent to `<>S`.

Reduction: weak completeness + something can be transformed into
strong completeness + something via a peer-gossip protocol
(every process broadcasts its suspect set; weak-complete +
gossip = strong-complete). Hence the four "essential" classes
are `P, S, <>P, <>S`.

### Theorem (`Omega` is the weakest detector for consensus)

`Omega` is the leader-election detector: it outputs at each
honest process some leader `leader_i in Pi`, with the
guarantee that *eventually* `leader_i = leader_j` for all
honest `i, j` and the leader is honest.

*Theorem (CHT 1996).* The weakest failure detector that
solves consensus in asynchronous crash-stop with `f < n / 2`
is `Omega`.

*Proof (sketch).*

- *Solvability with `Omega`.* Run the rotating-coordinator
  algorithm: each round, the elected leader proposes a value;
  others vote; if `>= n / 2 + 1` agree, decide. Once `Omega`
  stabilises, the leader is honest and consensus terminates.
- *Necessity of `Omega`.* Any detector that solves consensus
  can be reduced to `Omega`: from the protocol, extract a
  function from the local state to a "would-be leader". The
  reduction is the heart of CHT 1996.

QED (sketch).

### Theorem (`<>S` solves consensus with `f < n / 2`)

The Chandra-Toueg algorithm uses `<>S` to solve consensus in
asynchronous crash-stop with `f < n / 2`. The protocol is the
*rotating coordinator with timeouts*:

```
algorithm CT-Consensus (f < n / 2, detector D in <>S):
  for r = 1, 2, 3, ...
    coordinator c_r := r mod n
    if i = c_r:
      collect proposals from any majority not-suspected
      broadcast (estimate, r)
    else:
      wait for (estimate, r) from c_r OR D suspects c_r
      if received: ack with vote
      if suspected: nack with no-vote
    coordinator collects acks; if majority, broadcast (decide).
```

Once `<>S` stabilises (some honest process is never
suspected) and that process becomes coordinator (eventually,
by round-robin), the coordinator finishes the round and
consensus terminates.

### Comparison to BFT

`P, S, <>P, <>S, Omega` are designed for crash-stop. Byzantine
counterparts exist but are subtler: a Byzantine process can lie
about its suspect list, so the gossip-based reduction breaks.
Doudou-Schiper 1998 introduced *muteness detectors* for
Byzantine settings; modern BFT protocols often skip explicit
detectors in favour of view-change machinery (PBFT, HotStuff)
that implements an `Omega`-equivalent without naming it.

## Practice

### Heartbeat-based detectors

Production detectors are timeout-based. `i` periodically pings
`j`; if `i` does not hear from `j` within `T` ticks, `i`
suspects `j`. Tuning `T` is a tension between *false suspects*
(large `T` reduces them) and *detection latency* (small `T`
detects quickly).

This is a `<>S`-class detector under partial synchrony: once the
network is stable (after GST), `T` exceeds the actual delay and
honest processes are no longer suspected.

### Where detectors show up

- *Paxos / Raft.* The leader's heartbeat-and-timeout machinery
  is exactly an `Omega` implementation. Raft's election timeout
  is the detector's `T` parameter.
- *PBFT / HotStuff.* View-change timeouts behave like an
  `Omega`-detector; HotStuff-2 makes this explicit by tying view
  change to QC stability.
- *Cosmos CometBFT.* `propose_timeout`, `prevote_timeout`,
  `precommit_timeout` are the detector's three thresholds.
  Operationally tuned per-network.

### Robustness

Detector tuning has been the source of many production
incidents. Cassandra's "split brain" issues (a node thinks the
cluster has partitioned but the rest disagree) trace to
asymmetric detector states. Production systems combat this with
SWIM-style group membership (Das-Gupta-Motivala 2002) which adds
*piggybacked* dissemination of suspect updates.

## Formalisation aspects

```text
class FailureDetector (n : Nat) where
  output : NodeId -> Time -> Set NodeId  -- suspect list

class StronglyComplete (D : FailureDetector n) where
  prop : forall t, exists t' >= t,
    forall i j, IsHonest i -> IsCrashed j ->
    j ∈ D.output i t'

class EventuallyStronglyAccurate (D : FailureDetector n) where
  prop : exists t, forall t' >= t,
    forall i j, IsHonest i -> IsHonest j ->
    j ∉ D.output i t'

class OmegaDetector (D : FailureDetector n) where
  leader : NodeId -> Time -> NodeId
  eventually_agrees :
    exists t, forall t' >= t, forall i j,
    IsHonest i -> IsHonest j ->
    leader i t' = leader j t' /\ IsHonest (leader i t')
```

The Chandra-Toueg theorems become reductions between detector
classes. cslib's `LTS` framework with a parameter for the
detector's output is a natural encoding. Mathlib's
`Filter.Eventually` is the right modality for "eventually...".

## Verifiability and circuit encoding

**Tag: `na`** for the abstract detector hierarchy; `partial` for
heartbeat-based implementations.

A signed-heartbeat detector can be encoded in a SNARK: each
tick, a process signs its current "alive" status; recipients
verify the signature within `T`-tick freshness. Verifiable
detector outputs are useful for *cross-chain liveness proofs*
(zk-bridges that need to attest that a chain is making
progress, not merely that a state is consistent).

The abstract `Omega` detector is a *computational* object that
does not naturally fit into a per-block circuit; it lives at the
process-pair level. Concrete detector outputs (e.g. "validator
`v` was suspected at slot `s`") are verifiable; the abstract
property "`Omega` eventually stabilises" is not.

This is a useful design lens: BFT slashing predicates (Casper
FFG) are *verifiable accuracy* properties; they identify when
the detector has wrongly accused an honest validator. Module
0049 (Gasper) develops this further.

## Known attacks and limitations

- *Detector latency.* Real detectors can take seconds to
  minutes to stabilise. Production protocols set timeouts well
  above expected RTT (often `>10 * RTT`).
- *Adversarial detector inputs.* If the network is adversarial,
  a detector can be made arbitrarily slow to stabilise (this is
  precisely the FLP scenario). `<>S` is solvability *given*
  detector stabilisation; FLP says stabilisation may never come.
- *Byzantine generalisations.* Byzantine processes can lie
  about who is suspected. Solutions: signed heartbeats, muteness
  detectors, or skipping the detector formalism in favour of
  view-change schemes.

## Implementation notes

The crate provides a `HeartbeatDetector { timeout }`:

- Each node sends a `Heartbeat` once per tick.
- A node suspects `j` if it has not received `j`'s heartbeat
  within `timeout` ticks.

Tests verify completeness (a crashed node is eventually
suspected by all honest processes) and accuracy under benign
conditions (no honest node is suspected).

## References

- Chandra and Toueg, "Unreliable Failure Detectors for Reliable
  Distributed Systems", JACM 1996. [`ct1996`].
- Chandra, Hadzilacos, Toueg, "The Weakest Failure Detector for
  Solving Consensus", JACM 1996.
- Aguilera, Chen, Toueg, "On Quiescent Reliable Communication",
  SIAM Comp 2000.
- Doudou and Schiper, "Muteness Failure Detectors for Consensus
  with Byzantine Processes", PODC 1998.
- Das, Gupta, Motivala, "SWIM: Scalable Weakly-Consistent
  Infection-Style Process Group Membership Protocol", DSN 2002.

See also [`HISTORY.md`](../HISTORY.md), section "1986 to 1999".
