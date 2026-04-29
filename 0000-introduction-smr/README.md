# 0000: Introduction to State-Machine Replication

> "Replication is the application of state-machine refinement to a
> distributed setting. The state machine is the algorithm; the
> distribution is the engineering." (paraphrase, Schneider 1990)

## Historical context

The state-machine approach to fault tolerance was crystallised by
Lamport in his 1978 paper "Time, Clocks, and the Ordering of
Events", which gave the logical-clock formalism that any SMR
construction needs in order to talk about a "common order of
operations". Lamport sketched a more direct construction in 1984
(the "interactive consistency" line of work) and Schneider's 1990
ACM Computing Surveys tutorial "Implementing Fault-Tolerant
Services Using the State Machine Approach" became the canonical
reference: a replicated service is reduced to two requirements,
deterministic state machines and total-order broadcast.

This module sets up the SMR abstraction itself. We do not yet
discuss any concrete agreement protocol: the rest of the course
replaces the abstract "ordering oracle" introduced here with
specific protocols (Paxos, Raft, PBFT, HotStuff, Tendermint,
Bullshark, Mysticeti, Ouroboros, Gasper).

## System and threat model

This module is deliberately parametric.

- **Processes.** A finite set of replicas, each running a copy of a
  deterministic state machine.
- **Network.** Pair-wise channels for delivering operations to
  replicas. The exact reliability and timing model is left to the
  ordering oracle.
- **Failures.** Not specified at this layer. The SMR theorem is a
  *conditional* statement: given total-order broadcast, replicas
  agree on state. The premise is what subsequent modules build.
- **Crypto.** None at this layer.

## Theory

### Definition (state machine)

A *deterministic state machine* is a tuple `M = (S, s0, Op, Out,
delta, observe)` where:

- `S` is a set of states.
- `s0 in S` is the initial state.
- `Op` is a set of operations (commands).
- `Out` is a set of outputs.
- `delta : S * Op -> S` is the transition function.
- `observe : S * Op -> Out` is the output function.

We require `delta` and `observe` to be functions: given the same
state and operation, they always produce the same successor state
and output. This is the *determinism axiom* and is the load-bearing
hypothesis of the entire SMR programme.

### Definition (SMR run)

A *run* of a state machine on operation sequence
`o = o_1, o_2, ..., o_k` is the sequence of states
`s_0, s_1, ..., s_k` defined by `s_i = delta(s_{i-1}, o_i)` and
the sequence of outputs `y_1, ..., y_k` defined by
`y_i = observe(s_{i-1}, o_i)`.

### Definition (replicated state machine)

A *replicated state machine* over `M` is a system of `n` replicas,
each running an independent copy of `M`. Each replica `r_j` consumes
a sequence of operations `o_1^{(j)}, o_2^{(j)}, ...`. The system is
parameterised by an *ordering oracle* that promises every honest
replica is fed the same prefix of the same operation sequence.
Formally:

- *Total order*: there is a sequence `o = (o_1, o_2, ...)` such
  that, for every honest replica `r_j` and every index `i`, the
  `i`-th operation `r_j` consumes is `o_i` (when defined).
- *Validity*: every operation in the global sequence `o` was
  proposed by some client.
- *Eventual delivery*: for every client-proposed operation `o`,
  every honest replica eventually consumes a prefix of `o` that
  contains `o`.

### Theorem (SMR safety)

Let `M` be a deterministic state machine and let `r_j`, `r_k` be
two honest replicas of `M` driven by an ordering oracle satisfying
total order, validity, and eventual delivery. Then for every index
`i`, after both replicas consume their first `i` operations, they
are in the same state and have produced the same output prefix.

*Proof.* By induction on `i`. Base case `i = 0`: both replicas are
in `s_0` and have produced the empty output sequence. Inductive
step: assume after `i` operations they share state `s_i` and output
prefix `y_1, ..., y_i`. By total order, the `(i+1)`-th operation
`o_{i+1}` they each consume is the same. By determinism of
`delta` and `observe`, both replicas produce `s_{i+1} = delta(s_i,
o_{i+1})` and `y_{i+1} = observe(s_i, o_{i+1})`. QED.

The proof exposes the contract: **safety of the replicated service
reduces to the conjunction of (i) determinism of the local state
machine, and (ii) total-order broadcast on operations.** The rest
of the course is about constructing total-order broadcast under
various adversary models.

### Theorem (SMR-broadcast equivalence; Hadzilacos-Toueg 1993)

For any state machine `M` and any process group, an SMR oracle
satisfying total order, validity, and eventual delivery exists if
and only if a total-order (atomic) broadcast primitive exists. We
state this as a theorem here and refer the proof to module 0010
(Reliable, causal, atomic broadcast).

### Complexity

At this layer of abstraction there is nothing to count. Concrete
complexity bounds (round complexity, message complexity,
authenticator complexity, communication complexity) attach in
later modules to the specific protocols implementing the ordering
oracle.

## Practice

### Production systems built on the SMR pattern

- **etcd** (Kubernetes control plane) runs Raft as the ordering
  oracle. The state machine is a key-value store with watch
  semantics.
- **ZooKeeper** runs ZAB; the state machine is a hierarchical
  namespace.
- **Apache Kafka** uses ZooKeeper (legacy) or KRaft for metadata
  SMR; user log is a separate, partitioned eventual-consistency
  layer.
- **Spanner / CockroachDB** run multi-Paxos or Raft per shard; the
  state machine is a SQL transaction processor.
- **Aptos / Sui / Solana / Ethereum / Cardano** run BFT or PoS
  protocols as the ordering oracle and the state machine is the
  smart-contract VM. The *open membership* (proof-of-work, proof-of-
  stake) replicas of the SMR pattern emerge in Parts VIII to XII.

### Pragmatic concerns the abstraction hides

- **Snapshotting and log compaction.** Real replicas cannot keep
  the entire operation log forever. Production code must agree on
  snapshot points and reconstruct state from them.
- **Configuration changes.** Adding or removing a replica is a
  consensus action of its own; reconfiguration protocols (single-
  decree Paxos for membership, joint consensus in Raft) are
  themselves first-class.
- **Speculative execution.** Some systems execute operations
  optimistically before total order is decided, then roll back on
  conflict. This trades latency against rollback complexity.
- **Read paths.** Linearisable reads either go through the log
  (safe but slow) or use lease-based shortcuts (faster, more
  subtle). All non-trivial reads have a model-specific argument.

### Determinism in practice

The determinism axiom is famously hard to enforce on real
hardware: floating-point variation, hash-table iteration order,
timezones, randomised data structures, OS schedulers, and
non-deterministic JIT all break it. Production SMRs spend large
amounts of code suppressing non-determinism (whitelisted system
calls, reproducible RNGs, deterministic VMs). This is the original
motivation for the deterministic VMs of Aptos, Sui, and Solana, and
for replay-determinism in CometBFT.

## Formalisation aspects

A future Lean 4 formalisation would express:

```text
-- Pseudo-Lean, ASCII only.

structure StateMachine where
  S      : Type
  s0     : S
  Op     : Type
  Out    : Type
  delta  : S -> Op -> S
  observe: S -> Op -> Out
```

The replicated system can be modelled as a labelled transition
system over per-replica states. Reuse of cslib's `LTS`
infrastructure is natural: each replica's local execution is a
`LabelledTransition` whose labels are operations consumed in order.
The SMR safety theorem becomes:

```text
-- Pseudo-Lean.

theorem smr_safety
    (M : StateMachine)
    (r1 r2 : Replica M)
    (i : Nat)
    (htotal : TotalOrder r1 r2 i)
    : state_after r1 i = state_after r2 i
    /\ output_after r1 i = output_after r2 i := by
  induction i with
  | zero => exact ⟨rfl, rfl⟩
  | succ k ih =>
    -- use determinism of delta and observe
    sorry
```

The interesting formalisation work is not the SMR theorem itself
(which is a structural induction) but the *axiomatisation of the
ordering oracle*. The proof obligation that consensus protocols
discharge in later modules is precisely the `TotalOrder` premise of
this theorem.

Cross-references for later parts:

- The cslib `InferenceSystem` framework (`Cslib.Foundations.Logic`)
  is well-suited for stating the validity and eventual-delivery
  properties as deduction rules.
- Mathlib's `Stream'` and `List.IsPrefix` give the "shared prefix"
  formulation of agreement.

## Verifiability and circuit encoding

**Tag: `na`.**

This module defines the SMR pattern abstractly and contains no
protocol whose execution would be encoded in a circuit. The
notion does, however, anchor the rest of the course's verifiability
content. We define here, and refer back to in every subsequent
module, the following notion.

### Definition (verifiable replication algorithm)

A *verifiable replication algorithm* is an SMR construction in
which, in addition to the ordering oracle and the state machine,
there is a *succinct proof relation* `pi : (S, S', op-list) -> Bool`
such that:

- (Soundness) `pi(s, s', ops) = true` only if `s'` is the result of
  applying `ops` to `s` under `delta`, and `ops` is a total-order
  prefix produced by the oracle.
- (Succinctness) verifying `pi` takes time polylogarithmic in
  `|ops|`, in the number of replicas, and in the size of `s'`.
- (Completeness) honestly executed runs admit such a proof.

Mina's Pickles plus Ouroboros Samasika is the deployed example of
this notion. zk-rollup sequencers and zk-bridges realise weaker
variants in which the proof covers only the SMR's transition
relation but not the consensus oracle. Part XIII develops these
constructions in depth.

For now we record only that the verifiability tag classification
introduced in [`PROMPT.md`](../PROMPT.md) lifts to SMR
constructions: an SMR is `friendly` if `delta`, `observe`, and the
oracle's verifying predicate all admit efficient SNARK encodings;
`partial` if only the state machine does; `deployed` if a
production system already publishes such proofs.

## Known attacks and limitations

The SMR pattern *delegates* the hard part to the ordering oracle.
Every attack against an SMR-based system is, structurally, either:

1. An attack on the oracle (consensus): equivocation, leader bias,
   long-range, selfish mining, withholding, eclipse.
2. An attack on the determinism axiom: undeclared non-determinism in
   the state machine that lets two replicas legitimately disagree
   on `delta(s, op)` (timezone bugs, RNG drift, JIT differences).
3. An attack on the client-to-replica boundary: replay, censorship,
   front-running of the operation submission interface.

Type-2 attacks have caused production incidents (e.g. early
Ethereum chain splits attributed to client implementation
divergence). They are within scope of the formalisation effort but
out of scope of the agreement-protocol modules.

## Implementation notes

The crate `replication-0000-introduction-smr` provides:

- A `StateMachine` trait mirroring the definition above.
- A `Counter` example state machine.
- A `Replica<S>` that buffers operations and applies them in
  delivery order.
- A toy network in which one replica is the *leader* and broadcasts
  every client operation it sees to all other replicas. The
  simulator's `NoOpAdversary` provides total-order delivery for
  free, which suffices to demonstrate the SMR theorem
  experimentally.

The toy network deliberately violates fault tolerance: a single
crash of the leader silences the system, and a Byzantine leader can
equivocate. These attacks are taken up in subsequent modules.

The `tests/` directory contains:

- A property test asserting that, with `NoOpAdversary`, all replicas
  end in identical states regardless of which replica is the
  leader.
- A demonstration that, when the leader silently drops a subset of
  operations, the SMR safety property is preserved (replicas still
  agree on the prefix they did receive) but eventual delivery is
  not (some operations never appear).

## References

- Lamport, "Time, Clocks, and the Ordering of Events in a
  Distributed System", CACM 1978.
- Lamport, "Using Time Instead of Timeout for Fault-Tolerant
  Distributed Systems", TOPLAS 1984.
- Schneider, "Implementing Fault-Tolerant Services Using the State
  Machine Approach: A Tutorial", ACM Computing Surveys 1990.
  [`schneider1990`].
- Hadzilacos and Toueg, "Fault-Tolerant Broadcasts and Related
  Problems", in Distributed Systems (2nd ed.), Addison-Wesley
  1993. [`ht1993`].

See also [`HISTORY.md`](../HISTORY.md), section "Pre-1980" and
"1986 to 1999".
