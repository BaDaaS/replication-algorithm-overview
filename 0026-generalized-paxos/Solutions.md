# Module 0026 Solutions

## Solution 1 [T]: topological-sort equivalence

Let `T_1, T_2` be two topological sorts of a `C-Struct` whose
commands all commute pairwise. We show that applying `T_1`
and `T_2` to an initial state `s_0` produces the same final
state.

`T_1` and `T_2` differ by a sequence of *adjacent
transpositions* (any two topological sorts of a finite
partial order differ by adjacent swaps; this is a standard
fact in combinatorics). Each adjacent transposition swaps
`(c_i, c_{i+1})` to `(c_{i+1}, c_i)`. By the commutativity
hypothesis, the final state is unchanged.

By induction on the number of transpositions, the two sorts
yield the same final state. QED.

## Solution 2 [P]: KV with ranges

```rust
enum KvCommand {
    Write { key: u32, value: u32 },
    PointRead { key: u32 },
    RangeRead { lo: u32, hi: u32 },
}

impl Commutes for KvCommand {
    fn commutes_with(&self, other: &Self) -> bool {
        use KvCommand::*;
        match (self, other) {
            (Write { key: k1, .. }, Write { key: k2, .. }) =>
                k1 != k2,
            (Write { key, .. }, PointRead { key: k2 }) |
            (PointRead { key: k2 }, Write { key, .. }) =>
                key != k2,
            (PointRead { .. }, PointRead { .. }) => true,
            (Write { key, .. }, RangeRead { lo, hi }) |
            (RangeRead { lo, hi }, Write { key, .. }) =>
                !(*lo <= *key && *key <= *hi),
            (PointRead { .. }, RangeRead { .. }) |
            (RangeRead { .. }, PointRead { .. }) => true,
            (RangeRead { .. }, RangeRead { .. }) => true,
        }
    }
}
```

## Solution 3 [F]: pseudo-Lean Commutes

```text
class Commutes (alpha : Type) where
  commute : alpha -> alpha -> Bool
  commute_correct :
    forall (s : State) (a b : alpha),
    commute a b = true ->
    apply_machine (apply_machine s a) b =
    apply_machine (apply_machine s b) a

instance : Commutes KvWrite where
  commute := fun a b => a.key != b.key
  commute_correct := by
    -- Different keys imply independent state cells, so order
    -- doesn't matter.
    sorry
```

## Solution 4 [V]: in-circuit C-Struct

For `k = 10` commands:

- *Edges:* up to `k * (k - 1) / 2 = 45` partial-order edges.
- *Per-edge constraint:* "if edge `(i, j)` exists then `i`
  precedes `j` in the chosen linearisation": ~10 constraints.
- *Compatibility check:* for every pair `(i, j)` not in the
  partial order, verify `commutes(c_i, c_j) = true`: ~k^2 / 2
  hash invocations on the command pair, ~50 invocations,
  each ~200 constraints in Poseidon = ~10k.

Total: ~50 * 10 + 50 * 200 = ~10.5k constraints for the
C-Struct verification, plus the BLS quorum cert at ~10^6.

The C-Struct cost is sub-dominant. For larger batches, the
quadratic per-pair check becomes significant; production
typically uses small batches (~10) per commit.
