# 0038: CASPaxos -- Compare-and-Swap Paxos

## Historical context

Rystsov's 2018 paper "CASPaxos: Replicated State Machines
without logs" introduced a Paxos variant tailored for
*compare-and-swap* (CAS) registers. Each key is a separate
mini-state-machine; CASPaxos avoids the unbounded-log
problem of classic Multi-Paxos by storing only the *current
value* per key.

The protocol is a Paxos Synod per CAS operation: the
proposer reads the current value, computes the new value via
a client-supplied function `f(old) -> new`, and proposes it.
Suitable for distributed key-value stores with linearisable
CAS semantics.

## System and threat model

Same as Synod Paxos.

## Theory

### Per-key state

Each acceptor stores per key:

```
promised_ballot : Nat
accepted_value  : (Ballot, Value)
```

No log: just the latest accepted value.

### CAS protocol

```
client -> proposer: cas(k, f) where f : Value -> Value
proposer:
  phase 1 (prepare):
    pick fresh ballot b
    send Prepare(k, b) to acceptors
    collect majority Promises with current value
  phase 2 (compute and accept):
    new_value := f(latest accepted value)
    send Accept(k, b, new_value)
    collect majority Accepteds
  return new_value to client
```

The function `f` is supplied by the client. Common choices:

- `f(_) = v_new` (write).
- `f(v) = v + 1` (increment).
- `f(v) = if v == expected then new else v` (compare-and-set).

### Theorem (CASPaxos linearisability)

CASPaxos satisfies linearisable CAS semantics under crash-
recovery and `f < n / 2`.

*Proof.* Per-key Synod safety + the f-application step. The
proposer's Phase-2 value depends only on the latest accepted
value seen in Phase 1; quorum intersection ensures
linearisability.

### Performance

- *Per-CAS latency.* 2 round-trips (Phase 1 + Phase 2). With
  leader pinning per key, can amortise to 1 RT.
- *Storage cost.* Constant per key (no log).

The trade-off vs Multi-Paxos: no SMR replay, but each CAS
runs a full Paxos instance.

## Practice

### Where CASPaxos shows up

- *Etcd's compare-and-swap* uses Raft; CASPaxos provides the
  same semantics at lower per-operation cost (no log).
- *Consul KV* offers CAS via a similar pattern.
- *Cassandra Lightweight Transactions* are Paxos-based
  per-key CAS.

### Use cases

- *Distributed locks.* Acquire-by-CAS pattern.
- *Leader leases.* Renew via CAS.
- *Configuration storage.* Versioned config updates.
- *Atomic counters.* Increment via `f(v) = v + 1`.

## Formalisation aspects

```text
structure CASPaxosState where
  promised : Nat
  accepted : (Nat × Value)

theorem caspaxos_linearisable
    (sched : AsyncSched) (faults : CrashRecovery f) :
    LinearisableCAS := by
  -- Per-key Synod safety + f-application.
  sorry
```

## Verifiability and circuit encoding

**Tag: `friendly`.**

Per-CAS proof:

- Phase-1 BLS quorum cert: ~10^6.
- Phase-2 BLS quorum cert: ~10^6.
- f-application witness: small.

Total per CAS: ~2 * 10^6 constraints. With leader pinning
amortising Phase 1, ~10^6 per typical CAS.

The CAS structure is naturally per-key: a verifiable
distributed KV store with linearisable CAS is a clean
SNARK target. Aleo's snarkVM has compatible primitives.

## Known attacks and limitations

- *Per-CAS Phase 1 cost.* Without leader pinning, every CAS
  runs Phase 1. Leader-pinning recovers Multi-Paxos
  amortisation per key.
- *Function f restrictions.* The function must be
  deterministic and side-effect-free.

## Implementation notes

The crate provides a per-key CASPaxos simulator with:

- Three acceptors.
- One proposer.
- A test where the client increments a counter via CAS.

## References

- Rystsov, "CASPaxos: Replicated State Machines without
  logs", arXiv 2018.

See also [`HISTORY.md`](../HISTORY.md), section "2015 to 2019".
