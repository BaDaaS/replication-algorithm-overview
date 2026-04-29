# 0049: Prime

## Historical context

Amir, Coan, Kirsch, Lane published "Prime: Byzantine
Replication under Attack" at TDSC 2010 (preprint 2008).
Prime adds a *pre-ordering* phase: each replica commits to a
local ordering of pending requests before the primary's
pre-prepare. Replicas later verify that the primary respects
this pre-ordering, bounding the primary's ability to slow or
manipulate throughput under attack.

Prime targets WAN deployments where Aardvark's view-change
overhead is prohibitive.

## System and threat model

PBFT-style: partial synchrony, `f < n / 3`.

## Theory

```
each replica r:
  periodically broadcasts a "pre-order" of its local pending
  set
  monitors primary's pre-prepares: if not consistent with the
  pre-order, raises evidence
```

The primary cannot delay or reorder requests beyond the
pre-order without producing detectable evidence. This bounds
worst-case throughput tightly without aggressive view changes.

## Practice

Influence: pre-ordering ideas appear in Mempool/consensus
separation (Narwhal-Tusk module 0072), where the mempool's
pre-ordering bounds the consensus layer's manipulation.

## Verifiability

**Tag: `friendly`.** ~10^6 per commit.

## References

- Amir, Coan, Kirsch, Lane, "Prime: Byzantine Replication
  under Attack", TDSC 2010.

See also [`HISTORY.md`](../HISTORY.md), section "2009 to 2014".
