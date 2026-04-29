# 0059: HotStuff-2

## Historical context

Malkhi and Nayak's 2023 paper "HotStuff-2: Optimal Two-Phase
Responsive BFT" proved that HotStuff's four-phase pipeline is
not necessary for the linearity and responsiveness
properties. HotStuff-2 reaches both with just two phases per
block, achieving the *minimum* phase count for any
optimistically-responsive BFT under partial synchrony with
linear authenticator complexity.

The paper resolved an open question from the original
HotStuff: was three-chain or four-chain the minimum? Answer:
two-chain suffices, with a refined locking rule.

## System and threat model

Same as HotStuff: partial synchrony, `f < n / 3`,
BLS-aggregate signatures.

## Theory

### Why HotStuff needed four phases

HotStuff's three-chain commit rule (`B -> B' -> B''`) is
correct, but the proof requires *four* phases of voting per
block to maintain optimistic responsiveness across
view-changes. Specifically: if a view changes mid-pipeline,
the new leader needs the *previous* `pre-commit` QC to
extend safely; without four phases, a Byzantine leader can
trick the protocol into a chain inconsistency.

Malkhi-Nayak observe that this can be fixed by changing the
*locking rule*: a replica locks on a value when it sees a
QC at a certain rank, and only votes for proposals extending
its lock. With this refined rule, two phases suffice.

### How HotStuff-2 differs from HotStuff and Jolteon

| property               | HotStuff (PODC 19) | Jolteon (21)         | HotStuff-2 (23)      |
| ---------------------- | ------------------ | -------------------- | -------------------- |
| phases per block       | 4                  | 2 (commit) + 2 (chain) | 2                  |
| commit rule            | three-chain        | two-chain            | two-chain (refined)  |
| view-change argument   | implicit pipelined | extra invariant      | refined locking rule |
| common-case rounds     | 4                  | 2                    | 2                    |
| view-change responsiveness | optimistic     | optimistic           | optimistic           |
| asynchronous fallback  | no                 | yes (Ditto)          | no                   |
| proof complexity       | high               | medium               | medium               |
| linear authenticator   | yes                | yes                  | yes                  |

The structural progression: HotStuff (2019) introduced
optimistic responsiveness with linear authenticator. Jolteon
(2021) showed two-chain commit was safe with one extra
invariant. HotStuff-2 (2023) showed that with the right
locking rule, two-chain commit was *natively* safe under
HotStuff's exposition, removing Jolteon's auxiliary
invariants. The three are increasingly minimal but
equivalent in functional power.

### Optimality

Malkhi-Nayak prove that any BFT with linear authenticator
complexity and optimistic responsiveness needs at least two
phases. HotStuff-2 achieves this bound; further reduction
would require either super-linear authenticator (PBFT) or
losing optimistic responsiveness (Streamlet).

## Practice

- *Aptos.* Adopted HotStuff-2 ideas in their consensus
  refinements.
- *Subsequent BFT research (2024+).* HotStuff-2 is the
  modern baseline; Multi-leader HotStuff variants build on
  it.

## Verifiability

**tag: `friendly`.** Per-block ~10^6 constraints (one BLS
QC pairing). Same constraint cost as HotStuff and Jolteon;
the saving is in real-time latency, not prover work.

## References

- Malkhi, Nayak, "HotStuff-2: Optimal Two-Phase Responsive
  BFT", 2023.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to
2023".
