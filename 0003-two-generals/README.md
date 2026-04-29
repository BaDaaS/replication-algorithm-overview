# 0003: The Two Generals Problem

## Historical context

The Two Generals problem first appeared in the 1975 paper by
Akkoyunlu, Ekanadham, and Huber, "Some Constraints and Tradeoffs
in the Design of Network Communications" [`aeh1975`]. Jim Gray's
1978 chapter "Notes on Database Operating Systems" popularised it,
and it became the standard pedagogical impossibility result for
the unreliability of asynchronous message passing.

Two generals camped on opposite sides of an enemy valley want to
attack at the same time. Their only means of communication is a
messenger who must cross the valley and may be captured. They want
*coordinated* attack: both attack or neither attacks. Can they
agree on a time?

This module formalises the negative answer and builds the bridge
to FLP (module 0005), of which Two Generals is the elementary
two-process special case.

## System and threat model

- Two processes `G1, G2`.
- A bidirectional channel that may *lose* messages but does not
  corrupt them. (Lossy = an asynchronous channel with adversarial
  drops.)
- Both processes are *correct* (no Byzantine behaviour).
- The processes wish to reach a *binary agreement*: each ends in
  state `Attack` or `Retreat` and they end in the same state.

The threat is the lossy channel: the adversary chooses which
messages to deliver and which to drop.

## Theory

### Theorem (Two Generals impossibility)

There is no deterministic protocol that guarantees both generals
decide on a common attack time in finite expected message
exchanges, given a lossy channel that may drop any message.

*Proof.* Suppose such a protocol exists. Among all its possible
runs, choose one in which both generals decide `Attack` and the
total number of messages exchanged is *minimum*. Let `m` be the
last message of this run. Two cases:

- *m is sent from G1 to G2.* Then G1 decides without confirmation
  that `m` arrived. The adversary can drop `m`. G1 must still
  decide the same way (the protocol is deterministic and G1's
  view through the act of sending is the same). G2 has not
  received `m` and so has a strictly smaller view; if G2 still
  decides `Attack`, then the truncated run (without `m`) is also
  an `Attack`-deciding run, contradicting minimality. If G2
  decides `Retreat`, the agreement property fails.
- *m is sent from G2 to G1.* Symmetric.

In both cases we contradict either minimality or agreement. QED.

### Corollary (no probabilistic improvement under fully
lossy channels)

If the adversary can drop *every* message with non-zero
probability, even probabilistic protocols cannot achieve
agreement with probability 1. (They can achieve probability `1 -
epsilon` by exchanging enough messages, but not exactly 1.) This
is sometimes called the "asymptotic Two Generals" result and is
the easy first lemma of any randomised consensus argument.

### Relation to FLP and Byzantine Generals

- *FLP* (module 0005) generalises Two Generals to any number of
  processes and weakens "lossy channels" to "messages may be
  arbitrarily delayed". The proof in FLP uses a similar
  bivalence-by-extension argument.
- *Byzantine Generals* (module 0004) replaces "lossy channel" with
  "one general is malicious", obtaining a different impossibility
  threshold (`f < n/3`) under synchrony.

### Practical reductions

- *TCP three-way handshake.* The TCP SYN/SYN-ACK/ACK protocol
  does not solve Two Generals; it gives only a probabilistic and
  *one-sided* convergence: after the third packet, both ends are
  *very likely* to share a connection. Classic readers (Tanenbaum)
  point this out as a deliberate engineering compromise.
- *Two-phase commit.* 2PC blocks under coordinator failure
  precisely because of Two Generals: a participant in `Prepared`
  state cannot infer the coordinator's decision without further
  messages.
- *Eventual consistency.* Many distributed systems sidestep Two
  Generals by relaxing "agree at the same time" to "agree
  eventually".

## Practice

### Where the problem actually bites

The Two Generals impossibility is an exhortation, not a barrier.
Practical systems agree by *bounding* the lossy adversary or by
*relaxing* the goal:

- *Bounded loss.* Networks with redundancy and retries achieve
  effective reliability: under a bounded loss probability `p`, a
  protocol with `k` retries fails with probability `p^k`.
- *Relaxed agreement.* Eventual consistency, AP-systems, and
  CRDTs replace "exact agreement at a fixed time" with "agreement
  in the limit". The Two Generals counterargument no longer
  applies.

### Operational rule

Production systems are designed so that the *cost* of momentary
disagreement is bounded. A cash register might tolerate a few
seconds of inconsistency; a missile control system would not.

## Formalisation aspects

The impossibility is a *no-protocol* theorem. To formalise it we
need to quantify over all protocols. The standard pseudo-Lean
shape:

```text
structure Protocol where
  state    : Type
  init     : state
  send     : state -> Time -> Option (state * Message)
  recv     : state -> Message -> Time -> state
  decide   : state -> Time -> Option Decision

theorem two_generals_impossibility :
    forall (P : Protocol),
    forall (sched : LossySchedule),
      not (
        forall t, agreement_holds P sched t
        /\ both_decide P sched t
      ) := by
  -- minimum-message argument: contradicts agreement or
  -- minimality.
  sorry
```

The minimum-message argument is *finitary*: it operates on the
shortest run satisfying both generals decide. The corresponding
formalisation is a well-founded induction on message count plus a
case split on the last message's direction.

## Verifiability and circuit encoding

**Tag: `na`.**

There is no protocol to encode here, only the absence of one. The
result is a structural result about message-passing models. In
verifiable settings, the Two Generals problem reappears in the
guise of the *cross-chain finality* question: a light client of
chain `A` watching a header of chain `B` cannot synchronously
confirm that `B` has observed `A`'s header without an
out-of-protocol assumption.

The standard solution in verifiable replication is to require an
explicit *acknowledgement proof*: chain `B`'s next header includes
a SNARK-verifiable receipt of having processed chain `A`'s update.
This does not solve Two Generals (the receipt itself can be lost),
but it bounds the number of messages required for high-probability
convergence and provides a succinct audit trail. zk-bridges and
IBC light-client packets implement variants of this pattern.

## Known attacks and limitations

The result is robust:

- It assumes deterministic protocols. Randomised protocols can
  achieve probability `1 - epsilon` agreement but not 1.
- It assumes only loss, not corruption. Adding corruption makes
  the problem strictly harder.
- It assumes a binary decision. Generalisations to multi-valued
  decisions reduce to this case.

Common confusions:

- "TCP solves Two Generals." It does not. TCP gives bounded-loss
  convergence, not certainty.
- "We can solve it by sending lots of messages." More messages
  reduce failure probability but do not eliminate it.

## Implementation notes

The crate provides a tiny *attempted* Two Generals protocol over
the simulator. Two nodes try to coordinate an attack via repeated
acks. The test runs the protocol under (i) a reliable adversary
(`NoOpAdversary`) and shows agreement, and (ii) an adversary that
drops the last in-flight message and shows non-agreement: one
general decides `Attack`, the other decides `Retreat`.

This is not a "solution" to Two Generals (none exists). It is an
empirical demonstration of the impossibility on a finite bound.

## References

- Akkoyunlu, Ekanadham, Huber, "Some Constraints and Tradeoffs in
  the Design of Network Communications", SIGOPS 1975. [`aeh1975`].
- Gray, "Notes on Database Operating Systems", LNCS 60, 1978.
- Tanenbaum and van Steen, "Distributed Systems: Principles and
  Paradigms", chapter 2 (the standard pedagogical statement).

See also [`HISTORY.md`](../HISTORY.md), section "Pre-1980".
