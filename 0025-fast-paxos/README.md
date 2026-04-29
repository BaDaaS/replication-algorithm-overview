# 0025: Fast Paxos

## Historical context

Lamport's 2006 paper "Fast Paxos" (Distributed Computing)
introduced a Paxos variant that achieves *one-round-trip*
commits in the contention-free case, at the cost of a larger
quorum.

The classical Paxos commit takes two round trips: client to
leader, then leader to acceptors. Fast Paxos lets the client
send directly to the acceptors, with the leader only stepping
in to resolve conflicts. The price: classical Paxos's `f + 1`
quorum becomes Fast Paxos's `2f + 1` (larger), so `n` must be
`>= 3f + 1` for the fast path.

## System and threat model

- **Network.** Asynchronous, reliable.
- **Failures.** Crash-recovery; `f < n / 3` for the fast path.
- **Cryptography.** Authenticated channels.
- **Goal.** Single-decree consensus with a one-round-trip
  fast path.

## Theory

### Quorum structure

Fast Paxos uses two quorum sizes:

- *Classic quorum:* `Q_c = f + 1` (any majority in `n = 2f +
  1`).
- *Fast quorum:* `Q_f = 2f + 1` (super-majority).

When a fast-path proposal collects `Q_f` accepteds, it commits
in one round-trip. When only `Q_c` agree (with conflicts), the
leader runs a classic-style recovery to determine the value.

### Safety invariant

The Fast Paxos safety invariant requires *quorum
intersection* between any classic quorum and any fast quorum:

```
|Q_c ∩ Q_f| > f
```

For `n = 2f + 1, Q_c = f + 1, Q_f = 2f + 1`, the intersection
is `f + 1 > f`. The intersection ensures a fast decision is
visible to any later classic recovery quorum.

### Algorithm sketch

```
fast path:
  client -> all acceptors: PROPOSE(value, slot)
  acceptor: if no value yet for this slot:
    accepts the value
    replies ACCEPTED(slot, value)
  client: collects 2f + 1 ACCEPTEDs => decided

slow path (on conflict):
  if a slot has multiple proposed values:
    leader runs a classic Paxos recovery
    recovery quorum is f + 1 (classic)
    leader's chosen value = the one with the highest fast-path
                            count, or any if tied
  classic accept proceeds normally
```

### Theorem (Fast Paxos correctness)

Fast Paxos satisfies validity and agreement under
crash-recovery and `f < n / 3` for the fast path.

*Proof.* Quorum intersection ensures that any fast decision is
observed by any classic recovery. The leader's classic-path
proposal must therefore propose the fast-path value, preserving
the decision.

### Performance

- *Best case (no contention).* 1 round-trip latency.
- *Worst case (contention).* 2 round-trips (slow path) plus
  contention-resolution round.

## Practice

### Production use

Fast Paxos is rarely used directly. Variants:

- *Generalized Paxos* (Lamport 2005, module 0026): exploits
  *commutativity* of operations to allow fast-path commits in
  more cases.
- *EPaxos* (Moraru 2013, module 0027): generalises fast paths
  to multi-leader settings.
- *Mencius* (Mao 2008, module 0028): rotating fast-path leader
  per slot.

Production systems (CockroachDB, Spanner) tend to use
Multi-Paxos with leader pinning rather than Fast Paxos's
multi-proposer design, because the operational complexity
outweighs the latency saving for typical workloads.

## Formalisation aspects

```text
structure FastPaxosState extends SynodState where
  fast_quorum_size : Nat
  classic_quorum_size : Nat
  fast_path_done : Option (Slot × Value)

theorem fast_paxos_safety
    (n f : Nat) (h : n >= 3 * f + 1) :
    SynodSafety := by
  -- quorum intersection: |Q_c ∩ Q_f| > f
  sorry
```

## Verifiability and circuit encoding

**Tag: `friendly`.**

Verifiable Fast Paxos:

- Fast path: BLS-aggregated cert from `2f + 1` acceptors. One
  pairing, ~10^6 constraints.
- Slow path: classic Multi-Paxos cost, ~10^6 per commit.

Verifier checks: `(commit_type, qc, signers, threshold)`. The
threshold differs between fast and slow paths; the public
input includes which path was taken.

## Known attacks and limitations

- *Fast-path collisions.* If multiple clients propose
  conflicting values, the fast path always falls to the slow
  path. Production: rare under careful client design.
- *Larger quorum cost.* `2f + 1` of `3f + 1` requires more
  acceptors per commit. Bandwidth higher than classic Paxos.

## Implementation notes

This module is a sketch, given Fast Paxos's complexity. The
crate exposes only the quorum-size relations; the full
protocol is in the README.

## References

- Lamport, "Fast Paxos", Distributed Computing 2006.
- Howard, "Distributed Consensus Revised", PhD thesis 2019
  (covers Fast Paxos generalisations).

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
