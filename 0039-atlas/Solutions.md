# Module 0039 Solutions

## Solution 1 [T]: quorum intersection

`q_f + q_s > n`:

`(f + ceil(f/2) + floor(c/2)) + (f + 1) > n` whenever
`2f + 1 + ceil(f/2) + floor(c/2) > n`. Since `n = 2f + 1`,
this simplifies to `ceil(f/2) + floor(c/2) > 0`, which holds
for `f >= 1`.

So fast and slow quorums always intersect. QED.

## Solution 2 [P]: tuning c for 5% conflict

5% conflict means in 5% of commands, multiple proposers
disagree on `deps`. To accommodate without falling to slow
path, set `c >= 1` to enlarge `q_f` by 1. Trade-off: slightly
larger fast quorum -> slightly higher latency, but more
fast-path successes. Net throughput improvement.

For 1% conflict: `c = 0`, smallest quorum, accept the
occasional slow-path penalty.
For 50% conflict: `c >= 4`, large quorum, but slow path
dominates anyway; consider switching to single-leader Multi-
Paxos.

## Solution 3 [F]: pseudo-Lean

```text
def atlas_fast_quorum (n f c : Nat) : Nat :=
  f + (f + 1) / 2 + c / 2

def atlas_slow_quorum (n : Nat) : Nat := n / 2 + 1

theorem atlas_quorum_intersect
    (n f c : Nat) (h_n : 2 * f + 1 = n) :
    atlas_fast_quorum n f c + atlas_slow_quorum n > n := by
  unfold atlas_fast_quorum atlas_slow_quorum
  omega

theorem atlas_safety
    (n f c : Nat) (h_n : 2 * f + 1 = n) :
    SmrSafety := by
  -- Apply intersection + Synod safety per command.
  sorry
```

## Solution 4 [V]: verifiable Atlas

Per command (typical, fast path):

- BLS aggregate from q_f replicas: ~10^6.
- Dependency-graph encoding (small batch): ~k * 10^4.

For typical k = 10: ~10^6 + 10^5 = ~1.1 * 10^6 constraints.

Compare to EPaxos: same order; Atlas is slightly less
because its q_f is smaller, but the difference is negligible
in BLS-aggregated form (one pairing regardless of q_f).

Conclusion: Atlas's verifiability profile is similar to
EPaxos. The wins are wall-clock latency, not prover cost.
