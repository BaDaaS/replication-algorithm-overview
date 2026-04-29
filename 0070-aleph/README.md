# 0070: Aleph

## Historical context

Gagol, Lesniewski-Laas, Madej, Straszak, Swietek published
"Aleph: Efficient Atomic Broadcast in Asynchronous
Networks with Byzantine Nodes" at AFT 2019. Aleph is the
first DAG-based async BFT with formal proofs and
production-grade engineering. It introduces the
*round-by-round* DAG: each round, every replica produces
exactly one *unit* (block) referencing the previous
round's units.

## System and threat model

Asynchronous, `f < n / 3`, threshold-BLS for the common
coin.

## Theory

### Round-by-round DAG

Each unit references `n - f` parent units from the previous
round. After enough rounds, virtual voting (HashGraph-style
or coin-based) commits a total order on units.

### How Aleph differs from HashGraph

| property              | HashGraph    | Aleph       |
| --------------------- | ------------ | ----------- |
| DAG structure         | gossip-driven (random other-parent) | round-structured (each round, n - f parents) |
| termination guarantee | deterministic (gossip mixing) | probabilistic (common coin) |
| typical commit time   | ~log n RTT   | ~few RTT (with coin) |
| common coin           | none         | yes (threshold-BLS) |
| async                 | yes          | yes         |
| formal proofs         | partial      | full        |

The structural change: Aleph's *round-by-round* DAG gives
the protocol clearer phase boundaries than HashGraph's
free-form gossip, making formal analysis tractable.

## Practice

- *Aleph Zero* blockchain runs Aleph in production (mainnet
  since 2021).
- *Influence.* DAG-Rider (module 0071), Narwhal-Tusk (0072),
  Bullshark (0073) descend conceptually.

## Verifiability

**tag: `friendly`.** Per-unit Merkle proof + BLS signature
~10^4 constraints. Per-round commit cert ~10^6.

## References

- Gagol et al., "Aleph: Efficient Atomic Broadcast in
  Asynchronous Networks with Byzantine Nodes", AFT 2019.

See also [`HISTORY.md`](../HISTORY.md), section "2015 to
2019".
