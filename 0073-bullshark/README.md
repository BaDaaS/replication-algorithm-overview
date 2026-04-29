# 0073: Bullshark

## Historical context

Spiegelman, Giridharan, Sonnino, Kokoris-Kogias published
"Bullshark: DAG BFT Protocols Made Practical" at CCS 2022.
Bullshark replaces Tusk's asynchronous wave consensus with a
*partial-synchrony* commit rule that is faster when the
network is healthy, while keeping Tusk's async fallback as
a safety net.

The improvement: under partial synchrony, Bullshark commits
in 2 rounds (Narwhal layers) per anchor, vs Tusk's 6+
rounds. Production: Sui mainnet ran Bullshark over Narwhal
from 2022 to 2024.

## System and threat model

- **Mempool.** Narwhal (module 0072).
- **Consensus.** Bullshark.
- **Network.** Partial synchrony (fast path) + async
  (slow path).
- **Failures.** `f < n / 3`.

## Theory

### Anchor commit (fast path)

```
each round, the leader proposes an anchor vertex
if 2f + 1 vertices in the next two rounds reference the
anchor: commit
```

Two-round commit, like Jolteon's two-chain in HotStuff
family, but over the DAG.

### Async fallback

If the partial-synchrony fast path stalls, Bullshark falls
back to Tusk's async wave-anchor commit (slower but live).

### How Bullshark differs from Tusk

| property              | Tusk         | Bullshark          |
| --------------------- | ------------ | ------------------ |
| timing model          | async        | partial sync + async fallback |
| anchor commit rounds  | 6+           | 2 (fast) / 6+ (fallback) |
| latency (good network)| ~3-4 RTT     | ~2 RTT             |
| latency (under attack)| same         | falls back to Tusk |
| mempool layer         | Narwhal      | Narwhal            |
| production deployment | none         | Sui mainnet (2022-24) |

The Bullshark advantage: under realistic network conditions,
the fast path dominates and gives latency comparable to
HotStuff while preserving DAG-BFT throughput.

### How Bullshark differs from HotStuff

| property              | HotStuff     | Bullshark          |
| --------------------- | ------------ | ------------------ |
| mempool/consensus     | bundled      | separated (Narwhal)|
| commit                | per-block    | per-anchor (DAG)   |
| throughput at n=50    | ~50k tx/s    | ~300k tx/s         |
| latency               | ~few RTT     | ~2 RTT (DAG layers)|
| linearity             | yes          | yes                |
| async fallback        | no           | yes (Tusk)         |

Bullshark inherits Narwhal-Tusk's mempool advantage and
adds HotStuff-grade fast-path latency.

## Practice

- *Sui mainnet (2022-24).* Bullshark over Narwhal in
  production.
- *Mysticeti (2024).* Replaced Bullshark for sub-second
  latency (module 0076).
- *Reference implementation.* Mysten Labs open source.

## Verifiability

**tag: `friendly`.** Per anchor commit ~10^6 (BLS QC over
Narwhal certificates). Same constraint cost as HotStuff.

## References

- Spiegelman, Giridharan, Sonnino, Kokoris-Kogias,
  "Bullshark: DAG BFT Protocols Made Practical", CCS 2022.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to
2023".
