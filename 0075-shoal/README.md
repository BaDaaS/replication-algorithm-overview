# 0075: Shoal

## Historical context

Spiegelman, Gelashvili, Aleo, Rajath, Sonnino, Kokoris-
Kogias published "Shoal: Improving DAG-BFT Latency And
Robustness" 2023. Shoal is Aptos's pipelined upgrade to
Bullshark: instead of waiting for one anchor commit per
wave, Shoal pipelines multiple anchors simultaneously,
halving commit latency.

## System and threat model

Same as Bullshark: partial-sync + async fallback,
`f < n / 3`.

## Theory

### Pipelined anchors

Shoal proposes anchors at every round, not every wave.
Each anchor goes through Bullshark's two-round commit. With
multiple anchors in flight, throughput stays high while
latency drops.

### How Shoal differs from Bullshark

| property              | Bullshark | Shoal              |
| --------------------- | --------- | ------------------ |
| anchors per wave      | 1         | up to wave-length  |
| typical commit latency | ~2 RTT  | ~1 RTT             |
| pipelining            | no        | yes                |
| async fallback        | yes       | yes                |
| production            | Sui 22-24 | Aptos              |

The pipelining is the key difference; Shoal's commit cadence
is essentially "every round, an anchor".

## Practice

- *Aptos.* Production Shoal as of 2023.
- *Sui.* Considered Shoal but moved directly to Mysticeti
  (module 0076).

## Verifiability

**tag: `friendly`.** Per anchor ~10^6 (BLS QC).

## References

- Spiegelman et al., "Shoal: Improving DAG-BFT Latency And
  Robustness", 2023.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to
2023".
