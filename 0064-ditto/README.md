# 0064: Ditto -- Asynchronous Fallback

## Historical context

Ditto is the asynchronous-fallback companion to Jolteon
(module 0058), introduced in the same Gelashvili et al.
2021 paper "Jolteon and Ditto". Ditto answers: what
happens when the partial-synchrony assumption breaks?
Jolteon-Ditto switches between Jolteon (fast under
synchrony) and an asynchronous BFT (slow but live without
synchrony) based on observed network behaviour.

The combination is often called "network-adaptive BFT": the
protocol matches its theoretical guarantees to current
network conditions, instead of always running the
worst-case protocol.

## System and threat model

- *Jolteon mode.* Partial synchrony, `f < n / 3`, ~2 RTT
  latency.
- *Ditto mode.* Asynchronous, `f < n / 3`, expected `O(1)`
  rounds with common coin (HoneyBadger-style; see module
  0070).

## Theory

### Mode-switching

```
default mode: Jolteon (fast)
on detecting prolonged asynchrony (e.g., > k consecutive
view-changes without commit):
  switch to Ditto mode
on detecting return to synchrony (e.g., k stable
commits in Ditto):
  switch back to Jolteon
```

Mode-switch is itself a consensus decision: replicas commit
to the new mode via a small Jolteon round.

### How Jolteon-Ditto compares

| property                | Jolteon alone | HoneyBadger BFT | Jolteon-Ditto |
| ----------------------- | ------------- | --------------- | ------------- |
| sync regime latency     | ~2 RTT        | ~30s (drand)    | ~2 RTT        |
| async regime liveness   | no            | yes             | yes           |
| mode-switch overhead    | n/a           | n/a             | small         |
| fault tolerance         | f < n/3       | f < n/3         | f < n/3       |
| best of both worlds     | no            | partial         | yes           |

The structural advantage: Ditto preserves both Jolteon's
fast-path latency under good conditions and HoneyBadger-
style async liveness under adversarial conditions.

### Why this matters

Real networks oscillate between synchronous and
asynchronous behaviour (peak vs off-peak load, regional
outages, BGP route flaps). Static-mode protocols pay a
worst-case latency tax always; mode-adaptive protocols pay
only when the network is actually misbehaving.

## Practice

- *Aptos* explores Ditto-style fallback in research; not
  yet in mainnet production as of 2026.
- *Diem*'s legacy designs included an asynchronous-
  fallback prototype.

## Verifiability

**tag: `partial`.** Jolteon side is `friendly`; the async
fallback uses common-coin / threshold-BLS, which adds ~10^6
constraints per Ditto round.

## References

- Gelashvili, Spiegelman, Xiang, Danezis, Li, Malkhi, Xia,
  Zhou, "Jolteon and Ditto: Network-Adaptive Efficient
  Consensus with Asynchronous Fallback", 2021.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to
2023".
