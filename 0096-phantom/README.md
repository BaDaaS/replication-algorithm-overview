# 0096: PHANTOM and GHOSTDAG

## Historical context

Yonatan Sompolinsky and Aviv Zohar published "PHANTOM: A
Scalable BlockDAG Protocol" (IACR ePrint 2018/104; FC 2020 as
GHOSTDAG). PHANTOM extends SPECTRE (module 0094) to provide a
*total order* over all transactions in a block-DAG, while
preserving SPECTRE's high throughput.

The key idea: identify a *blue set* (well-connected blocks
representing honest mining) and a *red set* (poorly-connected
blocks, i.e., adversarial mining). Order blocks first by their
blue-score (depth in the blue subgraph), then break ties by
hash. The blue/red coloring uses a `k`-cluster heuristic:
blocks with at most `k` non-anti-cone neighbors are blue.

PHANTOM's optimised variant *GHOSTDAG* runs a greedy
heaviest-blue-subgraph algorithm in `O(\|B\| * k)` time, where
`\|B\|` is the DAG size. This is the protocol used by Kaspa.

## System and threat model

- **Network.** Bounded-delay (PSS).
- **Failures.** Computational honest-majority hashing.
- **Cryptography.** SHA-256 PoW.
- **Goal.** High-throughput PoW chain with deterministic total
  ordering.

## Theory

### Anticone and `k`-cluster

For a block `B` in the DAG, define:

- *Past(B).* All ancestors of `B`.
- *Future(B).* All descendants of `B`.
- *Anticone(B).* Blocks neither in `Past(B)` nor `Future(B)`.

A block is *blue* (`k`-cluster) if its anticone has at most
`k` blocks. Adversarial blocks tend to have large anticones
because they are produced in parallel without seeing the
honest network.

PHANTOM picks `k` as a parameter calibrated to the
network-delay bound `Delta`: `k = O(f * Delta)` (i.e., the
expected anticone size of an honest block in steady state).

### Coloring algorithm (PHANTOM)

`O(2^|B|)` exact: iterate over all subsets, find the largest
`k`-cluster. Practical: greedy heuristic (GHOSTDAG).

GHOSTDAG: at each tip, find the parent with the largest
blue-score, take its blue set as the starting cluster, and
greedily add each new block whose anticone-with-blue is at most
`k`. `O(\|B\| * k)` time per block, `O(\|B\|)` storage.

### Total ordering

Order blocks by blue-score (descending) then by hash. Within a
blue cluster, the order is canonical because the cluster is
closed under ancestry.

### Theorem (Sompolinsky-Zohar 2018, informal)

Under PSS-style bounded-delay with honest fraction
`alpha > 1/2 + epsilon` and `k = Theta(f * Delta)`: PHANTOM
provides total ordering with persistence and liveness
analogous to GKL backbone, at high throughput.

### Comparison: SPECTRE, PHANTOM, Conflux

| protocol | order   | k parameter   | algorithm cost | year | production |
| -------- | ------- | ------------- | -------------- | ---- | ---------- |
| SPECTRE  | partial | n/a           | `O(\|B\|^2)` voting | 2016 | research |
| PHANTOM  | total   | `Theta(f*Delta)` | `O(2^|B|)` exact | 2018 | research |
| GHOSTDAG | total   | `Theta(f*Delta)` | `O(\|B\|*k)` greedy | 2020 | Kaspa |
| Conflux  | total   | n/a (pivot)   | `O(\|B\|)` per block | 2020 | Conflux |

GHOSTDAG (the optimised PHANTOM) is the production-deployed
version, used by Kaspa.

### Properties

- *Total ordering.* Smart contracts and applications work as on
  Bitcoin/Ethereum.
- *High throughput.* Block rate scales with network capacity,
  not propagation delay.
- *No orphans.* Every honest block contributes.
- *Permissionless.* Any miner can join.

### Limitations

- *Parameter tuning.* `k` must be calibrated to the actual
  network conditions; under-estimating breaks safety, over-
  estimating breaks performance.
- *DAG storage.* Linear in block rate * time; no permanent
  pruning without sacrificing the coloring algorithm.
- *Confirmation latency.* Although throughput is high, individual
  block confirmation is delayed by the DAG depth needed for
  coloring stability.

## Practice

- *Kaspa.* GHOSTDAG in production since 2021. Block rate ~1
  block per second; can handle thousands of tx/s.
- *Aleph Zero (module 0070).* Inspired by but distinct from
  PHANTOM; uses BFT instead of PoW.

### Production-implementation notes

- Kaspa uses `k = 18` in mainnet, calibrated to ~1-second block
  intervals and ~5-second worldwide propagation delay.
- DAG pruning: Kaspa prunes very-old blocks; the security
  argument shifts from "DAG ordering" to "longest pruned-blue
  chain".
- GHOSTDAG performance: per-block coloring is `O(\|B\| * k)`;
  becomes a bottleneck for large DAGs.

## Verifiability and circuit encoding

**tag: `partial`.**

PHANTOM/GHOSTDAG circuits encode SHA-256 PoW per block plus the
DAG coloring algorithm. Greedy GHOSTDAG can be encoded as a
sequence of comparisons, but cost grows with DAG size.
SNARK light clients for Kaspa typically prove only a *blue
subchain* (the canonical pruned chain), recovering Bitcoin-like
costs.

## Known attacks and limitations

- *Anticone manipulation.* Adversary may produce blocks with
  carefully-crafted anticones to delay coloring stability.
  Sompolinsky-Zohar bound this within `k`.
- *Spam DAG.* High-rate adversarial mining inflates DAG storage.
- *Mining-power capture.* Standard `> 1/2` threshold breaks all
  guarantees.

## References

- Sompolinsky, Zohar, "PHANTOM: A Scalable BlockDAG Protocol",
  IACR ePrint 2018/104.
- Sompolinsky, Zohar, "Bitcoin's Underlying Incentives",
  Communications of the ACM, 2018.
- Sompolinsky, Wyborski, Zohar, "PHANTOM and GHOSTDAG",
  AFT 2021 (revised analysis).

## Implementation notes

The crate provides a `BlockDag` (re-using the SPECTRE
abstraction at module-level) and a simple anticone computation.
A full `k`-cluster algorithm is omitted; tests verify the
anticone of a small DAG.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
