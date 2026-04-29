# 0082: Motorway

## Historical context

Motorway (2025) is a research-stage DAG-BFT successor to
Autobahn that targets ultra-high throughput on heterogeneous
networks. It splits the consensus path into a fast lane
(homogeneous, low-latency replicas) and a slow lane
(heterogeneous validators), with a unified commit rule.

## Theory

Two-lane parallel consensus with cross-lane synchronisation:
fast lane commits in ~1 RTT under low-latency conditions;
slow lane absorbs heterogeneity without stalling the fast
lane.

## Practice

Research reference, 2025.

## Verifiability

**tag: `friendly`.** Standard.

## References

- Motorway 2025 preprint.

See also [`HISTORY.md`](../HISTORY.md), section "2024 to
2026".
