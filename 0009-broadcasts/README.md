# 0009: Reliable, Causal, and Atomic Broadcast

## Historical context

Broadcast primitives are the *atoms* of distributed-systems
programming. A long line of work, surveyed by Hadzilacos and
Toueg in their 1993 chapter "Fault-Tolerant Broadcasts and
Related Problems" [`ht1993`], formalises three layers:

- *Reliable broadcast (RB)* (Bracha 1987 in the asynchronous
  Byzantine model; folkloric earlier under crashes): all
  honest processes deliver the same set of messages.
- *Causal broadcast (CB)* (Birman-Schiper-Stephenson 1991):
  reliable broadcast plus a partial order respecting causality
  (`->` in Lamport's sense).
- *Atomic broadcast (AB)*, also called *total-order broadcast*
  (TOB): reliable broadcast plus a total order on deliveries.

The central result of this module is the equivalence of atomic
broadcast and consensus (Hadzilacos-Toueg 1993, Theorem 5.7).
Every later consensus module in this course is, structurally, a
TOB construction.

## System and threat model

Parametric in the failure model. We give the standard
crash-stop and Byzantine variants where the bounds differ.

## Theory

### Definition (reliable broadcast)

A broadcast primitive `RB-broadcast(m), RB-deliver(m)` such that:

- *Validity.* If an honest process broadcasts `m`, then every
  honest process eventually delivers `m`.
- *Integrity.* Each `m` is delivered at most once at each
  process, and only if some honest process broadcast it.
- *Agreement.* If an honest process delivers `m`, then every
  honest process eventually delivers `m`.

Bracha's 1987 asynchronous reliable broadcast (see Bracha-Toueg
1985) implements this with `O(n^2)` messages per broadcast in
the Byzantine model with `f < n / 3`.

### Theorem (Bracha's RB safety)

Under `f < n / 3` Byzantine faults in the asynchronous model,
Bracha's "echo-ready" protocol satisfies validity, integrity,
and agreement.

*Proof.* The standard double-counting:

- For an `m` to be delivered, at least `2f + 1` `ready(m)`
  messages must be received. Of these, at least `f + 1` are
  from honest processes.
- An honest process sends `ready(m)` only after seeing
  `n - f` `echo(m)` messages, of which at least `f + 1` are
  honest.
- An honest process sends `echo(m)` only on receipt of
  `init(m)` from the broadcaster. So the message `m` exists.

Two distinct `m, m'` cannot both have `2f + 1` `ready` because
that would require `f + 1` honest processes to ready `m` and
the other `f + 1` to ready `m'`, contradicting `f < n / 3`.
QED.

### Definition (causal broadcast)

A broadcast that, in addition to RB, satisfies:

- *(CB).* If `m_1 -> m_2` (Lamport's happens-before), then
  every honest process delivers `m_1` before `m_2`.

Implementation: each broadcaster timestamps its message with a
vector clock, and recipients buffer until all causal
predecessors have been delivered. Birman-Schiper-Stephenson
1991.

### Definition (atomic / total-order broadcast)

A broadcast that, in addition to RB, satisfies:

- *(TOB).* If two honest processes deliver `m_1` and `m_2`,
  they deliver them in the same relative order.

### Theorem (Hadzilacos-Toueg 1993)

In any failure model, atomic broadcast and (binary or
multivalued) consensus are *equivalent*: a primitive solving
either gives a primitive solving the other.

*Proof.*

- *(consensus -> AB).* Run consensus repeatedly: in iteration
  `i`, processes propose the head of their FIFO send buffer (or
  a special "no-op"). The decided value is the next message in
  the global order. Reliable delivery of decisions follows from
  consensus's agreement and validity.
- *(AB -> consensus).* Each process AB-broadcasts its proposed
  value. The first AB-delivered value is the consensus
  decision. Agreement follows from AB's total order; validity
  follows from RB's validity.

QED.

### Theorem (RB / CB / TOB hierarchy)

CB is RB plus FIFO causal order; TOB is RB plus a total order.
TOB is strictly stronger than CB, which is strictly stronger
than RB. Each can be implemented from the next-stronger one
trivially; the converse implementations require additional
structure (vector clocks for CB, consensus for TOB).

### Communication complexity

- *RB:* `O(n^2)` messages per broadcast (Bracha 1987 lower
  bound matches).
- *CB:* `O(n^2)` plus vector-clock overhead per message.
- *TOB:* same as consensus, `Omega(n * f)` per broadcast
  (Dolev-Reischuk).

## Practice

### Where each broadcast is used

- *RB.* Used in Byzantine reliable broadcast layers and as a
  primitive for asynchronous BFT (HoneyBadger, Dumbo).
- *CB.* Causal-order multicast in production gossip systems
  (CRDT-based databases). Riak, Cassandra hinted handoff.
- *TOB.* The agreement primitive of every SMR-based system.
  Tendermint, HotStuff, Aptos's Quorum Store all expose TOB
  semantics.

### Modern variants

- *Multi-shot RB.* A single execution disseminates many
  messages amortising over the `n^2` ready exchange. Aleph
  uses this.
- *Avid (asynchronous verifiable information dispersal).*
  Erasure-coded RB for large messages; bandwidth `O(|m| / n)`
  per process. Used in HoneyBadger for transaction batches.
- *Bracha + threshold sigs.* Compresses the `2f + 1` `ready`
  collection into a single threshold signature. Used in
  Mostefaoui-Moumen-Raynal 2014/2015.

### Gossip layer

Every production system layers RB / CB / TOB over a *gossip*
protocol that carries the actual bytes. CometBFT's mempool
broadcast, libp2p's gossipsub, Sui's gossip layer all
implement CB-like FIFO-causal semantics by default.

## Formalisation aspects

```text
class ReliableBroadcast (M : Type) where
  broadcast : NodeId -> M -> Effect
  deliver   : NodeId -> M -> Effect

  validity   : forall p m, IsHonest p -> Broadcast p m ->
               forall q, IsHonest q -> Eventually (Deliver q m)
  integrity  : forall p m, AtMostOnce (Deliver p m) /\
               (Deliver p m -> exists q, IsHonest q /\ Broadcast q m)
  agreement  : forall p m, IsHonest p -> Deliver p m ->
               forall q, IsHonest q -> Eventually (Deliver q m)

class AtomicBroadcast (M : Type) extends ReliableBroadcast M where
  total_order : forall p q m1 m2,
    IsHonest p -> IsHonest q ->
    DeliverOrder p m1 m2 -> DeliverOrder q m1 m2
```

The Hadzilacos-Toueg equivalence becomes a theorem
`AtomicBroadcast M iff Consensus M`. cslib's
`InferenceSystem` is a natural fit for the deduction-style
specification of these properties; the derived consensus
construction follows the proof in [`ht1993`].

## Verifiability and circuit encoding

**Tag: `friendly`.**

A reliable broadcast layer is SNARK-friendly: each delivery is
a witness of `2f + 1` signatures (or one threshold signature)
on the same content. Verifying RB delivery in circuit reduces
to verifying the threshold signature's validity, which is a
single pairing check for BLS.

Atomic broadcast is, by Hadzilacos-Toueg, equivalent to
consensus in the verifiability sense as well: a SNARK proof of
TOB delivery is a SNARK proof of consensus on the delivered
sequence. Mina's Pickles essentially produces this object: the
chain prefix is the TOB-ordered sequence; the proof attests to
the consensus that produced it.

Causal broadcast is *not* directly SNARK-friendly: the
vector-clock predicates require comparing per-process timestamp
vectors, which has `O(n)` constraints per delivery. Production
systems that need verifiable causal order (e.g. some zk-based
chat protocols) typically commit to the vector-clock hash and
prove inclusion paths.

## Known attacks and limitations

- *Byzantine RB requires `f < n / 3`.* Below this threshold,
  no asynchronous Byzantine RB exists (Toueg 1984).
- *Crash-stop weakening.* With crash failures, RB is solvable
  with `f < n / 2` (or `f < n` with stable storage).
- *RB is not consensus.* RB does not provide a total order;
  bridging from RB to TOB requires consensus or a randomised
  coin.

## Implementation notes

The crate provides Bracha's asynchronous reliable broadcast
in a simulator-friendly form:

- `BrachaNode<M>` participates in echo-ready exchanges over an
  opaque message type `M: Clone + Eq + Hash`.
- A test broadcasts a message from one node and verifies that
  every other node delivers it under `NoOpAdversary` and a
  `OmissionAdversary` that drops a fraction below the safe
  threshold.

The implementation tracks per-message echo and ready counts
and triggers delivery once `2f + 1` `Ready` messages have
arrived. Threshold parameters are configurable.

## References

- Bracha, "Asynchronous Byzantine Agreement Protocols",
  Information and Computation 1987.
- Bracha and Toueg, "Asynchronous Consensus and Broadcast
  Protocols", JACM 1985.
- Hadzilacos and Toueg, "Fault-Tolerant Broadcasts and Related
  Problems", in Distributed Systems (2nd ed.), Addison-Wesley
  1993. [`ht1993`].
- Birman, Schiper, Stephenson, "Lightweight Causal and Atomic
  Group Multicast", TOCS 1991.
- Mostefaoui, Moumen, Raynal, "Signature-Free Asynchronous
  Byzantine Consensus with `t < n / 3` and `O(n^2)` Messages",
  PODC 2014.

See also [`HISTORY.md`](../HISTORY.md), section "1986 to 1999".
