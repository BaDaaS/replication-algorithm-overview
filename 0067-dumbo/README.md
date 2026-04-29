# 0067: Dumbo

## Historical context

Guo, Lu, Lu, Tang, Zhang, Zhao published "Dumbo: Faster
Asynchronous BFT Protocols" at CCS 2020. Dumbo refines
HoneyBadger to address its main bottleneck: the `n`
parallel ABA instances per epoch. Dumbo runs a *committee
ABA*: only a small subset of replicas participate in each
ABA, with the committee elected via VRF.

The result: from `n` ABA instances per epoch (HoneyBadger)
to `O(log n)` (Dumbo1) or just one (Dumbo2). Throughput at
n = 100: ~5x HoneyBadger.

## System and threat model

Asynchronous, `f < n / 3`, threshold-BLS, VRFs.

## Theory

### Dumbo1: committee-based ABA

```
each epoch:
  elect a committee of k = O(log n) replicas via VRF
  these k replicas run an MMR-style ABA
  remaining n - k replicas observe
```

### Dumbo2: single-batch ABA

A clever reformulation: each epoch needs only *one* ABA
instance, on the question "which proposers' batches are
included". The single ABA decides a bitmap of committed
batches.

### How Dumbo differs from HoneyBadger

| property              | HoneyBadger | Dumbo1 | Dumbo2 |
| --------------------- | ----------- | ------ | ------ |
| ABAs per epoch        | n           | O(log n) | 1   |
| throughput at n=100   | ~1500 tx/s  | ~7000 tx/s | ~10000 tx/s |
| committee selection   | n/a         | VRF    | n/a    |
| AVID dispersal        | yes         | yes    | yes    |
| async                 | yes         | yes    | yes    |

The structural improvement: HoneyBadger's `n` ABAs were
its dominant cost; Dumbo's single (or `log n`) ABAs slash
the bottleneck.

### How Dumbo differs from Bullshark

Bullshark (module 0073) is a DAG-based BFT under partial
synchrony with no ABA instances at all; Dumbo retains
asynchrony but uses ABA. The two are alternative
solutions to the same throughput problem.

## Practice

- Reference implementation in Go.
- Influence on Speeding Dumbo (module 0068) and on
  general async BFT design.

## Verifiability

**tag: `friendly`.** Per-epoch: 1 ABA proof + `n` AVID
witnesses. ~10^7 constraints for n = 100, ~5x cheaper than
HoneyBadger.

## References

- Guo, Lu, Lu, Tang, Zhang, Zhao, "Dumbo: Faster
  Asynchronous BFT Protocols", CCS 2020.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to
2023".
