# 0114: Ouroboros Chronos

## Historical context

Christian Badertscher, Peter Gazi, Aggelos Kiayias, Alexander
Russell, and Vassilis Zikas published "Ouroboros Chronos:
Permissionless Clock Synchronization via Proof-of-Stake" at
CCS 2019. Chronos addresses a foundational issue in
slot-based PoS: *clock synchronisation*.

Praos (module 0111) and most slot-based PoS protocols assume
that participants share a synchronised global clock. In
practice, clocks drift; without synchronisation, participants
disagree on which slot is current, and the protocol's safety
analysis breaks down.

Chronos provides a permissionless clock-synchronisation gadget
that runs alongside the chain protocol: nodes use the
chain itself as a fault-tolerant time reference,
counter-acting clock drift even when an `< 1/2` adversary
deliberately mis-reports time.

## System and threat model

- **Network.** Bounded-delay PSS.
- **Failures.** Adaptive Byzantine `< 1/2` stake. Adversarial
  clocks: arbitrary drift up to a bounded rate.
- **Cryptography.** Praos cryptography (VRF + KES) plus
  signed timestamp messages.
- **Goal.** Maintain pairwise clock-skew bound `< Delta`
  among honest nodes despite drift and adversarial mis-
  reports.

## Theory

### Clock model

Each node has a local clock that ticks; ticks may not be
identical across nodes (drift). Adversarial nodes may
report arbitrary time-stamps in their messages.

### Chronos algorithm

Periodically (e.g., once per epoch), nodes exchange
*timestamp votes*: each honest node signs a message saying
"my current local time is `T`". The collected signed
timestamps are filtered: extreme outliers discarded;
median computed.

Each honest node then *adjusts* its local clock toward the
median. The honest-majority assumption ensures the median is
honest, so honest clocks converge.

### Theorem (BGKRZ 2019, informal)

Under bounded-delay PSS with adaptive Byzantine `< 1/2`:
Chronos maintains pairwise clock-skew bound `< Delta` among
honest nodes for ever, despite arbitrary drift and
adversarial timestamps. Combined with Praos, the chain
protocol satisfies CP/CG/CQ.

### Comparison: clock-sync gadgets

| protocol      | network  | adversary    | sync mechanism        | year |
| ------------- | -------- | ------------ | --------------------- | ---- |
| NTP           | internet | malicious    | trusted servers       | 1985 |
| PTP           | LAN      | benign       | hardware timestamps   | 2002 |
| Chronos       | PSS      | Byzantine    | permissionless median | 2019 |
| Polkadot BABE | BFT      | Byzantine    | committee timestamps  | 2019 |

Chronos is the first formally-proven permissionless clock-
sync gadget for PoS chains.

### Properties

- *Permissionless clock sync.*
- *Byzantine-tolerant.*
- *Composable* with Praos.
- *Eventually-bounded skew.*

### Limitations

- *Convergence speed.* Median-aggregation converges in
  `O(1/alpha)` rounds, not instantly.
- *Cold start.* New nodes must download recent timestamp
  votes to align their clocks.
- *Drift handling.* Clocks must drift slowly relative to
  protocol steps.

### Subsequent influence

- *Cardano* uses Chronos-style time-on-chain mechanisms.
- *Solana's Proof-of-History* is a related but
  different approach (cryptographic time-stamps via VDF).
- *Polkadot BABE* incorporates timestamp gossip.

## Practice

Chronos has been deployed in Cardano alongside Praos. It
keeps cluster clocks in sync without trusted NTP servers.

### Production-implementation notes

- *Timestamp interval.* Cardano: once per epoch (~5 days).
- *Filter outlier threshold.* Drop top/bottom 5% of
  timestamps.
- *Local-clock adjustment rate.* Limit per-update adjustment
  to avoid instability.

## Verifiability and circuit encoding

**tag: `partial`.**

Chronos circuits encode timestamp-vote signatures and a
median-aggregation predicate. Cost is dominated by
signature verification per timestamp; median computation is
cheap arithmetic.

## Known attacks and limitations

- *Coordinated drift.* If `>= 1/2` of stake-holders' clocks
  drift in the same direction, the median follows them.
  Mitigation: stake decentralisation.
- *Filter parameter tuning.* Outlier-filter must be set
  conservatively to avoid honest-clock drops.

## References

- Badertscher, Gazi, Kiayias, Russell, Zikas, "Ouroboros
  Chronos: Permissionless Clock Synchronization via Proof-of-
  Stake", CCS 2019.

## Implementation notes

The crate provides a `median_clock` function: given a list of
signed timestamps from stakeholders, computes the trimmed
median for clock adjustment. Tests verify median calculation
on small inputs.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
