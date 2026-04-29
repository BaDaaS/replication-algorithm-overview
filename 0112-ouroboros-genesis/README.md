# 0112: Ouroboros Genesis

## Historical context

Christian Badertscher, Peter Gazi, Aggelos Kiayias, Alexander
Russell, and Vassilis Zikas published "Ouroboros Genesis:
Composable Proof-of-Stake Blockchains with Dynamic Availability"
at CCS 2018. Genesis solves a fundamental practical issue with
PoS protocols: *bootstrapping* a new node from genesis without
relying on an existing trusted checkpoint.

In Praos and earlier Ouroboros variants, a new node joining the
network must trust a *recent finalised checkpoint* (weak
subjectivity). This is acceptable for warm joins but limits the
"trustless" property of PoS: a node syncing from genesis on its
own would be vulnerable to long-range rewrites.

Genesis adds a *chain-density* tie-breaker rule: when faced
with two competing chains both claiming valid stake history,
prefer the one with greater density (more blocks per unit
time) within a *common-prefix forkable interval*. This rules
out long-range attacks built from sparsely-mined adversarial
chains.

## System and threat model

- **Network.** Bounded-delay PSS.
- **Failures.** Adaptive Byzantine; safety threshold
  `alpha > 1/2`.
- **Cryptography.** VRF + forward-secure signatures (as Praos).
- **Goal.** Bootstrappable PoS without trusted checkpoint.

## Theory

### Long-range attack

In naive PoS, an attacker who acquires majority stake at *some
past time* can rewrite history from that time onwards. The
attacker can produce a long alternative chain spanning years,
indistinguishable from the canonical chain by stake-fraction
alone.

PoW solves this implicitly: rewriting requires accumulated
work proportional to time. PoS does not have this physical
constraint; the attacker can mine the alternative chain
quickly with their captured keys.

### Genesis chain-density rule

For two chains diverging at point `P`:

1. Identify the *forkable interval*: time `[P, P + L]` where
   `L` is the security parameter.
2. Count blocks in this interval on each chain.
3. Prefer the chain with more blocks (higher density).

The intuition: the canonical chain (with majority honest stake)
will have more blocks per unit time than any adversarial branch
(with minority captured stake), even if both produce valid
signatures.

### Theorem (BGKRZ 2018, informal)

Under bounded-delay PSS with adaptive Byzantine `< 1/2`:
Ouroboros Genesis achieves CP / CG / CQ even for nodes
bootstrapping from genesis, *without* a trusted checkpoint.

### Comparison: Praos vs Genesis

| property             | Praos               | Genesis               |
| -------------------- | ------------------- | --------------------- |
| trusted bootstrap    | required            | not required          |
| long-range defence   | weak subjectivity   | chain density rule    |
| network model        | semi-sync           | semi-sync             |
| adaptive corruption  | yes                 | yes                   |
| year                 | 2018                | 2018                  |

### Properties

- *Bootstrap from genesis* without trusted setup.
- *Long-range attack resistant* via chain-density.
- *Adaptive corruption* tolerated.
- *Composable proof* in UC framework (Universal Composability).

### Limitations

- *Density-rule subtlety.* Implementation must compute density
  carefully across stake snapshots.
- *Bootstrapping latency.* A new node must download enough
  history to evaluate the density rule across forkable
  intervals.
- *Adversarial-density attacks.* If the adversary controls
  enough stake to match honest density at brief intervals,
  the rule can be exploited; analysis bounds this.

### Subsequent work

- *Cardano* uses Genesis-style density rules in production
  (combined with weak subjectivity for practical bootstrapping).
- *Mina (Samasika)*. Recursive SNARK-style succinct proof of
  density.
- *Ethereum's GASPER* uses a different mechanism (slashing +
  weak subjectivity).

## Practice

- *Cardano* incorporates Genesis ideas (chain-density tie-breaker)
  in production, though most operational nodes still use weak-
  subjectivity checkpoints for fast warm starts.
- *Mina (module 0145).* Samasika protocol uses recursive SNARK
  proofs of Genesis-style density for succinct light clients.

## Verifiability and circuit encoding

**tag: `friendly`.**

Genesis circuits encode the chain-density predicate (block
count over a window) plus VRF + signature verification.
Density verification is simple arithmetic; total cost is
dominated by signatures.

Mina's Samasika protocol (module 0145) encodes the entire
Genesis-style chain selection in a Pickles SNARK, achieving a
~constant-size light-client proof.

## Known attacks and limitations

- *Stake-grinding.* Mitigated by VRF.
- *Posterior corruption.* Mitigated by forward-secure
  signatures.
- *Density-window optimisation.* Adversary may try to match
  honest density at carefully-timed windows; analysis bounds
  this.

## References

- Badertscher, Gazi, Kiayias, Russell, Zikas, "Ouroboros
  Genesis: Composable Proof-of-Stake Blockchains with Dynamic
  Availability", CCS 2018.
- Cardano Foundation, "Genesis specification", 2020.

## Implementation notes

The crate provides a chain-density `density` function counting
blocks within a `(start, end)` window, plus a `prefer_dense`
selector that picks the denser of two candidate chains. Tests
verify the selection on simple cases.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
