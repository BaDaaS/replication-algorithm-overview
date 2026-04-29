# Module 0036 Solutions

## Solution 1 [T]: read linearisability

Two cases:

- *Local read returns clean(v).* `clean(v)` is set only after
  the tail acks; so `v` is a committed write. Reading `v` is
  consistent with linearisable order.
- *Tail-fallback read.* The tail returns its current
  committed version. Same argument.

In both cases the read returns a committed value, satisfying
linearisability (the read is positioned just after the
returned write in the linearisation).

## Solution 2 [P]: throughput modelling

Workload: 90% reads, 10% writes.

Vanilla Chain Replication: writes propagate through n
replicas; reads only at tail. Total per-replica work:

- Tail: `0.1 * write_cost + 0.9 * read_cost`.
- Other replicas: `0.1 * write_cost` only.

Tail is the bottleneck; throughput limited by tail.

CRAQ: reads distributed across all n replicas (clean state).

- Each replica: `0.1 * write_cost + 0.9 * read_cost / n`.

Throughput improvement: `n / (1 + 0.9 * 9 / 1) ~= n / 9.1`.
For n = 5: ~5x read throughput vs vanilla.

For 99% reads, the improvement approaches `n` (linear
scaling).

## Solution 3 [F]: pseudo-Lean clean/dirty

```text
structure KeyState where
  clean : Option Value
  dirty : List (Version × Value)

def local_read (s : KeyState) : Option Value :=
  if s.dirty.isEmpty then s.clean else none

theorem local_read_returns_committed
    (s : KeyState) (v : Value) :
    local_read s = some v -> CommittedValue v := by
  intro h
  -- s.dirty is empty, s.clean = some v
  -- by invariant, s.clean is set only after tail ack
  sorry
```

## Solution 4 [V]: read attestations

Per-read proof:

- Replica's signature on (key, returned_value, "clean"
  flag): ~3k constraints.
- Merkle path from the value to the chain's committed root:
  ~10k constraints.
- Verifier check: signature verifies; Merkle path matches the
  committed-state root.

Total per read: ~13k constraints. Per write: same as Chain
Replication (~15k for chain of 5).

The verifier-side cost is ~constant per read, regardless of
replica count, allowing CRAQ to scale read verification.
