# Module 0003 Solutions

## Solution 1 [T]: complete the impossibility proof

A *run* of a protocol `P` against a lossy schedule `S` is a finite
or infinite sequence of triples `(t_i, src_i, dst_i, msg_i)`
such that the protocol's `send` and `recv` functions, applied
in order, yield the schedule's deliveries. A run *witnesses
agreement* if both generals reach the `Attack` decision at some
finite time.

By assumption, `P` solves Two Generals: there exists a run `R*`
witnessing agreement of length `k_min < infinity`. Take `R` of
this minimal length; let `m` be its last message.

*Case A:* `m` is from `G1` to `G2`. Consider the truncated run
`R'` in which `m` is dropped. `G1`'s view in `R'` is the same up
to and including the act of sending `m`; in particular `G1`'s
state machine produces the same outputs (it has already decided
in `R` because `R`'s last action is sending `m`, after which `G1`
might or might not decide).

- *Subcase A1:* `G1` had decided before `m`. Then in `R'`, `G1`
  has the same decision (`Attack`). For agreement to hold in
  `R'`, `G2` must also decide `Attack`. But `G2`'s view in `R'`
  is strictly shorter than in `R` (it lacks `m`). If `G2` already
  decided in `R'`, then truncating `R` at `G2`'s decision gives
  an even shorter agreement-witnessing run, contradicting
  `k_min`. If `G2` did not decide in `R'` (or decided `Retreat`),
  agreement fails.
- *Subcase A2:* `G1` decides only after sending `m`. Then in `R`,
  `G1` decides `Attack` having only sent `m` (no further actions
  before deciding). The same trace, with `m` dropped, is
  indistinguishable from `G1`'s viewpoint, so `G1` still decides
  `Attack` in `R'`. The argument continues as in A1.

*Case B:* symmetric.

In each case we contradict `k_min` or agreement, so `P` does not
exist. QED.

## Solution 2 [T]: probabilistic Two Generals

Fix any deterministic protocol `P` of `r` rounds. The adversary
strategy "drop every message independently with probability `p`"
results in a uniformly random schedule. With probability at least
`p^r`, *all* `r` messages are dropped; in this run, neither
general can have observed any communication and so both must
decide based on their initial state alone. If the protocol is
deterministic, this means both generals make a fixed decision
without any input. To satisfy validity (the proposer's input is
respected when no failures), the fixed decision must equal the
proposer's input. But if the input was `Attack`, both generals
would attack while the proposer might be alone; if `Retreat`,
both generals retreat while the proposer's order goes unheard.

Hence `Pr [agreement fails] >= p^r`, which is non-zero for any
finite `r`.

The bound is tight up to constants: a protocol that sends `r`
messages in each direction and decides on the majority of
received acknowledgements achieves failure probability `O(p^r)`.

## Solution 3 [P]: TCP and Two Generals

- *Three-way handshake.* The third packet (ACK) is the *witness*
  that both ends have the same connection state. With only two
  packets (SYN, SYN-ACK), the server cannot be sure the client
  received the SYN-ACK. The third ACK is structurally a probe of
  the cross-process state. It reduces the failure probability
  from `p` to `p^2`.
- *SYN cookies.* Defend against connection exhaustion (a TCP-
  specific concern) by encoding the connection-state hash into
  the SYN-ACK's sequence number, so the server need not keep
  per-connection state until after the third ACK. This is
  orthogonal to Two Generals but illustrates how protocol
  designers use cryptography to compress state.
- *Finite accept queue.* Any process must bound its memory; an
  unbounded queue is itself a denial-of-service vector. The
  bounded queue is acknowledgement that the protocol cannot
  prevent dropped connections (the second TCP-level Two Generals
  appearance: server may drop a queued connection if the queue
  overflows).

## Solution 4 [F]: pseudo-Lean lossy schedule

```text
def LossySchedule := Time -> Envelope -> Bool

def Run (P : Protocol) (S : LossySchedule) : Type :=
  -- a coinductive list of state-message pairs consistent with
  -- P's transition function and S's deliveries
  sorry

def AgreementRun (P : Protocol) (S : LossySchedule) (R : Run P S)
    : Prop :=
  exists t : Time,
    P.decide (state_of R G1 t) t = some Attack /\
    P.decide (state_of R G2 t) t = some Attack

theorem two_generals_no_protocol :
    forall (P : Protocol),
    not (forall (S : LossySchedule),
           exists (R : Run P S), AgreementRun P S R) := by
  intro P assumed
  -- Pick any S0 with the assumed agreement run R0; let k0 be its
  -- minimum message count.
  obtain <<S0, R0, hAgree>> := ...
  -- Use Nat.lt_wfRel.wf to do strong induction on message count
  -- across all possible schedules and runs.
  sorry
```

Mathlib facts needed:

- `Nat.lt_wfRel.wf` for the well-founded induction.
- `DecidableEq Envelope` to do the case split on the last
  message.
- `Filter.atTop` if extending to infinite runs.

## Solution 5 [V]: cross-chain finality

Cross-chain finality between chains `A` and `B`:

- *(i) Bounded loss.* IBC packets carry timeouts. If a packet
  is not acknowledged within the timeout, the source chain
  rolls back its commitment. The protocol does not solve Two
  Generals; it admits the loss and explicitly handles it.
- *(ii) Cryptographic commitments.* zk-bridges replace
  acknowledgements with succinct proofs of inclusion in a finalised
  block. The light client of `A` does not wait for `B` to
  acknowledge; it watches `A`'s state-root commitment to the
  proof's inclusion. This collapses the protocol to one
  message (the proof) plus on-chain inspection. The Two
  Generals impossibility is sidestepped by replacing "did `B`
  hear me?" with "is the proof valid against `A`'s state?".
- *(iii) Economic finality.* If `A` has a slashable
  acknowledgement and `B` posts a witness of equivocation, the
  cross-chain "agreement" is enforced economically rather than
  protocologically. Bridge designs that combine zk proofs with
  economic guarantees (Polyhedra, EigenLayer-secured oracles)
  exemplify this pattern.
