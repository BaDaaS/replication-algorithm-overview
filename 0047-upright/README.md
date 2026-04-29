# 0047: UpRight

## Historical context

Clement, Kapritsos, Lee, Wang, Alvisi, Dahlin, Riche
published "UpRight Cluster Services" at SOSP 2009. UpRight
is a BFT replication library separating *correctness* faults
from *liveness* faults. The model: tolerate `u` Byzantine
("upright"-violating) faults plus `r` crash faults. With
`n = 2u + r + 1` replicas, UpRight achieves the same
fault tolerance as PBFT plus crash-only fault budget.

The model accommodates a realistic deployment: rare
Byzantine failures (`u` small) plus more frequent crashes
(`r` larger). UpRight's threshold rule lets you spend more
replicas on crash tolerance without paying for unnecessary
Byzantine resilience.

## System and threat model

- **Network.** Partial synchrony.
- **Failures.** `u` Byzantine + `r` crash, `n = 2u + r + 1`.
- **Goal.** Linearisable BFT under hybrid failure model.

## Theory

Quorum: `u + r + 1` (intersect with `u + r + 1` to give `1`
non-faulty witness). For `u = 1, r = 2`, `n = 5` and quorum
= 4.

UpRight's safety reduces to standard BFT-quorum intersection
arguments adapted for the hybrid model.

## Practice

UpRight's library inspired Hyperledger Fabric's pluggable
ordering service (BFT vs Raft modes) and Cockroach's hybrid
fault models.

## Verifiability

**Tag: `friendly`.** Standard BFT cost.

## References

- Clement et al., "UpRight Cluster Services", SOSP 2009.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
