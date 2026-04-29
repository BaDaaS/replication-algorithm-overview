# 0094: SPECTRE

## Historical context

Yonatan Sompolinsky, Yoad Lewenberg, and Aviv Zohar published
"SPECTRE: A Fast and Scalable Cryptocurrency Protocol"
(IACR ePrint 2016/1159). SPECTRE generalises Bitcoin's chain
structure to a *block DAG*: each block references multiple
parents (not just one), giving up linear chain ordering in
exchange for high throughput.

In SPECTRE, blocks form a directed acyclic graph rather than a
chain; each new block references all *tips* of the DAG at the
time of mining. The DAG admits parallel mining without
orphaning honest blocks. Conflict resolution between conflicting
transactions uses a pairwise voting rule based on DAG
ancestry.

SPECTRE was the first block-DAG protocol; it is the ancestor of
PHANTOM (module 0096), Conflux (module 0099), and the
DAG-BFT family (modules 0070-0082). Its main weakness is that
it provides a partial-order on transactions only; not a total
order. Subsequent protocols (PHANTOM, Conflux) restored total
ordering.

## System and threat model

- **Network.** Bounded-delay (PSS-style).
- **Failures.** Computational majority of honest hashing power.
- **Cryptography.** SHA-256 PoW.
- **Goal.** High-throughput PoW with rapid pairwise transaction
  confirmation.

## Theory

### Block DAG

Each block references all tips of the DAG seen by the miner at
mining time. Tips are blocks with no descendants in the
miner's view. Newly mined block becomes a fresh tip.

Properties:

- *No orphans.* Honest blocks are always in the DAG.
- *Parallel mining.* Multiple miners can produce blocks
  simultaneously without wasting work.
- *Throughput.* Limited only by per-miner block rate, not by
  global propagation delay.

### Pairwise voting rule

Given two conflicting transactions `tx_1` and `tx_2`, SPECTRE
asks every block in the DAG to *vote*:

- If `tx_1` is in block `B_1` and `tx_2` is in block `B_2`:
  every block sees the order in which it received `B_1` and
  `B_2`, and votes for the earlier one.
- A transaction is *accepted* once the cumulative vote
  (weighted by descendants) is decisive.

Sompolinsky et al. prove: in the bounded-delay model with
honest-majority hashing, conflicting transactions converge to a
unique winner with probability `1 - exp(-Theta(k))` after `k`
descendant blocks.

### Partial vs total order

SPECTRE provides a *pairwise* ordering: for any two
non-conflicting transactions, no order is needed; for any pair
of conflicting transactions, the protocol picks one. This
suffices for cryptocurrency payments (each payment either is
or is not in the ledger) but does not suffice for smart
contracts (which may need a total order over all transactions).

PHANTOM (module 0096) and Conflux (module 0099) restore total
ordering using DAG-traversal heuristics.

### Comparison: chain vs DAG

| protocol     | structure | order | throughput | confirmation latency | year |
| ------------ | --------- | ----- | ---------- | -------------------- | ---- |
| Bitcoin      | chain     | total | low        | ~1 hour              | 2008 |
| GHOST        | tree      | total | medium     | ~1 hour              | 2013 |
| SPECTRE      | DAG       | partial | high     | seconds (pairwise)   | 2016 |
| PHANTOM      | DAG       | total | high       | seconds (linearised) | 2018 |
| Conflux      | DAG       | total | high       | seconds (PoW + pivot)| 2020 |

### Properties

- *Pairwise consistency.* Conflicting transactions converge to
  a unique winner with overwhelming probability.
- *High throughput.* Block rate can scale to network capacity.
- *Fast pairwise confirmation.* Within seconds for typical
  parameters.
- *Cumulative work counted.* All honest work contributes to
  security.

### Limitations

- *No total order.* Smart contracts and ordering-dependent
  applications (DEX, MEV-aware) require an extension.
- *DAG bookkeeping.* Each node must maintain the entire DAG
  (linear in block rate * time).
- *Honest-majority dependence.* Like all PoW chains.
- *Cumulative-voting complexity.* The voting rule is
  computationally expensive; SPECTRE specifies pruning
  heuristics, but implementation complexity is high.

### Subsequent influence

- *PHANTOM.* Sompolinsky-Zohar 2018 add a coloring algorithm
  to restore total order.
- *Conflux.* Pivot-chain selection plus ordered references.
- *DAG-BFT (Narwhal, Bullshark).* Apply the DAG idea to BFT
  rather than PoW.

## Practice

- *Kaspa* (cryptocurrency, 2021 onward). Production deployment
  of GHOSTDAG (a PHANTOM/SPECTRE successor).
- *Aleph Zero (module 0070).* DAG-BFT in production.

## Verifiability and circuit encoding

**tag: `partial`.**

SPECTRE circuits encode SHA-256 PoW per block (as in Bitcoin)
plus DAG traversal for the voting rule. Voting rule
verification is `O(n^2)` in the number of blocks in the
relevant time window; expensive in SNARKs.

A more practical SNARK approach: encode only the DAG structure
and a finalisation rule (e.g., "all blocks with at least `k`
descendants are stable"), accepting eventual-consistency rather
than instant pairwise voting.

## Known attacks and limitations

- *Long-range adversarial DAG.* An attacker who can mine a
  parallel DAG branch with comparable cumulative work can
  influence the voting rule. Honest-majority assumption rules
  this out probabilistically.
- *Double-spend attempts.* Attempted double-spends are visible
  to all blocks; voting rule resolves them.
- *Spam DAG.* High-rate mining can balloon DAG storage costs.

## References

- Sompolinsky, Lewenberg, Zohar, "SPECTRE: A Fast and Scalable
  Cryptocurrency Protocol", IACR ePrint 2016/1159.

## Implementation notes

The crate provides a `BlockDag` struct holding a parent map
(each block references multiple parents). Tests verify tip
detection (blocks with no descendants) and DAG ancestry.

See also [`HISTORY.md`](../HISTORY.md), section "2014 to 2017".
