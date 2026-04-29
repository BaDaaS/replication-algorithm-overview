# 0035: Chain Replication

## Historical context

Van Renesse and Schneider published "Chain Replication for
Supporting High Throughput and Availability" at OSDI 2004.
The protocol arranges replicas in a *linear chain*: the head
accepts writes; updates flow down the chain; the tail responds
to clients. Reads are served by the tail.

Chain Replication's selling point is *strong consistency at
storage-system throughput*: it achieves linearisability
without per-write quorum costs.

## System and threat model

- **Network.** Reliable FIFO point-to-point; assumed (not
  proven) to be partially synchronous for failure detection.
- **Failures.** Crash-stop. A separate *configuration
  master* manages the chain.
- **Goal.** Linearisable storage with high throughput.

## Theory

### Topology

Replicas are arranged as `head -> R_1 -> R_2 -> ... -> tail`.

```
Client write -> head
head -> R_1 -> ... -> tail
tail -> Client (ack)

Client read -> tail
tail -> Client (value)
```

### Algorithm

**Write path:**

```
client -> head: write(k, v)
head: append (k, v) to local store; forward to next
each replica: same
tail: store; respond to client with ack
```

The write is acknowledged *only after the tail has stored
it*, ensuring all replicas in the chain have seen the value.

**Read path:**

```
client -> tail: read(k)
tail: respond with current value
```

### Theorem (Chain Replication safety)

Chain Replication is linearisable for both reads and writes.

*Proof.* Writes are totally ordered by the head's local order.
Each write propagates down the chain in FIFO order. Reads at
the tail see the latest write (because the tail's
acknowledgement is the commit point).

### Failure handling

Each crash type has a recovery procedure:

- *Head crash.* The next-in-chain becomes the new head.
- *Tail crash.* The previous-in-chain becomes the new tail.
- *Middle replica crash.* Bypass it; head/tail unchanged.

The configuration master (a Paxos-replicated state machine)
orchestrates the recovery.

### Performance

- *Throughput.* Linear in chain length: every replica writes
  in parallel.
- *Latency.* Linear in chain length: writes propagate through
  all replicas.

The trade-off vs Multi-Paxos: throughput per replica is
higher (no quorum overhead), but write latency scales with
chain depth.

## Practice

### Where Chain Replication is used

- *Microsoft FDS (Flat Datacenter Storage).* Chain replication
  for blob storage.
- *Hibari.* Distributed key-value store using chain
  replication.
- *Object-storage systems.* Many adopt chain-replication
  patterns for write-heavy storage.

### CRAQ extension

CRAQ (Chain Replication with Apportioned Queries; Terrace-
Freedman 2009, module 0036) extends Chain Replication so
that *intermediate* replicas can serve reads under certain
conditions. This addresses the tail's read-bottleneck
problem.

## Formalisation aspects

```text
structure ChainReplicationState where
  chain        : List NodeId  -- head first, tail last
  store        : Map Key Value
  pending      : Queue Update

theorem chain_replication_linearisable
    (sched : AsyncSchedule) (faults : Set NodeId) :
    Linearisable ChainReplication sched faults := by
  -- FIFO + tail-ack point gives total order.
  sorry
```

## Verifiability and circuit encoding

**Tag: `friendly`.**

Verifiable Chain Replication: each replica signs its updates
in chain order. Per-write proof:

- Head signature on the original update: ~3k constraints.
- Each chain link's signature: ~3k each, ~3k * (n - 1) total.
- Tail's ack signature: ~3k.

For chain length 5: ~15k constraints. Smaller than BLS-
aggregate Paxos (~10^6), because there is no quorum overhead.

The trade-off: chain replication's serialisability is
explicit (chain order), so the proof structure is simpler
than quorum-based protocols.

## Known attacks and limitations

- *Tail bottleneck for reads.* All reads go to the tail.
  CRAQ addresses this.
- *Head bottleneck for writes.* All writes start at the head.
  Sharded chain replication (one chain per partition)
  addresses this.
- *Latency vs throughput.* Long chains have high write
  latency.

## Implementation notes

The crate provides a 3-replica chain (head, middle, tail).
Tests verify a write propagates to all three; a read at the
tail returns the latest value.

## References

- van Renesse and Schneider, "Chain Replication for Supporting
  High Throughput and Availability", OSDI 2004.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
