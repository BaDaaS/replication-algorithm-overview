# 0104: Frosty

## Historical context

In 2024, researchers at the Yin Sirer lab (with Ava Labs)
released *Frosty*, a refined Avalanche-family protocol with
*formal* security guarantees. The original Avalanche paper
(Team Rocket 2018, module 0102) has had its security analysis
revised multiple times; Frosty consolidates the analysis into
a precisely-stated theorem.

Frosty's contribution: clean security proofs in the asynchronous
model with explicit bounds on the failure probability as a
function of parameters `k, alpha, beta`. Importantly, Frosty
clarifies the *adaptive* adversarial model (where the
adversary may corrupt nodes after seeing their messages) and
provides bounds that are tight up to constant factors.

## System and threat model

- **Network.** Asynchronous (FLP-resilient).
- **Failures.** Byzantine, adaptive, with stake-weighted
  fraction `f < 1/3 - epsilon`.
- **Cryptography.** Standard signatures; no special
  cryptographic assumptions.
- **Goal.** Sub-linear-communication consensus with
  exponentially small error.

## Theory

### Refined Snowball

Frosty's voting is the same Snowball as Avalanche: sample `k`,
threshold `alpha`, finalisation counter `beta`. The improvement
is the security proof: rather than informal claims, Frosty
proves:

```
Pr[disagreement] <= n * exp(-Theta(beta * (alpha/k - 1/2)^2))
```

where `n` is the number of nodes. This decays exponentially in
`beta` whenever `alpha > k/2`.

### Adaptive adversary

Frosty handles an *adaptive* adversary that may corrupt
additional nodes after seeing messages. The bound:
provided `f < 1/3 - epsilon` of stake is corruptible at any
time, agreement holds with the same exponential decay.

### Comparison: Avalanche security analyses

| paper          | model           | adversary    | bound type      | year |
| -------------- | --------------- | ------------ | --------------- | ---- |
| Team Rocket    | async           | static       | informal        | 2018 |
| Yin Sirer 2020 | async           | static       | tighter         | 2020 |
| Frosty         | async           | adaptive     | tight, formal   | 2024 |
| Lux            | async           | adaptive     | extension       | 2024 |

Frosty is the current canonical reference for Avalanche's
formal security.

### Properties

- *Tight error bound* depending on `beta, alpha, k`.
- *Adaptive-adversary security*.
- *Sub-linear communication* (Snowball-style).
- *Probabilistic finality*.

### Subsequent influence

- *Lux Network.* Avalanche subnet ecosystem with Frosty-
  inspired optimisations.
- *Sirius / Ava Labs research.* Continued refinements of the
  Avalanche family.

## Practice

Frosty is being adopted by Avalanche Subnets and other
Snow*-family projects. The security analysis is now sufficiently
rigorous for production claims of provable safety.

### Production-implementation notes

- Same parameters as Avalanche/Snowman: `k = 20, alpha = 12,
  beta = 15-20` in mainnet.
- Stake-weighted sampling for adaptive-adversary resistance.
- Soft fork to deploy Frosty's improved analysis in existing
  Avalanche subnets.

## Verifiability and circuit encoding

**tag: `partial`.**

Frosty circuits are similar to Snowman/Avalanche: gossip-query
state machine plus signature verification. The improved formal
analysis lets a SNARK light client argue *quantitative*
security, not just heuristic.

## Known attacks and limitations

- *Same fundamental limits.* Probabilistic finality, partition-
  liveness, parameter sensitivity.
- *Adaptive corruption.* Tightly bounded but not zero;
  high-value transactions need many `beta` confirmations.

## References

- Yin, Sekniqi, et al., "Frosty: Bringing strong liveness
  guarantees to the Avalanche network", arXiv 2404.14250, 2024.
- Yin, Sekniqi, van Renesse, Sirer, "Snow*", Avalanche 2020.

## Implementation notes

The crate provides a `SecurityBound` calculator that estimates
disagreement probability for given `n, k, alpha, beta`. Tests
verify the bound is exponential in `beta`.

See also [`HISTORY.md`](../HISTORY.md), section "2024 to 2026".
