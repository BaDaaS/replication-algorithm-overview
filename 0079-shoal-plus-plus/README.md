# 0079: Shoal++

## Historical context

Shoal++ (Aptos 2024) is a further refinement of Shoal
(module 0075) that pushes anchor pipelining: anchors are
proposed and committed every round, with multiple in-flight
simultaneously. Latency drops from Shoal's ~1 RTT to
~half RTT in the steady state.

## System and threat model

Same as Bullshark and Shoal.

## Theory

Shoal++ proposes anchors per-round per-leader, with a
ranked commit rule that resolves conflicts when multiple
anchors converge. Pipelining depth is bounded by the
network's maximum in-flight blocks.

### How Shoal++ differs from Shoal

| property              | Shoal      | Shoal++       |
| --------------------- | ---------- | ------------- |
| anchors per round     | 1          | up to wave    |
| pipelining depth      | wave       | dynamic       |
| typical latency       | ~1 RTT     | ~half RTT    |
| robustness            | high       | high          |
| production            | Aptos 2023 | Aptos 2024+   |

## Practice

Aptos production as of 2024.

## Verifiability

**tag: `friendly`.** ~10^6 per anchor.

## References

- Aptos engineering blog, "Shoal++", 2024.

See also [`HISTORY.md`](../HISTORY.md), section "2024 to
2026".
