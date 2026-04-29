# 0024: Cheap Paxos

## Historical context

Lamport and Massa's 2004 paper "Cheap Paxos" introduced a
resource-saving variant: with `n = 2f + 1` acceptors,
normal-case operation only needs `f + 1` quorum; the remaining
`f` *auxiliary* acceptors participate only during failure
recovery. Auxiliaries can be cheap (no stable storage,
intermittent availability) most of the time.

The motivation is operational: in a deployment with `n = 5`
servers (`f = 2`), three of them are continuously committed
storage nodes; two can be off-the-shelf machines that come
online only during failover. The cost saving is significant in
production hardware budgets.

## System and threat model

- **Network.** Asynchronous, reliable.
- **Failures.** Crash-recovery.
- **Stable storage.** Only `f + 1` *main* acceptors keep stable
  storage. Auxiliaries do not.
- **Goal.** Same as Paxos: SMR with `f < n / 2` resilience.

## Theory

### Algorithm

The key insight: in a `2f + 1`-acceptor Paxos, the safety
proof requires that any two majorities intersect. With `n = 2f
+ 1`, this means any majority has at least `f + 1` acceptors,
and any two intersect in at least 1.

Cheap Paxos partitions the `2f + 1` acceptors into:

- *Main acceptors:* the `f + 1` always-on, stable-storage
  group.
- *Auxiliary acceptors:* the remaining `f`, used only in
  reconfiguration.

Normal-case operation: the leader collects `f + 1` accepteds
from main acceptors. The auxiliaries are not consulted.

Failure case: if a main acceptor crashes, the leader runs a
*reconfiguration* sub-protocol to incorporate one or more
auxiliaries into the active set, restoring `f + 1` available
acceptors.

### Theorem (Cheap Paxos safety)

Cheap Paxos satisfies the Synod safety theorem under
crash-recovery for the main acceptors and intermittent
availability for auxiliaries.

*Proof.* The reconfiguration is the load-bearing step. When a
main acceptor crashes, the protocol runs a Synod instance to
agree on the new active set; this instance uses the surviving
main acceptors plus the auxiliaries, all of which must be
online. The result is a new active set of `f + 1` acceptors
(all with stable storage), and operation continues.

The proof reduces to two Synod safety arguments: one for the
data SMR, one for the configuration SMR. See Lamport-Massa
2004, Theorems 1 and 2.

### Cost analysis

Hardware: `n - (f + 1) = f` machines without stable storage.
For `n = 5, f = 2`, this saves 2 of 5 storage tiers (~40% of
hardware cost in storage-heavy deployments).

Latency: identical to standard Multi-Paxos in the normal case;
slightly worse during reconfiguration.

## Practice

### When Cheap Paxos makes sense

- *Resource-constrained deployments.* Edge computing,
  embedded systems, IoT.
- *Hybrid cloud.* Main acceptors in on-premise; auxiliaries in
  cloud (for failover only).
- *Cost-sensitive financial systems.* Stable storage is
  expensive; auxiliaries reduce per-replica TCO.

### Production deployment

Cheap Paxos is rarely used in pure form. Production systems
typically:

- Use full Paxos (5 storage replicas).
- Or use Multi-Paxos with reconfiguration (full cost, but
  flexible).

Cheap Paxos's pedagogical value is showing that Paxos's quorum
structure can be partitioned along *availability* and
*persistence* axes, opening the design space for hybrid
deployments.

## Formalisation aspects

```text
structure CheapPaxosState extends MultiPaxosState where
  main_set       : Finset NodeId
  auxiliary_set  : Finset NodeId

theorem cheap_paxos_safety
    (n f : Nat) (h : 2 * f + 1 = n)
    (sched : AsyncSchedule)
    (h_main_alive : MainAcceptorsAreReachable f) :
    SmrSafety := by
  -- two-level Synod composition
  sorry
```

The two-level structure (data SMR + configuration SMR) is the
formalisation challenge: the configuration SMR's decisions
constrain the data SMR's quorum.

## Verifiability and circuit encoding

**Tag: `friendly`.**

Verifiability is identical to Multi-Paxos for normal operation
(~10^6 constraints per commit, one BLS pairing). The
reconfiguration adds:

- One Synod instance for the configuration change: ~2 * 10^6
  constraints.
- Then a recovery for each new active acceptor: ~10^6 per
  catch-up.

Total per reconfiguration: ~3 * 10^6 to 5 * 10^6 constraints.

The verifier can check that any decision is supported by a
quorum of the *currently active* set; this requires the proof
to carry the configuration as a public input.

## Known attacks and limitations

- *Auxiliary unavailability during reconfiguration.* If
  auxiliaries are offline when needed, reconfiguration stalls.
  Production must monitor auxiliary health.
- *Configuration churn.* Frequent reconfigurations can lead to
  thrashing. Hysteresis (don't reconfigure on transient
  failures) helps.

## Implementation notes

This module reuses Multi-Paxos (module 0023) and adds:

- A `CheapPaxosNode` wrapper that distinguishes main from
  auxiliary roles.
- A reconfiguration trigger when a main acceptor stops
  responding.

The crate is intentionally minimal; full reconfiguration is
sketched in Exercises.

## References

- Lamport and Massa, "Cheap Paxos", DSN 2004.
- Lamport, "Reconfigurable Paxos", 2009.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
