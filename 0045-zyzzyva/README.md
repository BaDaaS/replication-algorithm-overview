# 0045: Zyzzyva -- Speculative BFT

## Historical context

Kotla, Alvisi, Dahlin published "Zyzzyva: Speculative
Byzantine Fault Tolerance" at SOSP 2007 (TOCS journal 2009).
Zyzzyva is a *speculative* BFT: replicas execute requests
optimistically without waiting for full agreement, then roll
back if a violation is detected.

Throughput: 80k ops/sec on commodity hardware of 2007,
significantly faster than PBFT's 30k. The speculation comes
at the cost of more complex client-side logic.

## System and threat model

- **Network.** Partially synchronous.
- **Failures.** Byzantine, `f < n / 3`.
- **Goal.** Linearisable BFT with speculative-execution
  fast path.

## Theory

### Algorithm sketch

```
client -> primary: request
primary: assigns sequence n; broadcasts to replicas
replicas: execute speculatively; reply to client with state
          digest

client:
  collects 3f + 1 matching replies: commit (fast path)
  collects 2f + 1 matching replies: trigger commit phase
  collects fewer: PBFT-style fallback
```

The fast path requires 3f + 1 replies (everyone). With one
faulty replica, falls to 2f + 1 path.

### Theorem (Zyzzyva correctness)

Under partial synchrony with `f < n / 3`, Zyzzyva satisfies
linearisability. Speculative execution rolls back on
violations; clients verify response consistency.

### Performance

- *Fast path.* `~80k ops/sec` (2007 hardware).
- *Slow path.* PBFT-equivalent.

## Practice

### Influence

Zyzzyva's speculative-execution idea inspired the broader
"fast-path" line: HotStuff's optimistic responsiveness,
Aptos's pipelined execution, Sui's parallel execution.

### Zyzzyva5

Kotla et al. 2008 extended Zyzzyva to require `n = 5f + 1`
for the fast path, simplifying the protocol but increasing
replica count.

## Verifiability

**Tag: `partial`.** Speculation is not naturally verifiable;
the rollback path complicates SNARK proofs. Per-commit cost
is similar to PBFT in the slow path.

## References

- Kotla, Alvisi, Dahlin, "Zyzzyva: Speculative Byzantine
  Fault Tolerance", SOSP 2007.
- Kotla et al., "Zyzzyva", TOCS 2009.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
