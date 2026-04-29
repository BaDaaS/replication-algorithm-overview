# Module 0025 Solutions

## Solution 1 [T]: quorum intersection

For `n = 3f + 1, Q_c = Q_f = 2f + 1`:

```
|Q_c ∩ Q_f| >= 2(2f + 1) - n = 4f + 2 - 3f - 1 = f + 1 > f.
```

So the intersection exceeds `f`, satisfying the Fast Paxos
safety invariant. This is the n = 3f + 1 variant, sometimes
written as "Fast Paxos with `n = 3f + 1`".

## Solution 2 [P]: contention scenario

Client A proposes `(slot=1, value=A)`. Client B proposes
`(slot=1, value=B)` concurrently. Acceptors split: some accept
A first, others accept B first. Neither value gets `2f + 1`
accepteds.

The leader notices the conflict (collects ACCEPTEDs from
acceptors and sees disagreement). Runs a classic Phase 1, then
proposes the value with the majority of fast-path votes (or
either if tied). Slow path commits.

The recovery has the structural cost of a classic Paxos round
plus the failed fast-path round, totalling ~2.5 round-trips
under contention.

## Solution 3 [F]: state machine

```text
inductive FastPaxosState where
  | initial
  | fast_proposed (v : Value)
  | classic_promised (b : Ballot)
  | classic_accepted (b : Ballot) (v : Value)
  | decided (v : Value)

theorem fast_paxos_safety :
    forall (s1 s2 : FastPaxosState),
      Decided s1 v1 -> Decided s2 v2 -> v1 = v2 := by
  -- Case analysis on whether each decision was fast or slow.
  -- Fast: 2f + 1 accepteds, intersect any classic recovery.
  -- Slow: classic Synod safety.
  sorry
```

## Solution 4 [V]: verifiable Fast Paxos

Per commit:

- *Fast path.* BLS-aggregated cert from `2f + 1` acceptors:
  ~10^6 constraints, plus a public-input flag indicating
  "fast".
- *Slow path.* Classic Multi-Paxos cost: ~10^6 constraints
  (Phase 2 only, Phase 1 amortised across slots).

The verifier reads the path flag and applies the corresponding
quorum-size check (`2f + 1` for fast, `f + 1` for slow).
Constraint count is roughly equal in both cases, with the
public-input flag adding ~1 constraint.
