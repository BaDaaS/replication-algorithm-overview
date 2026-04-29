# 0078: Sailfish

## Historical context

Sailfish (Diem-Naor-Spiegelman 2024) is a DAG-BFT optimised
for low-latency commits in adversarial conditions, achieved
via a novel *direct commit* rule: under partial synchrony,
each anchor commits in just one DAG round (vs Mysticeti's
two for steady path).

Trade-off: the direct commit requires stricter synchrony
than Mysticeti's steady path; falls back gracefully under
adversarial scheduling.

## System and threat model

Partial synchrony, `f < n / 3`.

## Theory

### Direct anchor commit

A leader vertex `L` is committed in the same round as it is
proposed, provided `2f + 1` of the next round's vertices
extend `L`. With one round of latency, much faster than
Bullshark's two-round commit.

### How Sailfish differs

| property              | Bullshark | Mysticeti  | Sailfish |
| --------------------- | --------- | ---------- | -------- |
| commit rounds         | 2         | 1 (fast)/2 | 1        |
| latency under attack  | 2 RTT     | degrades   | 2 RTT    |
| robustness            | high      | high       | high     |

The direct commit is a refinement of Mysticeti's fast path
made into the steady-state behaviour.

## Practice

Reference design as of 2024-2025.

## Verifiability

**tag: `friendly`.** ~10^6 per anchor.

## References

- "Sailfish: Towards Improving Latency of DAG-Based BFT",
  2024.

See also [`HISTORY.md`](../HISTORY.md), section "2024 to
2026".
