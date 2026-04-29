# 0081: BBCA-chain

## Historical context

BBCA-chain (2024, "Byzantine Broadcast Channel Aggregator -
chain") is a DAG-BFT variant introducing *channel-based*
DAG aggregation: each broadcast lane has its own
sequencing constraints, allowing pipelined commits per
lane independent of others. Inspired by sharded Narwhal
designs.

## Theory

Per-lane DAG; cross-lane synchronisation only at anchor
commits. Reduces inter-lane interference: a slow
producer in one lane does not stall others.

### How BBCA-chain differs

| property              | Bullshark | BBCA-chain          |
| --------------------- | --------- | ------------------- |
| DAG structure         | global    | per-lane + sync points |
| latency under partial slowdown | degrades | per-lane preserved |
| typical commit latency | ~2 RTT  | ~2 RTT              |

## Practice

Reference design 2024.

## Verifiability

**tag: `friendly`.** Standard.

## References

- "BBCA-chain", 2024.

See also [`HISTORY.md`](../HISTORY.md), section "2024 to
2026".
