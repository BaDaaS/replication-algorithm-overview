# 0053: BFT2F

## Historical context

Li and Mazieres published "Beyond One-third Faulty Replicas
in Byzantine Fault Tolerant Systems" at NSDI 2007. BFT2F
addresses the question: what BFT guarantees can a system
still provide when more than `f` of `3f + 1` replicas are
faulty (i.e., when the standard PBFT threshold is exceeded)?

Answer: with `f < f' < 2f + 1` faulty replicas, BFT2F
provides *fork* consistency (clients detect inconsistency
across forks) rather than full linearisability. With `f' >=
2f + 1`, no consistency is achievable.

## System and threat model

PBFT-style with up to `f' < 2f + 1` Byzantine.

## Theory

The key insight: even with too many Byzantine replicas, a
fault-detection mechanism can give clients evidence of
inconsistency. Clients exchange "tickets" attesting their
observations; mismatches indicate forks.

## Practice

Influence: blockchain light-client design (clients detect
forks via header mismatch). Modern variants in zk-bridges.

## Verifiability

**Tag: `friendly`.** Standard cost.

## References

- Li, Mazieres, "Beyond One-third Faulty Replicas in
  Byzantine Fault Tolerant Systems", NSDI 2007.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
