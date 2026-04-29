# Module 0020 Solutions

## Solution 1 [T]: VR safety invariant

Invariant: For all view numbers `v` and op numbers `op_no`, if
`op` was committed at `op_no` in any view `v' < v`, then the
primary of view `v` has `op` at position `op_no` in its log.

Proof. Suppose `op` was committed at `op_no` in view `v'`. By
the commit rule, `op` is on at least `f + 1` replicas' logs.
The view-`v` primary collects `f + 1` `DO_VIEW_CHANGE`
messages. By inclusion-exclusion in `n = 2f + 1`, the two
sets intersect in at least one replica. That replica's log
contains `op` at `op_no`. The longest-log rule preserves it.

## Solution 2 [T]: VR-Paxos translation

| VR concept           | Paxos concept    |
| -------------------- | ---------------- |
| view number `v`      | ballot `b`       |
| primary of view `v`  | leader of ballot |
| `PREPARE(v, op_no, op)` | `accept(b, op_no, op)` |
| `PREPARE_OK`         | `accepted`       |
| `START_VIEW_CHANGE`  | `prepare(b)`     |
| `DO_VIEW_CHANGE`     | `promise(b, log)`|
| `START_VIEW(v, log)` | `accept(b, log)` |
| primary's commit threshold `f + 1` | acceptors' `f + 1` |

The mapping is a bijection on protocol states and messages.
The two protocols are equivalent up to renaming.

## Solution 3 [P]: view-change extension

State machine:

```
Normal -> View-Change-Initiated -> Awaiting-DVC -> View-Change-Done -> Normal
```

Messages: `START_VIEW_CHANGE(v + 1)`, `DO_VIEW_CHANGE(v + 1, log)`,
`START_VIEW(v + 1, log)`. The new primary collects `f + 1`
`DO_VIEW_CHANGE`, picks the longest log, and broadcasts
`START_VIEW`.

Test: kill the primary node (simulate crash), verify a backup
becomes primary in view 1 and resumes operation.

## Solution 4 [P]: persistence

Per Liskov-Cowling 2012, persist:

- `view_num`: prevents accepting old-view messages on recovery.
- `log[]`: the committed operations.
- `op_num`, `commit_num`: where we are in the log.
- `latest accepted view-change votes`: prevents
  contradicting an in-progress view change.

These are the analogues of Raft's `currentTerm`, `votedFor`,
`log[]`. Production: the WAL must be fsync'd before responding
to peers.

## Solution 5 [F]: pseudo-Lean

```text
structure VrState where
  view_num    : Nat
  op_num      : Nat
  commit_num  : Nat
  log         : List Operation
  status      : Status

inductive Status where
  | normal
  | view_change
  | recovering

theorem vr_safety
    (n f : Nat) (h : 2 * f + 1 = n)
    (sched : AsyncSched) (corrupt : CrashRecovery f) :
    Linearisable VR sched corrupt := ...
```

CSLib's `LTS` works because VR transitions are well-defined
per view. View change is a "labelled transition" between LTS
phases.

## Solution 6 [V]: verifiable VR commit

Per commit:

- BLS-aggregated PrepareOk certs from `f + 1` replicas:
  one pairing, `~10^6` constraints.
- View-number consistency: ~constraints.
- Op_num monotonicity: ~constraints.

Total per commit: `~10^6` constraints. Same as multi-Paxos.
Recursion across slots gives constant proof.

## Solution 7 [V]: view-change in circuit

Per view change:

- `f + 1` BLS-aggregated `DO_VIEW_CHANGE` certs: each carries
  a log commitment (Merkle root). One pairing per cert.
  `~10^6 * (f + 1)` constraints (without nesting).
- Longest-log selection: `~constraints` proportional to log
  length plus comparisons. ~`10^4`.
- Final `START_VIEW` broadcast: 1 BLS, `~10^6`.

Total per view change: `~10^7` constraints (dominated by `f +
1` DO_VIEW_CHANGE pairings). With recursive aggregation, the
final view-change proof can be `O(1)`.

The view-change cost is significantly higher than normal-case;
production verifiable BFT amortises by making view changes
rare (HotStuff-2's optimistic responsiveness has typical
`O(1)`-round commit, view change only on faults).
