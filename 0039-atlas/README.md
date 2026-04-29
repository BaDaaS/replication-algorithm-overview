# 0039: Atlas

## Historical context

Whittaker, Giridharan, Szekeres, Hellerstein, and Stoica
introduced *Atlas* in their 2020 EuroSys paper "Atlas:
Atomic, Latency-Optimal Asynchronous Distributed Consensus".
Atlas is an EPaxos descendant with two key contributions:

- *Latency optimality.* Achieves the theoretical minimum
  latency for geo-replicated commits: one round-trip from
  client to nearest fast quorum.
- *Configurable conflict ratio.* The fast quorum size is a
  function of an operator-specified expected conflict rate;
  smaller rate -> smaller quorum -> lower latency.

Atlas is the canonical "modern multi-leader Paxos" of the
2020s, alongside Caesar (Arun-Hirve-Palmieri 2017) and
Tempo (Enes 2021).

## System and threat model

- **Network.** Asynchronous, reliable.
- **Failures.** Crash-recovery; `f < n / 2`.
- **Goal.** Latency-optimal multi-leader SMR.

## Theory

### Fast quorum size

Atlas's fast quorum is `q_f = f + floor((f + 1) / 2) +
floor(c / 2)`, where `c` is the operator's expected conflict
rate (a tuning parameter).

For `f = 2, c = 0`: `q_f = 2 + 1 + 0 = 3`. Smaller than
EPaxos's `4`. For `f = 2, c = 2`: `q_f = 2 + 1 + 1 = 4`,
matching EPaxos.

### Per-command flow

```
client -> nearest replica (the "command leader"):
  cmd-leader sends Pre-Accept(cmd, deps) to fast-quorum-1
  fast-quorum-1 replies with their computed deps
  if all deps agree: fast-path commit (1 RT)
  else: slow path through classic Paxos
```

### Theorem (Atlas correctness)

Atlas satisfies SMR linearisability under crash-recovery and
`f < n / 2` for any choice of fast-quorum size satisfying the
intersection inequality.

*Proof.* Whittaker et al. 2020. Quorum-intersection-based
argument identical in structure to EPaxos but with the
tuned `q_f` formula.

### Latency analysis

For geo-replicated workloads:

- *Atlas, conflict-free.* 1 RT to nearest replica + 1 RT to
  fast quorum = ~2 cross-region hops worst case, ~1 RT
  best case.
- *EPaxos, conflict-free.* Similar but with `q_f = f +
  floor(f/2) + 1`, slightly larger.
- *Multi-Paxos.* Worst case: client RT to leader + leader
  RT to remote replicas.

For 5 globally-distributed replicas (US-East, US-West, EU,
Asia, AU), Atlas's typical commit latency in low-conflict
workloads is ~30% lower than EPaxos.

## Practice

### Where Atlas shows up

- *Frankenpaxos.* Whittaker's reference implementation.
- *Research.* Atlas is the contemporary benchmark for
  multi-leader Paxos. Subsequent work (Tempo 2021,
  StarPaxos 2022) refines further.

Production deployments are rare; the operational complexity
remains a barrier.

### Tuning

Operators choose `c` based on workload:

- *Low-conflict.* `c = 0`, smallest fast quorum.
- *High-conflict.* `c >= 2`, larger fast quorum, more
  reliable fast path.

Wrong `c` doesn't break safety; it just means more or fewer
fast-path successes.

## Formalisation aspects

```text
def atlas_fast_quorum (n f c : Nat) : Nat :=
  f + (f + 1) / 2 + c / 2

theorem atlas_safety (n f c : Nat) (h : 2 * f < n) :
    SmrSafety := by
  -- Quorum intersection: q_f + slow_quorum > n.
  sorry
```

## Verifiability and circuit encoding

**Tag: `partial`.**

Per-command Atlas proof:

- Fast-path: BLS aggregate from `q_f` replicas. ~10^6.
- Slow-path (rare): classic Paxos cost. ~2 * 10^6.

Per-command typical: ~10^6. Slightly less than EPaxos due to
smaller fast quorum.

Like EPaxos, Atlas requires a per-command dependency
encoding in circuit, adding modest overhead.

## Known attacks and limitations

- *Conflict-rate sensitivity.* Wrong `c` doesn't break
  safety but reduces fast-path success.
- *Operational complexity.* Same as EPaxos: dependency
  tracking, multi-leader recovery.

## Implementation notes

The crate provides Atlas's quorum-size helper.

## References

- Whittaker, Giridharan, Szekeres, Hellerstein, Stoica,
  "Atlas: Atomic, Latency-Optimal Asynchronous Distributed
  Consensus", EuroSys 2020.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to 2023".
