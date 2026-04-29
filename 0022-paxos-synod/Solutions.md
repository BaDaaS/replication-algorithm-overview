# Module 0022 Solutions

## Solution 1 [T]: safety proof

Suppose `b_1 < b_2` both decide. `b_1`'s phase-2 quorum `Q_1`
and `b_2`'s phase-1 quorum `Q_2` are both majorities, so they
intersect: pick `a ∈ Q_1 ∩ Q_2`.

`a ∈ Q_1`: `a` accepted `(b_1, v_1)` in `b_1`'s phase 2. After
this, `a` has `accepted = Some(b_1, v_1)`.

`a ∈ Q_2`: `a` responded to `b_2`'s phase 1 with `Promise(b_2,
last)`. The `last` field carries `(b_a, v_a)` where `b_a` is
the highest ballot `a` has accepted, including `b_1`. So `b_a
>= b_1`.

`b_2`'s proposer collects promises and picks the value with
the highest `b_a`. Since `b_a >= b_1` for `a`, and any other
acceptor's `b_a` is also `<= b_2`, the chosen value is some
`v_a` with `b_a` between `b_1` and `b_2`.

Inductively (over the chain of ballots between `b_1` and
`b_2`), `v_2 = v_a = v_1`.

QED.

## Solution 2 [T]: dueling proposers

Proposer P1 sends Prepare(1); acceptors promise. Before P1's
Phase 2, proposer P2 sends Prepare(2); acceptors promise to 2,
breaking P1's quorum. Before P2's Phase 2, P1 sends
Prepare(3); acceptors promise to 3. Repeat.

Each proposer's Phase 2 finds an acceptor with a higher
promised ballot, so Phase 2's accepted count never reaches a
quorum. The schedule is consistent with FLP: under asynchrony,
deterministic Paxos cannot guarantee termination.

Mitigation: leader election ensures only one proposer at a time
makes progress.

## Solution 3 [P]: leader election

Use the failure detector from module 0012: the eventual leader
is the unique honest process not suspected by any other.
Implement: each proposer waits for `Omega.leader() = self`
before issuing a ballot. Once the leader is stable, no other
proposer issues new ballots, so the leader's ballot succeeds.

This is the FLP-escape: Omega + Paxos => terminating
consensus.

## Solution 4 [F]: pseudo-Lean Synod

```text
structure SynodAcceptor where
  promised  : Option Nat
  accepted  : Option (Nat × Value)

structure SynodProposer where
  ballot    : Nat
  preferred : Value

theorem synod_safety
    (n : Nat) (acceptors : Fin n -> SynodAcceptor)
    (h_quorum : forall b, acceptedBy b acceptors >= n / 2 + 1) :
    forall (b1 b2 : Nat) (v1 v2 : Value),
    b1 < b2 ->
    DecidedAt b1 v1 -> DecidedAt b2 v2 -> v1 = v2 := by
  -- chain induction on ballots between b1 and b2
  sorry
```

Mathlib's `Finset.card_inter` gives the quorum-intersection
lemma. Lamport's TLA+ proof and IronFleet's Coq proof are
direct precedents.

## Solution 5 [V]: verifiable Synod

Per Synod commit:

- Phase 1 BLS aggregate: one pairing, ~10^6 constraints.
- Phase 2 BLS aggregate: one pairing, ~10^6.

Total: ~2 * 10^6 constraints per single-decree commit. With
multi-Paxos amortising Phase 1, per-commit cost drops to
~10^6.

Production: Aptos's DiemBFT and HotStuff variants pipeline
multiple ballots; the cost is per-decision, not per-Synod-
instance.
