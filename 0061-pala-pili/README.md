# 0061: Pala and Pili

## Historical context

Tse-Hsuan Chan's 2018-2019 papers introduced two
streamlined-BFT designs developed at the ThunderToken /
Thunder Research project:

- *Pala* (2018, "Pala: A Simple Partially Synchronous
  Blockchain"): an explicit two-chain commit BFT in the
  spirit of Streamlet, with a production focus.
- *Pili* (2019, "Pili: Pipelined Lazy Liveness via Periodic
  View Changes"): adds an asynchronous-fallback layer with
  pipelined lazy liveness, anticipating the Jolteon-Ditto
  split.

Both predate Jolteon and HotStuff-2 chronologically and
arrive at similar two-chain commit ideas independently.

## System and threat model

Partial synchrony, `f < n / 3`, BLS aggregate signatures.

## Theory

### Pala (2018)

- Two voting phases per block: vote and notarise.
- Two-chain finality: a block is final when its grandchild
  is notarised.
- Periodic view changes: the leader rotates every fixed
  number of slots, regardless of fault detection.

### Pili (2019)

- Pala plus asynchronous fallback: under prolonged
  asynchrony (no view-change quorum), switch to a slower
  but live protocol.
- Anticipates the Jolteon-Ditto pattern (module 0058 + 0064).

### How Pala/Pili differ from contemporaries

| property                  | Pala 2018 | Streamlet 2020 | HotStuff 2019 | Jolteon 2021 |
| ------------------------- | --------- | -------------- | ------------- | ------------ |
| commit chain length       | 2         | 3              | 3             | 2            |
| optimistic responsive     | partial   | no             | yes           | yes          |
| async fallback (Pili)     | no        | no             | no            | with Ditto   |
| pedagogical clarity       | medium    | high           | low           | medium       |
| production deployment     | ThunderToken | none       | many (Diem)   | many (Aptos) |
| historical priority       | first 2-chain | first textbook | first OR    | refined OR + 2-chain |

Pala's two-chain rule predates both Jolteon's and HotStuff-2's
formal analysis. The protocol was operationally proven in
the ThunderToken testnet but not adopted as widely as Diem's
HotStuff.

### Why Pala/Pili matter historically

The ideas in modern BFT (two-chain commit, asynchronous
fallback, periodic leader rotation) all appear in Pala/Pili
before becoming canonical in Jolteon-Ditto and HotStuff-2.
Chan's papers are pedagogical predecessors deserving more
citation than they receive.

## Practice

- *ThunderToken / ThunderCore.* Pala-based blockchain in
  production (small market cap).
- *Influence.* Tendermint developers cite Pala in CometBFT
  design notes; Aptos's pacemaker design inherits Pala's
  periodic-rotation pattern.

## Verifiability

**tag: `friendly`.** Per-block ~10^6 (BLS QC).

## References

- Chan, "Pala: A Simple Partially Synchronous Blockchain",
  2018.
- Chan, "Pili: Pipelined Lazy Liveness via Periodic View
  Changes", 2019.

See also [`HISTORY.md`](../HISTORY.md), section "2015 to
2019".
