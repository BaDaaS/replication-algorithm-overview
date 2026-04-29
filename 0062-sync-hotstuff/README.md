# 0062: Sync HotStuff

## Historical context

Abraham, Malkhi, Nayak, Ren, Yin published "Sync HotStuff:
Simple and Practical Synchronous State Machine Replication"
at S&P 2020 (preprint 2019). Sync HotStuff is HotStuff
adapted for the *synchronous* model: assumes a known message-
delay bound `Delta` and exploits it to commit in ~2 * Delta
with `f < n/2` (vs HotStuff's `f < n/3` under partial
synchrony).

The synchronous model is harder to satisfy in production but
gives stronger fault tolerance: half the replicas can be
Byzantine.

## System and threat model

- **Network.** Synchronous (known `Delta`).
- **Failures.** Byzantine, `f < n/2`.
- **Goal.** SMR with optimal synchronous resilience.

## Theory

### Why synchrony allows `f < n/2`

Under synchrony, an honest validator can detect a Byzantine
process refusing to vote in time. Quorums shrink: instead
of `2f + 1` of `3f + 1`, just `f + 1` of `2f + 1`. The
"missing" votes are interpreted as faults.

### How Sync HotStuff differs

| property              | HotStuff (PS)    | Sync HotStuff |
| --------------------- | ---------------- | ------------- |
| timing model          | partial sync     | full sync     |
| resilience            | `f < n/3`        | `f < n/2`     |
| commit latency        | 4 RTT            | ~2 * Delta    |
| optimistic responsive | yes              | no (timing-bound) |
| n requirement         | `3f + 1`         | `2f + 1`      |

Trade-off: stronger resilience but tighter synchrony
assumption. Production rarely deploys Sync HotStuff because
real networks aren't reliably synchronous; the partial-
synchrony assumption is more robust.

### Why synchrony helps fault tolerance

Standard intuition: synchrony lets honest replicas detect
silence as a fault. Without synchrony (FLP-style adversary),
silence is indistinguishable from delay; protocols must
tolerate up to `f < n/3` arbitrary behaviour.

## Practice

- *Research benchmark.* Sync HotStuff is the tightest
  synchronous BFT result.
- *Niche deployments.* Internal data-centre BFT where the
  network is calibrated.

## Verifiability

**tag: `friendly`.** Per-block ~10^6 (BLS QC). The synchrony
assumption is non-circuit (timing oracle).

## References

- Abraham, Malkhi, Nayak, Ren, Yin, "Sync HotStuff: Simple
  and Practical Synchronous State Machine Replication", S&P
  2020.

See also [`HISTORY.md`](../HISTORY.md), section "2015 to
2019".
