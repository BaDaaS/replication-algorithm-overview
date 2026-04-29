# Module 0038 Solutions

## Solution 1 [T]: linearisability

Per key, CASPaxos runs Synod for each CAS. By Synod safety,
each ballot decides on a unique value. The f-application
step computes the new value from the latest accepted; quorum
intersection ensures all proposers see the same latest.

Linearisability: each CAS is positioned at its decision
point in the global order; reads return the latest committed
value. QED.

## Solution 2 [P]: leader pinning per key

Reserve one process as the leader per key. The leader runs
Phase 1 once; subsequent CAS operations skip Phase 1 and go
directly to Phase 2.

Cost amortised: Phase 1 (~10^6 constraints in verifiable
form) divided across N CAS operations; per-CAS cost drops to
Phase 2 alone (~10^6 / N + 10^6).

For N >> 1, per-CAS cost approaches the Multi-Paxos
amortisation: ~10^6 + 10^6 ~= 2 * 10^6, but the constant is
smaller per key because there's no log.

## Solution 3 [F]: pseudo-Lean

```text
structure CASPaxosState where
  promised : Nat
  accepted : Option (Nat × Value)

def cas_step
    (state : CASPaxosState) (b : Nat) (f : Value -> Value)
    : CASPaxosState × Value :=
  let v_new := f (state.accepted.map Prod.snd |> default v_init)
  ({ promised := b, accepted := some (b, v_new) }, v_new)

theorem caspaxos_linearisable :
    forall (key : Key) (ops : List CASOp),
    LinearisableExecution ops := by
  -- Per-key Synod safety + f-application.
  sorry
```

## Solution 4 [V]: verifiable CAS

Per CAS:

- Phase 1 BLS quorum cert: ~10^6 constraints.
- f-witness: ~few constraints (depends on f's complexity).
- Phase 2 BLS quorum cert: ~10^6 constraints.

Total per CAS: ~2 * 10^6 constraints. With per-key leader
pinning amortising Phase 1 across N CAS: ~10^6 per CAS.

Production: Aleo's snarkVM can express this directly; CAS
is a primitive in the snarkVM record model.
