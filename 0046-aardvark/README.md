# 0046: Aardvark -- Robust BFT

## Historical context

Clement, Wong, Alvisi, Dahlin, Marchetti published "Making
Byzantine Fault Tolerant Systems Tolerate Byzantine Faults"
at NSDI 2009. Aardvark addresses a subtle problem: pre-2009
BFT protocols (PBFT, Zyzzyva) optimised for *fault-free*
performance but degraded sharply under Byzantine activity. A
malicious primary could slow the protocol indefinitely
without triggering view change.

Aardvark's contribution: design BFT for *robust* performance
under attack rather than peak performance under benign
conditions.

## System and threat model

Same as PBFT: partial synchrony, `f < n / 3`.

## Theory

### Robustness mechanisms

- *Aggressive view changes.* Performance-monitoring triggers
  a view change if the primary's throughput drops below a
  threshold.
- *Mandatory primary rotation.* Even without faults, primary
  rotates periodically.
- *Resource isolation.* Per-client request rate limits
  prevent denial-of-service.
- *Signed authentication.* Replaces PBFT's MAC vectors with
  signatures (slower but accountable).

### Theorem (Aardvark robustness)

Aardvark guarantees *bounded performance degradation* under
any Byzantine schedule: the throughput is at least `(1 -
f/n) * peak_throughput`.

### Performance trade-off

- *Fault-free.* ~30% slower than Zyzzyva due to signature
  costs.
- *Under attack.* `> 100x` faster than Zyzzyva.

## Practice

### Influence

- HotStuff's responsiveness analysis explicitly considers
  the Aardvark robustness criterion.
- Production systems (Aptos, Sui) include performance-
  monitoring and primary rotation.

## Verifiability

**Tag: `friendly`.** Signature-based; per-commit ~10^6
constraints.

## References

- Clement, Wong, Alvisi, Dahlin, Marchetti, "Making Byzantine
  Fault Tolerant Systems Tolerate Byzantine Faults", NSDI 2009.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
