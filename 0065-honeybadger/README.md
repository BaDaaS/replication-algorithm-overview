# 0065: HoneyBadger BFT

## Historical context

Miller, Xia, Croman, Shi, Song published "The Honey Badger
of BFT Protocols" at CCS 2016. HoneyBadger is the first
practical *asynchronous* BFT: it makes no synchrony
assumption at all, yet achieves throughput competitive with
PBFT under benign conditions. The design is the canonical
proof that asynchronous BFT can be practical when armed with
threshold cryptography.

The provocative name signals the protocol's robustness
under arbitrary scheduling: like a honey badger, it does
not care whether the network is fast, slow, or
adversarial.

## System and threat model

- **Network.** Asynchronous, reliable channels (every sent
  message is eventually delivered).
- **Failures.** Byzantine, `f < n / 3`.
- **Cryptography.** Threshold signatures (BLS), threshold
  encryption (for fairness).
- **Goal.** Linearisable BFT under any schedule.

## Theory

### Architecture

HoneyBadger composes three layers:

1. **AVID (asynchronous verifiable information dispersal).**
   Each replica disperses its proposed batch via Bracha-
   style RB; recipients reconstruct a batch from `2f + 1`
   shares. Bandwidth-efficient (`O(|m|/n)` per recipient).
2. **`n` parallel ABAs.** Each replica's batch is voted on
   independently via an asynchronous binary agreement (e.g.,
   MMR 2014, module 0017). The set of accepted batches is
   the round's output.
3. **Threshold encryption.** Transactions in batches are
   encrypted; only after the batches are agreed upon are
   they decrypted via threshold decryption. Prevents
   front-running.

### Per-round flow

```
each replica:
  encrypt local batch with threshold pk
  AVID-disperse encrypted batch
  for each replica's batch:
    vote ABA(yes) if dispersed and seen
  collect ABA outputs: set of accepted batches
  threshold-decrypt accepted batches in round-robin order
  apply transactions to state machine
```

### Theorem (HoneyBadger correctness and throughput)

Under any asynchronous schedule with `f < n / 3` Byzantine,
HoneyBadger satisfies linearisability and probabilistic
liveness in expected `O(1)` rounds.

Throughput: bounded only by network bandwidth; reported
throughput in the paper is comparable to PBFT under benign
schedules and far exceeds PBFT under adversarial schedules.

### How HoneyBadger differs from prior BFT

| property                | PBFT             | HoneyBadger      |
| ----------------------- | ---------------- | ---------------- |
| timing model            | partial sync     | async            |
| leader bottleneck       | yes (single primary) | no (parallel) |
| latency under good network | ~3 RTT       | ~few RTT (parallel ABAs) |
| latency under adversarial scheduling | unbounded | bounded by RB + ABA |
| fault-free throughput   | high             | competitive      |
| common-coin requirement | no               | yes (threshold-BLS) |
| front-running mitigation | no              | yes (threshold encryption) |
| best-of-n parallelism   | no               | yes              |

The structural insight: instead of a single leader (which
becomes a target under adversarial scheduling),
HoneyBadger has every replica propose simultaneously.
Asynchronous ABA agrees on *which proposals* to include.

### How HoneyBadger differs from Bracha-Tendermint

Tendermint and Bracha BFT (modules 0055, 0015) operate per
slot; HoneyBadger operates per *batch* (many slots' worth of
operations at once). The batching gives HoneyBadger its
throughput advantage: amortises the threshold cryptography
overhead over many transactions.

## Practice

- *Original implementation.* Python prototype, 2016.
- *Influence.* Dumbo (module 0066), Tusk-as-mempool, drand-
  based async BFT.
- *Production-ish.* Some private blockchain deployments
  inspired by HoneyBadger, but no major mainnet uses
  vanilla HoneyBadger.

### Why it's not deployed at large scale

- *Threshold-DKG cost.* Each epoch requires a fresh DKG;
  expensive at large `n`.
- *AVID overhead.* Per-batch dispersal is bandwidth-heavy.
- *Threshold-encryption complexity.* Production systems
  often opt for simpler patterns (e.g., commit-reveal).

## Verifiability

**tag: `friendly`.** Per-round in circuit:

- `n` ABA proofs: each ~10^6, total ~`n * 10^6` constraints.
- Threshold-encryption decrypt witness: ~10^6 per BLS
  pairing.
- AVID reconstruction: erasure-coding constraints, ~10^4
  per share.

Total per round (HoneyBadger epoch): `~n * 10^6 + 10^6`.
For `n = 100`, ~10^8 constraints.

Recursive aggregation reduces per-epoch proof size to O(1).

## References

- Miller, Xia, Croman, Shi, Song, "The Honey Badger of BFT
  Protocols", CCS 2016.

See also [`HISTORY.md`](../HISTORY.md), section "2015 to
2019".
