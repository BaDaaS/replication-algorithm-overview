# 0036: CRAQ -- Chain Replication with Apportioned Queries

## Historical context

Terrace and Freedman's 2009 USENIX ATC paper "Object Storage
on CRAQ: High-Throughput Chain Replication for Read-Mostly
Workloads" extended Chain Replication (module 0035) to allow
*any* replica to serve reads, not just the tail. This
removes the tail's read bottleneck and gives proportional
read throughput as the chain grows.

The trick: replicas store *clean* (all-replicas-agree) and
*dirty* (still-propagating) versions. A read at any replica
returns the clean version if available; otherwise the
replica queries the tail.

## System and threat model

Same as Chain Replication.

## Theory

### Per-key state

Each replica stores per key:

```
clean:  the latest version known committed (all-replicas-agree)
dirty:  list of versions in flight, with version numbers
```

### Read protocol

```
read(k) at replica R:
  if R has only clean(k):
    return clean(k)
  else:
    query tail for the latest version
    return that version
```

If a replica has uncommitted updates for `k`, it cannot serve
the read locally; it queries the tail. If the tail has the
same dirty version, the version is returned. If the tail has
moved past it, the new version is returned.

### Write protocol

Same as Chain Replication: head -> chain -> tail. As the
write propagates, each replica adds a new dirty version. When
the tail acks, the dirty version is promoted to clean and the
ack flows back up the chain (replicas garbage-collect old
dirty versions).

### Theorem (CRAQ linearisability)

CRAQ is linearisable for both reads and writes.

*Proof.* For writes: same as Chain Replication. For reads:
either return clean (provably committed) or the tail's
version (also committed). No dirty versions are returned
without tail confirmation.

### Performance

- *Read throughput.* Proportional to chain length: each
  replica can serve reads for keys it has clean.
- *Read latency.* Local reads are fast (no inter-replica
  message); reads that fall through to the tail pay the
  chain RTT.

For read-heavy workloads with rare writes, CRAQ achieves
near-perfect read scaling.

## Practice

### Where CRAQ shows up

- *Object-storage systems.* Microsoft FDS extended with CRAQ
  read paths.
- *KV stores.* Hibari, Cassandra (with TTL caching) take
  related approaches.
- *Modern distributed databases.* "Read replicas with
  follower reads" patterns approximate CRAQ semantics.

### Workload sensitivity

CRAQ shines for read-heavy workloads. For write-heavy
workloads, the per-key dirty state is the bottleneck.
Production tunes:

- *Garbage collection of dirty versions.*
- *Bloom filters* to test if a replica has clean for a key.
- *Read-routing policies.* Route to the most-likely-clean
  replica.

## Formalisation aspects

```text
structure CraqState extends ChainReplicationState where
  clean  : Map Key Value
  dirty  : Map Key (List (Version × Value))

theorem craq_linearisable
    (sched : AsyncSchedule) :
    Linearisable CRAQ sched := by
  -- Reads either return clean (committed) or tail-confirmed.
  sorry
```

## Verifiability and circuit encoding

**Tag: `friendly`.**

CRAQ's verifiability is similar to Chain Replication: per-
write signature chain. Per-read: a signed clean-state
attestation, ~3k constraints.

The clean/dirty distinction is local to each replica; the
verifier doesn't see it directly. The proof attests that the
read returned a *committed* version, regardless of whether
it was served locally or from the tail.

## Known attacks and limitations

- *Stale clean state.* If a replica has clean(v_1) but the
  global state has moved to v_2 (committed, propagating), the
  replica's read returns v_1 unless the replica detects the
  staleness. The protocol handles this by querying the tail
  on dirty-state ambiguity.
- *Per-key state.* Per-key dirty/clean tracking adds
  per-replica memory.

## Implementation notes

The crate provides a CRAQ extension to the chain replicator:
each replica tracks per-key clean and dirty versions.
Tests verify reads at any replica return committed values.

## References

- Terrace and Freedman, "Object Storage on CRAQ: High-
  Throughput Chain Replication for Read-Mostly Workloads",
  USENIX ATC 2009.

See also [`HISTORY.md`](../HISTORY.md), section "2009 to 2014".
