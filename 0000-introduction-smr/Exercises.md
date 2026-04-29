# Module 0000 Exercises

Each exercise is tagged `[T]` (theory), `[P]` (practice), `[F]`
(formalisation aspects), or `[V]` (verifiability and circuit
encoding).

## Exercise 1 [T] (warm-up): determinism and output equivalence

Suppose two replicas `r1` and `r2` of a state machine `M` share the
initial state and consume the same operation sequence
`o_1, ..., o_k`. Prove that they produce the same output sequence.
Identify the precise place in the proof where you use the
determinism axiom (Section 2 of [Schneider 1990]).

## Exercise 2 [T]: SMR / atomic broadcast equivalence

Hadzilacos and Toueg (1993) proved that, in any failure model,
there exists a total-order broadcast primitive iff there exists an
SMR oracle. Reproduce the construction in both directions:

- (TOB to SMR.) Given a total-order broadcast primitive
  `tob-broadcast`, build an SMR oracle. Argue that the resulting
  oracle satisfies total order, validity, and eventual delivery.
- (SMR to TOB.) Given an SMR oracle and a state machine that simply
  records its inputs (the "log state machine"), build a total-order
  broadcast primitive. Argue that messages are delivered in the
  same order at every honest replica.

## Exercise 3 [P]: detect determinism failures

Modify `Counter` to introduce a non-deterministic operation
`AddRandom` that adds a random number derived from the local
hostname (or any non-replicated source). Run the simulator and
observe that the SMR safety theorem fails. Discuss two production
mitigations: (a) compile-time prohibitions on non-determinism, and
(b) runtime detection by hash comparison of replicas' states.

## Exercise 4 [P]: snapshotting and log compaction

Extend `LeaderBroadcastNode` to broadcast a `Snapshot { seq, state }`
message every `k` operations. A new follower joining the system
should be able to catch up by applying the latest snapshot and
then any operations with sequence number larger than the snapshot's
`seq`. Discuss the invariant that has to hold between the snapshot
sequence number and the operation log to keep SMR safety.

## Exercise 5 [F]: pseudo-Lean SMR signature

Write the `StateMachine` and `Replica` definitions in
formalisation-ready pseudo-Lean (ASCII only), modelled on the
`StateMachine` structure in this module's README. Identify:

- The expected reuse of cslib's `Cslib.Foundations.LTS` for the
  per-replica execution.
- The two natural safety predicates: *prefix agreement* (every two
  honest replicas' logs are prefix-comparable) and *value
  agreement* (committed values at the same index are equal).
- The hypothesis that has to be discharged externally to apply the
  SMR safety theorem (the existence of a total-order oracle).

## Exercise 6 [F]: equivalent characterisations of "honest replica"

The text speaks of "honest replicas" without defining the predicate.
Give three equivalent characterisations:

1. As a property of the local execution (the replica's transition
   trace is a prefix of `delta`-driven traces from `s_0`).
2. As a property of the network interface (the replica sends out
   only what its local state mandates).
3. As a property of the cryptographic interface (in modules that
   use signatures, the replica only signs messages corresponding to
   states it has actually visited).

Argue that the three characterisations agree under the modelling
assumptions of this module.

## Exercise 7 [V]: a verifiable counter SMR

Sketch a *verifiable* version of the leader-broadcast SMR over
`Counter`. Define:

- A statement language `(s, s', op-list)` to be proved succinctly.
- The constraint system encoding `delta` and `observe` for
  `Counter` over a SNARK-friendly arithmetisation (assume
  `wrapping_*` is replaced by arithmetic in a prime field of
  appropriate size).
- The missing oracle component: a proof that `op-list` was the
  output of the ordering oracle.

Identify which of the four verifiability tags (`na`, `friendly`,
`partial`, `deployed`) the resulting construction would carry, and
explain why we cannot reach `deployed` without committing to a
specific consensus protocol.

## Exercise 8 [V]: non-SNARK-friendly determinism

Real virtual machines use non-SNARK-friendly primitives (Keccak-
256, ECDSA, system calls). For each of the following, give a
SNARK-friendly substitute used in a deployed system, and discuss
the cost in proving constraints per opcode:

- Keccak-256 (Ethereum opcode `KECCAK256`).
- ECDSA over secp256k1 (Ethereum signature verification).
- Mapping iteration order in the EVM.

Cite at least one production reference (e.g. zkEVM project
documentation) for each substitute.
