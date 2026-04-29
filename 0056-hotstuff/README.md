# 0056: HotStuff

## Historical context

Yin, Malkhi, Reiter, Gueta, Abraham published "HotStuff: BFT
Consensus with Linearity and Responsiveness" at PODC 2019.
HotStuff is the structural breakthrough of modern BFT:

- *Linear authenticator complexity per phase.* `O(n)` MACs/
  signatures per phase, vs PBFT's `O(n^2)`. Achieved via
  threshold signatures.
- *Optimistic responsiveness.* Bounded latency under stable
  networks: each new view's first block commits within one
  round-trip after view stabilisation, regardless of any
  prior partition.
- *Chained pipeline.* Multiple proposals in flight; each
  block's commit relies on the next block's QC.

HotStuff is the basis of every production BFT since 2019:
LibraBFT/DiemBFT (Facebook's Libra), Aptos's consensus,
Sui's Bullshark/Mysticeti family, Cypherium, ConsenSys
Quorum.

## System and threat model

- **Network.** Partial synchrony.
- **Failures.** Byzantine, `f < n / 3` (`n = 3f + 1`).
- **Cryptography.** BLS threshold signatures (or any
  aggregatable signature scheme).
- **Goal.** Linearisable SMR with linear authenticator
  complexity.

## Theory

### Four phases per block

```
prepare:    leader proposes block; replicas vote; QC formed.
pre-commit: replicas vote on prepare-QC; new QC formed.
commit:     replicas vote on pre-commit-QC; new QC formed.
decide:     replicas commit the block.
```

Each phase produces a *quorum certificate* (QC) over `2f +
1` signatures. The QC is a single aggregate signature; the
leader collects votes and aggregates.

### Chained pipelining

Adjacent blocks share phases:

```
block_k prepare-vote = block_{k-1} pre-commit
block_k pre-commit-vote = block_{k-1} commit
block_k commit-vote = block_{k-1} decide-trigger
```

So each new block effectively progresses every other block
in the pipeline by one phase. Steady-state throughput: one
block per round-trip.

### Theorem (HotStuff correctness and optimistic
responsiveness)

Under partial synchrony with `f < n / 3`, HotStuff:

- *Safety.* No two honest replicas commit different blocks
  at the same height. (Standard BFT quorum-intersection.)
- *Liveness.* Eventually a block commits.
- *Optimistic responsiveness.* If the network is stable for
  `delta` between view changes, the new view's first block
  commits in `O(delta)`, independent of any prior delays.

Yin et al. 2019, Theorems 5 and 6.

### Why linearity matters

PBFT's `O(n^2)` authenticator complexity per phase makes it
impractical at large validator counts (>100). HotStuff's
`O(n)` linear authenticator (one BLS aggregate per QC)
scales to thousands of validators. Modern PoS chains
(Ethereum, Aptos, Sui) routinely have hundreds of
validators; HotStuff's design makes this feasible.

## Practice

### Production lineage

- *LibraBFT (2019).* Facebook's Libra blockchain.
- *DiemBFT v1, v2, v3, v4 (2019-2021).* Diem's evolution.
- *Aptos.* Quorum Store + DiemBFTv4.
- *Sui (initial).* Bullshark (DAG variant).
- *HotStuff-2 (2023).* Removes the four-phase pipeline lag.
- *Various Cypherium, Solidity-style chains.*

### Engineering refinements

- *Pacemaker.* Separates leader rotation from consensus.
  Each leader has a fixed term; pacemaker rotates if the
  leader fails.
- *Block proposal.* Leaders include the previous block's QC
  in their proposal; this is the "extends" relation.
- *State sync.* New validators catch up via QC chains.

## Formalisation aspects

```text
structure Block where
  parent : BlockHash
  qc     : QuorumCert
  payload: List Operation

structure QuorumCert where
  height   : Nat
  block    : BlockHash
  agg_sig  : Signature

theorem hotstuff_safety
    (n f : Nat) (h : 3 * f + 1 = n) :
    forall (sched : PartiallySync) (corrupt : Byzantine f),
    forall (b1 b2 : Block),
    Committed b1 -> Committed b2 ->
    SameHeight b1 b2 -> b1 = b2 := by
  -- 2f + 1 QC intersection.
  sorry
```

HotStuff has been formalised in TLA+ (Malkhi-Nayak 2020) and
in Coq (Berkovits et al. 2019).

## Verifiability and circuit encoding

**Tag: `friendly`.**

HotStuff is the canonical SNARK-friendly BFT. Per-block proof:

- BLS QC verification: ~10^6 constraints (one pairing).
- Block-extends relation: ~10^4 (Merkle path in chain).

Total per block: ~10^6. Pickles-style recursion gives
constant proof for arbitrary chain length.

This is why Aptos, Sui, and zk-bridge designs to/from
HotStuff-family chains adopt this template.

## Known attacks and limitations

- *View-change cost.* Each view change is `O(n)` messages
  per phase, `O(n)` phases worst-case = `O(n^2)`.
- *Pipeline lag.* The four-phase chain means a fresh block
  takes 4 rounds to commit. HotStuff-2 removes this.

## Implementation notes

The crate provides type-level definitions of HotStuff
blocks and QCs.

## References

- Yin, Malkhi, Reiter, Gueta, Abraham, "HotStuff: BFT
  Consensus with Linearity and Responsiveness", PODC 2019.

See also [`HISTORY.md`](../HISTORY.md), section "2015 to 2019".
