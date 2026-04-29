# Module 0008 Solutions

## Solution 1 [T]: Gilbert-Lynch in full

Two processes `p_1, p_2`. The system runs a register `R` that
supports `write(v)` and `read()`. We assume:

- Linearisable: every read returns a value `v` such that some
  serialisation of the operations consistent with real-time
  order has `read = v` immediately after the latest preceding
  `write(v)`.
- Available: every operation eventually responds.
- Partition tolerance: the system is correct (linearisable and
  available) under arbitrary partitions.

Construct a schedule:

1. Initial register value `0`.
2. The wire between `p_1, p_2` is partitioned at time `t_0`.
3. `p_1` issues `write(1)` at time `t_1 > t_0`. By
   availability, `p_1`'s write completes at some time `t_1' >
   t_1`.
4. `p_2` issues `read()` at time `t_2 > t_1'`. By availability,
   `p_2`'s read completes at some time `t_2' > t_2`.

`p_2`'s read at time `t_2'` precedes any communication from
`p_1` (the partition is in place). `p_2`'s response `v` is
either `0` or `1`:

- If `v = 0`, then the schedule `write(1) -> read = 0` is not
  linearisable (the `write` precedes the `read` in real time
  but the `read` does not see `1` or any later write).
- If `v = 1`, then `p_2` somehow knew about `p_1`'s write
  despite the partition; impossible by definition of a
  partition.

Either way, one of CAP fails. QED.

## Solution 2 [T]: relaxations

- *Linearisable -> sequential consistency.* Drops the
  real-time order requirement; permits a global serial order
  consistent only with each process's own program order.
  Production: many key-value stores (Riak, Memcached) provide
  sequential consistency only.
- *Sequential consistency -> eventual consistency.* Drops the
  global serial order; only requires that, in the absence of
  further updates, all replicas eventually converge.
  Production: DynamoDB, Cassandra, Riak (default), DNS.
- *Available -> bounded staleness.* Permits the read to be at
  most `delta` old. Production: AWS Aurora reader replicas
  (typically <1 second stale).

## Solution 3 [P]: bounded staleness

```rust
pub struct StalenessAwareReader {
    pub local: u64,
    pub last_local_update: Time,
    pub bound: Time,
    pub last_read: Option<u64>,
}

impl StalenessAwareReader {
    fn try_read(&mut self, now: Time) {
        if now - self.last_local_update <= self.bound {
            self.last_read = Some(self.local);
        }
        // else: refuse to respond
    }
}
```

The system gives "read your own writes" within the bound but
permits arbitrary staleness beyond it. Production: AWS DynamoDB
with `ConsistentRead = false`; Spanner with bounded-staleness
read transactions.

## Solution 4 [P]: Spanner's TrueTime

TrueTime exposes a clock with bounded uncertainty: `TT.now()`
returns an interval `[earliest, latest]` with
`latest - earliest <= epsilon` (typically `<7ms`). Spanner
uses TrueTime in two ways:

1. *Commit-wait.* Before responding to a write, Spanner waits
   `epsilon` to ensure the commit timestamp is in the past for
   every observer. This guarantees external consistency
   (linearisability) without needing cross-region acks for
   every operation.
2. *Clock-skew bounded leases.* Leader leases are bounded by
   the maximum clock skew, so a fail-stop leader's lease
   expires globally consistently.

The result is that Spanner can be PC: under partition, the
minority partition cannot extend its lease, so it stops
serving reads; the majority partition continues. The
"unfairness" of failover is bounded by the lease duration.

## Solution 5 [F]: linearisability in pseudo-Lean

```text
inductive Op where
  | write (v : Nat)
  | read

structure HistoryEvent where
  op       : Op
  process  : NodeId
  start    : Time
  finish   : Time
  result   : Option Nat  -- only for read

def IsLinearisable (history : List HistoryEvent) : Prop :=
  exists (serial : List HistoryEvent),
    serial.Permutations history /\
    (forall (i j : Fin serial.length),
      i < j /\ serial[i].finish < serial[j].start ->
      i < j /\ serial[i].finish < serial[j].start) /\
    (forall (i : Fin serial.length),
      serial[i].op = Op.read ->
      serial[i].result = LatestWritten serial i)
```

The first quantifier asserts that `serial` is a permutation; the
second asserts real-time order is preserved; the third asserts
each read returns the latest preceding write. Mathlib has
`List.Permutations`, `Order.Embedding`, and the standard
infrastructure.

## Solution 6 [V]: verifiable AP-or-CP

A verifiable replication algorithm that publishes safety
proofs is structurally CP-leaning at the *commitment* level:
under partition, the partition's smaller side cannot produce
a fresh proof, so light clients on that side see no progress
and may reject as stale. The system is *not unavailable in the
service sense*: the smaller side may continue to serve stale
proofs (the latest one before the partition), which is bounded
staleness.

The relationship between the SNARK and availability:

- The SNARK certifies safety: the prefix is consistent.
- The SNARK does *not* certify liveness or availability: a
  light client cannot tell from the proof alone whether the
  protocol is making progress or has halted.
- Light clients use *out-of-band* signals (latest slot
  timestamp, recent gossip) to detect halts.

In Mina's case, the network gossips proofs continuously, and
a 30-second-old proof is treated as a halt indicator. zk-rollup
sequencers similarly time out batches; Aztec's documentation
specifies a 1-hour batch-timeout that triggers fallback
mechanisms (force-included transactions on L1).

So a verifiable replication algorithm is CP at the safety
layer and *bounded-staleness available* at the service layer:
the staleness bound is a separately tuned operational
parameter.
