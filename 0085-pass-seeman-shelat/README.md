# 0085: Pass-Seeman-Shelat Asynchronous-Network Analysis

## Historical context

Rafael Pass, Lior Seeman, and Abhi Shelat published "Analysis of
the Blockchain Protocol in Asynchronous Networks" at Eurocrypt
2017. It extends the GKL backbone analysis (module 0084) from
the synchronous round model to a more realistic
*partially synchronous* (sometimes called *bounded-delay*) model
parametrised by a network-delay bound `Delta`.

GKL 2015 assumed honest messages are delivered in one round.
Real networks are bounded-delay: there is a constant `Delta`
such that any message sent at time `t` is delivered by time
`t + Delta`, but the adversary controls timing within that
window. Pass et al. prove that Nakamoto's protocol still
satisfies CP / CG / CQ, but with weaker constants depending on
`Delta`.

## System and threat model

- **Network.** Bounded-delay: every honest message is delivered
  to every honest party within `Delta` time units. The adversary
  may delay, reorder, but not drop or fabricate honest messages.
- **Failures.** Computational majority of honest hashing power.
  Adversary may be Byzantine, may corrupt parties statically.
- **Cryptography.** Random-oracle SHA-256.
- **Parameter.** `Delta`: known network-delay bound (in
  rounds, time units, or block-arrival rate).

### Why bounded delay matters

In synchronous rounds, an honest miner who finds a block has it
delivered to all peers before the next round. In bounded-delay,
two honest miners may each find a block within the same `Delta`
window, neither aware of the other. The result is a *natural
fork* even without adversarial action.

Pass et al. quantify this: the rate of natural forks scales as
`f * Delta` where `f` is the per-time-unit mining probability.
If `f * Delta` is small (block intervals much greater than
network delay), forks are rare; if `f * Delta` is close to 1,
forks are frequent and the chain can stagnate.

## Theory

### Theorem (PSS 2017, informal)

Under the bounded-delay model with delay `Delta` and honest
hashing fraction `alpha > 1/2`, the backbone protocol satisfies
CP / CG / CQ provided:

```
alpha * (1 - 2 * f * Delta) >= (1 + delta) * beta
```

for some `delta > 0`, where `beta = 1 - alpha` is the
adversarial fraction and `f` is the per-time mining
probability.

The crucial difference from GKL: the synchronous bound
`alpha * (1 - 2 * f) >= ...` becomes
`alpha * (1 - 2 * f * Delta) >= ...`. The honest-power penalty
scales with `f * Delta`.

### Bitcoin parameter analysis

Bitcoin's mainnet:

- Block interval: ~600 seconds.
- Network propagation `Delta`: ~10 seconds (95th percentile).
- `f * Delta = (1/600) * 10 = 1/60`.

Substituting: `alpha * (1 - 2/60) >= (1 + delta) * beta`. For
`alpha = 0.55`, `beta = 0.45`: `0.55 * 58/60 = 0.532` vs
`(1 + delta) * 0.45`, satisfied for `delta < 0.18`. So Bitcoin
provides a security margin if honest hashing fraction exceeds
~52%.

### Comparison with prior analyses

| Property              | GKL 2015 | PSS 2017      | GKL 2017 (var diff) |
| --------------------- | -------- | ------------- | ------------------- |
| Network               | sync     | bounded delay | sync                |
| Difficulty            | constant | constant      | variable            |
| Adversary             | static   | static        | static              |
| Bound on `t/n`        | `< 1/2`  | `< 1/2 - O(f * Delta)` | `< 1/2`        |
| Common prefix proof   | yes      | yes           | yes                 |
| Bound tightness       | tight (sync) | open       | tight (within model) |

PSS 2017 is generally considered the most practically relevant
analysis: real networks are bounded-delay, not synchronous.

### Proof techniques

PSS introduce the notion of a *convergence opportunity*: a time
window of length `> Delta` in which only honest miners produce
blocks. They show:

1. Convergence opportunities occur at rate proportional to
   `alpha * (1 - f * Delta)`.
2. Each convergence opportunity reduces the gap between the
   longest honest chain and the longest adversarial chain.
3. With overwhelming probability, the gap grows linearly,
   ensuring CP and CQ.

This style of analysis is now standard in PoS proofs (e.g.,
Ouroboros, modules 0125 onward) and in DAG-BFT analyses (e.g.,
Bullshark, module 0073).

### Subsequent refinements

- *Garay-Kiayias-Panagiotakos 2020.* Variable difficulty in
  bounded-delay networks.
- *Kiffer-Rajaraman-Shelat 2018.* Concrete numeric bounds (e.g.,
  exact security levels for given parameters).
- *Dembo et al. 2020.* Tighter bounds via relativistic arguments.
- *Garay-Kiayias-Leonardos 2024.* Adaptive corruptions in
  bounded-delay networks.

## Practice

PSS 2017 is the analysis Bitcoin engineers usually quote when
discussing security margins. Key practical implications:

- *Block-interval choice.* Long intervals (~10 minutes) give
  small `f * Delta` and large security margin. Litecoin (2.5
  min) and Dogecoin (1 min) reduce the margin.
- *Network propagation.* Reducing `Delta` (better gossip, FIBRE,
  compact blocks) directly improves security.
- *Hash power thresholds.* The minimum honest fraction depends
  on `f * Delta`; higher `f * Delta` means higher required
  honest fraction.

## Verifiability and circuit encoding

**tag: `partial`.**

The PSS bound is a property of the chain protocol, not a
verifiable property of a specific chain. SNARK light clients
verify CP/CG/CQ on the actual chain, with cost dominated by
SHA-256 (see module 0083).

Some work uses PSS bounds as *parameters* in circuit-checked
predicates (e.g., a circuit that accepts a chain header sequence
only if it satisfies a `Delta`-conformant CP property).

## Known attacks and limitations

- *Selfish mining.* PSS analysis does not prevent selfish
  mining (Eyal-Sirer 2014; module 0087); it bounds safety, not
  incentive compatibility.
- *Network partitioning.* If a partition lasts longer than
  `Delta`, the bound does not apply during the partition. After
  the partition heals, the longer chain wins.
- *Adaptive corruptions.* PSS assumes static adversary; adaptive
  models give weaker bounds.

## References

- Pass, Seeman, Shelat, "Analysis of the Blockchain Protocol in
  Asynchronous Networks", Eurocrypt 2017.
- Garay, Kiayias, Panagiotakos, "Consensus from Signatures of
  Work", CT-RSA 2020.
- Kiffer, Rajaraman, Shelat, "A Better Method to Analyze
  Blockchain Consistency", CCS 2018.

## Implementation notes

The crate provides a simple bounded-delay simulator: events
occur at integer times, messages have delay in `[1, Delta]`, and
miners produce blocks at random times. Tests verify that the
adversarial fork rate matches the predicted `f * Delta` scaling.

See also [`HISTORY.md`](../HISTORY.md), section "2009 to 2014".
