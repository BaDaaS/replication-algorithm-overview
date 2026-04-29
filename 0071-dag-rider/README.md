# 0071: DAG-Rider

## Historical context

Keidar, Kokoris-Kogias, Naor, Spiegelman published "All You
Need is DAG" at PODC 2021. DAG-Rider made the modern
DAG-BFT pattern explicit: build a *reliable broadcast DAG*
in async, then run a *separate consensus* (Tusk-style) over
the DAG to commit anchors.

The separation is the foundational insight of modern DAG-
BFT: dissemination (DAG) decoupled from agreement (anchor
commit), each scaled independently.

## System and threat model

Asynchronous, `f < n / 3`, threshold-BLS for the random
anchor selection.

## Theory

### DAG construction

Each round, every replica RB-broadcasts a vertex
referencing `2f + 1` vertices from the previous round. The
DAG is constructed locally by each replica; consistency is
maintained by RB.

### Wave-based commit

Every four rounds form a *wave*. At the end of each wave, a
*leader vertex* is randomly chosen via threshold-BLS coin.
If the leader vertex has enough support in the DAG, it
commits, and all its causal ancestors commit in
deterministic order.

### How DAG-Rider differs from Aleph

| property                | Aleph     | DAG-Rider           |
| ----------------------- | --------- | ------------------- |
| commit primitive        | virtual voting | leader anchor + wave |
| wave length             | implicit  | explicit (4 rounds) |
| common coin             | yes       | yes (per wave)      |
| latency (good case)     | ~few RTT  | 4 RTT (wave)        |
| theory                  | event-driven | wave-driven      |
| influence on production | minor     | major (Narwhal-Tusk, Bullshark) |

DAG-Rider's wave-based anchor commit is the structural
template for Narwhal-Tusk (module 0072) and beyond.

## Practice

- Reference design rather than production deployment.
- Influence: Narwhal-Tusk's wave structure descends
  directly.

## Verifiability

**tag: `friendly`.** Per-wave commit ~10^6 (BLS) plus
DAG-vertex Merkle proofs ~10^4 each.

## References

- Keidar, Kokoris-Kogias, Naor, Spiegelman, "All You Need
  is DAG", PODC 2021.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to
2023".
