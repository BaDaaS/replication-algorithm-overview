# 0044: HQ Replication

## Historical context

Cowling, Myers, Liskov, Rodrigues, Shrira published "HQ
Replication: A Hybrid Quorum Protocol for Byzantine Fault
Tolerance" at OSDI 2006. HQ is a hybrid: contention-free
operations use a Q/U-style fast quorum (one round-trip);
contended operations fall back to a PBFT-style three-phase
commit.

The hybrid design tries to capture the best of both worlds.
Replica count: `n = 3f + 1` (PBFT-equivalent), unlike Q/U's
`5f + 1`.

## System and threat model

- **Network.** Asynchronous (with eventual delivery).
- **Failures.** Byzantine, `f < n / 3` (`n = 3f + 1`).
- **Goal.** Linearisable BFT with a fast path for non-
  contended operations.

## Theory

### Algorithm sketch

```
client write:
  send (op) to all replicas
  collect 2f + 1 receipts (write certificates)

  if all certificates carry the same prior state:
    fast-path commit (1 RT)
  else:
    fall back to PBFT three-phase exchange
```

The fast path requires 2f + 1 *agreeing* receipts; any
divergence triggers the slow path.

### Theorem (HQ correctness)

Under `n = 3f + 1`, HQ satisfies linearisability for both
fast and slow paths.

### Performance

- *Best case (no contention).* 1 round-trip.
- *Worst case (contention).* PBFT-equivalent three-phase
  exchange.

Cowling et al. report 30-50% throughput improvement over
PBFT for read-heavy or low-contention workloads.

## Practice

HQ is rarely deployed. Its lessons influenced Zyzzyva
(module 0045) and the broader "fast-path BFT" line.

## Verifiability

**Tag: `friendly`.** Per-commit ~10^6 constraints (BLS
aggregate). Hybrid path adds a small flag.

## References

- Cowling, Myers, Liskov, Rodrigues, Shrira, "HQ
  Replication: A Hybrid Quorum Protocol for Byzantine Fault
  Tolerance", OSDI 2006.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
