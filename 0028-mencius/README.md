# 0028: Mencius

## Historical context

Mao, Junqueira, and Marzullo published "Mencius: Building
Efficient Replicated State Machines for WANs" at OSDI 2008.
Mencius is a Multi-Paxos variant in which slot ownership
rotates among replicas: slot `i` is owned by replica `i mod
n`. Each replica is the proposer for its own slots, balancing
the load and avoiding the single-leader bottleneck.

Mencius is named after the Chinese philosopher; the analogy
is that each replica takes its turn ruling.

## System and threat model

Same as Multi-Paxos: asynchronous, crash-recovery, `f < n / 2`.

## Theory

### Slot-rotation rule

Slot `i` is *owned* by replica `i mod n`. The owner is the
proposer for that slot. Other replicas propose `NO-OP` if they
have nothing to propose for their slots and want to keep the
log progressing.

### Algorithm

```
each replica `r`:
  for slot i = r, r + n, r + 2n, ...:
    if r has a client request:
      propose Accept(b, i, op)
    else:
      propose Accept(b, i, NO-OP)  // skip slot

  for slot i not owned by r:
    listen for Accept; respond Accepted
```

If owner `r` is slow or crashed, other replicas can *skip*
its slots after a timeout, proposing `NO-OP` themselves via a
classic-Paxos-style Phase 1 takeover.

### Theorem (Mencius correctness)

Mencius satisfies SMR safety under crash-recovery and `f < n /
2`. The proof reduces to per-slot Synod safety (each slot is a
separate Synod instance with the owner as initial proposer).

### Throughput

In a balanced workload (each replica has roughly equal
proposal rate), Mencius's throughput is `n` times that of
single-leader Multi-Paxos: each replica runs a separate Paxos
flow.

### Latency

Each replica's commit latency is one round-trip (its own slot
plus a Paxos quorum). Slow replicas trigger the takeover
mechanism.

## Practice

### Where Mencius shows up

- *Geo-distributed databases.* Each region's replica handles
  its own slots, reducing inter-region latency.
- *Research systems.* Mencius is a benchmark for multi-leader
  designs; subsequent work (EPaxos, Atlas, Caesar) builds on
  similar ideas.

Mencius is rarely deployed in pure form: production systems
prefer either single-leader Multi-Paxos (operationally
simpler) or full multi-leader designs (EPaxos with conflict
detection).

### Skip-slot challenges

The skip mechanism is the protocol's most subtle part:

- Too eager skipping leads to throughput loss (real proposals
  are skipped).
- Too late skipping leads to stalls (waiting on a crashed
  owner).

Production tunes the timeout; typical values are `~10 * RTT`.

## Formalisation aspects

```text
structure MenciusSlot where
  owner       : NodeId
  state       : SynodState
  was_skipped : Bool

theorem mencius_safety
    (n f : Nat) (h : 2 * f + 1 = n) (sched : AsyncSched) :
    SmrSafety := by
  -- Per-slot Synod safety + skip handling.
  intro slot ...
  cases (was_skipped slot)
  case true => exact noop_satisfies_smr ...
  case false => exact synod_safety ...
```

## Verifiability and circuit encoding

**Tag: `friendly`.**

Mencius's per-slot verifiability is identical to Multi-Paxos:
~10^6 constraints per commit (BLS pairing). Skipped slots
contain `NO-OP` and have negligible per-slot cost.

The multi-leader structure means parallel proof generation:
each replica proves its own slots, with the L1 verifier
checking the per-slot certs.

## Known attacks and limitations

- *Owner crash.* Crashed owners hold up their slots until
  other replicas notice and run the takeover. Bounded by
  timeout.
- *Throughput imbalance.* If one replica receives most
  client requests, others' slots are filled with NO-OPs,
  losing the multi-leader benefit. Mitigation: client-side
  load balancing.

## Implementation notes

The crate provides a Mencius simulator with three replicas,
each with its own pending operations. Tests verify:

- Each replica commits its own slots.
- Cross-replica log consistency (all see the same sequence).

## References

- Mao, Junqueira, Marzullo, "Mencius: Building Efficient
  Replicated State Machines for WANs", OSDI 2008.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
