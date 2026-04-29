# Module 0031 Solutions

## Solution 1 [T]: optimal Q1/Q2 for n = 7

The intersection requirement: any Q1 must intersect any Q2.
For "all subsets of size k1" vs "all subsets of size k2",
intersection is non-empty iff `k1 + k2 > n`.

For n = 7:

| Q1 size | Q2 size | constraint | per-commit cost (Q2) |
| ------- | ------- | ---------- | -------------------- |
| 7       | 1       | 7+1=8>7    | 1 sig                |
| 6       | 2       | 8>7        | 2 sigs               |
| 5       | 3       | 8>7        | 3 sigs               |
| 4       | 4       | 8>7 (=)    | 4 sigs (classic)     |

Minimum per-commit: Q1 = 7, Q2 = 1. Trade-off: leader change
needs all 7 acceptors. Acceptable for stable-leader workloads.

## Solution 2 [P]: grid quorums

```rust
fn grid_quorums(n: usize) -> FlexibleQuorums {
    let side = (n as f64).sqrt() as usize;
    assert_eq!(side * side, n, "n must be square");
    let mut p1 = Vec::new();
    let mut p2 = Vec::new();
    for c in 0..side {
        let column: BTreeSet<NodeId> = (0..side)
            .map(|r| NodeId((r * side + c) as u32))
            .collect();
        p1.push(column);
    }
    for r in 0..side {
        let row: BTreeSet<NodeId> = (0..side)
            .map(|c| NodeId((r * side + c) as u32))
            .collect();
        p2.push(row);
    }
    FlexibleQuorums { p1, p2 }
}
```

For n = 9 (3x3 grid): Q1 size 3 (column), Q2 size 3 (row).
Each row intersects each column at exactly one element.

## Solution 3 [F]: pseudo-Lean

```text
class FlexibleQuorum (n : Nat) where
  p1 : Set (Finset (Fin n))
  p2 : Set (Finset (Fin n))
  intersect :
    forall q1 q2,
    q1 ∈ p1 -> q2 ∈ p2 -> (q1 ∩ q2).Nonempty

theorem flexible_paxos_safety
    {FQ : FlexibleQuorum n}
    : SynodSafety := by
  -- Intersection lemma replaces classic Paxos's majority
  -- intersection. The Synod proof composes with this lemma.
  intro b1 b2 v1 v2 h1 h2
  obtain <<a, ha1, ha2>> := FQ.intersect ...
  ...
```

## Solution 4 [V]: workload-tuned

99% commits, 1% leader changes:

- Commit cost: Q2 size sigs * frequency = c * 0.99 * Q2 sigs.
- Leader change cost: Q1 size sigs * 0.01 * Q1 sigs.

Total = `0.99 * Q2 + 0.01 * Q1` per operation amortised.

Subject to Q1 + Q2 > n. Minimise: Q2 = 1, Q1 = n. Total =
0.99 + 0.01n. For n = 5: 0.99 + 0.05 = 1.04 sigs per op
amortised, vs classic majority = 0.99 * 3 + 0.01 * 3 = 3
sigs per op. ~3x speedup.

Production: this is exactly why systems like CockroachDB
allow "near-line" replicas (cheap Phase 2 quorum) plus a few
canonical replicas (Phase 1 quorum).
