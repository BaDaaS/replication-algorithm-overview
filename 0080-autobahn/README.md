# 0080: Autobahn

## Historical context

Giridharan, Hellerstein, Kokoris-Kogias, Sonnino,
Spiegelman, Stoica, Whittaker published "Autobahn: Seamless
High Speed BFT" at SOSP 2024. Autobahn merges the Narwhal
mempool with a fast linear consensus, producing a
streamlined alternative to DAG-BFT designs that achieves
similar throughput with lower complexity.

## System and threat model

Partial synchrony, `f < n / 3`.

## Theory

### Linear consensus over Narwhal

Instead of DAG-Rider/Bullshark's wave-based commit,
Autobahn runs a linear single-leader consensus over the
Narwhal certificate stream. The leader proposes the next
*lane* of certificates; replicas vote.

### How Autobahn differs from Bullshark

| property              | Bullshark         | Autobahn          |
| --------------------- | ----------------- | ----------------- |
| consensus topology    | wave-anchor (DAG) | linear (HotStuff-like) |
| typical latency       | ~2 RTT            | ~2 RTT            |
| code complexity       | high (DAG state)  | medium (linear)   |
| throughput at n=50    | ~300k tx/s        | ~250k tx/s        |
| async fallback        | Tusk              | optional          |

The simplification: Autobahn keeps Narwhal's mempool but
trades the DAG consensus for a HotStuff-like linear one,
preserving most throughput while halving consensus-layer
code complexity.

## Practice

Reference design 2024.

## Verifiability

**tag: `friendly`.** Per-block ~10^6.

## References

- Giridharan et al., "Autobahn: Seamless High Speed BFT",
  SOSP 2024.

See also [`HISTORY.md`](../HISTORY.md), section "2024 to
2026".
