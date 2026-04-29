# 0069: HashGraph

## Historical context

Leemon Baird's 2016 whitepaper "The Swirlds Hashgraph
Consensus Algorithm" introduced HashGraph, the first
practical *DAG-based* asynchronous BFT. Each event in
HashGraph references its parent and one randomly-chosen
"other-parent", forming a DAG of gossip events. Consensus
emerges from the DAG's topology: a virtual voting protocol
runs over the DAG without exchanging additional consensus
messages.

HashGraph is deployed in the Hedera blockchain (mainnet
since 2019). Patent-encumbered until 2025; this slowed
adoption.

## System and threat model

- **Network.** Asynchronous, gossip-based.
- **Failures.** Byzantine, `f < n / 3`.
- **Cryptography.** Standard signatures.
- **Goal.** Total-order broadcast over a gossip DAG.

## Theory

### Gossip about gossip

Every event includes:

- Two parent hashes (self-parent, other-parent).
- A signature.
- Optional payload (transactions).

Each replica gossips events to a randomly-chosen peer; the
peer integrates new events into its local DAG. The "gossip
about gossip" pattern: the events themselves carry the
information needed for consensus.

### Virtual voting

Once the DAG has enough events, each replica can
*deterministically* compute consensus order by
"virtually voting" over the DAG: which events are seen by
which others, and at what rounds. No vote messages are
exchanged; the DAG topology is sufficient.

### How HashGraph differs from prior async BFT

| property                | HoneyBadger      | HashGraph         |
| ----------------------- | ---------------- | ----------------- |
| communication           | structured (RB+ABA) | gossip (random)|
| consensus messages      | explicit votes   | implicit (DAG topology) |
| common coin             | threshold-BLS    | none (deterministic) |
| async                   | yes              | yes               |
| typical latency         | ~few RTT         | ~log n RTT (gossip mixing) |
| patent issues           | none             | yes (until 2025)  |

The DAG-as-consensus pattern is HashGraph's lasting
contribution; it directly inspired Aleph (module 0070),
DAG-Rider (0071), Narwhal-Tusk (0072), and the entire
modern DAG-BFT line.

### Why deterministic without a coin?

HashGraph proves termination without a common coin via a
careful argument about gossip-event reachability under
asynchrony. The trade-off: rounds-to-commit grows with
`log n` (gossip mixing time), vs HoneyBadger's `O(1)`
(with coin).

## Practice

- *Hedera Hashgraph.* Production deployment since 2019.
- *Influence.* DAG-BFT family (Aleph, Narwhal-Tusk,
  Bullshark, Mysticeti) descends conceptually.
- *Patent expiry (2025).* Has opened up academic and
  open-source deployment.

## Verifiability

**tag: `partial`.** The DAG is naturally Merkle-friendly
(each event has its parents' hashes). Per-event proof
~constant. Full-DAG consistency proof is `O(n^2)` for `n`
recent events but recursive aggregation works.

## References

- Baird, "The Swirlds Hashgraph Consensus Algorithm:
  Fair, Fast, Byzantine Fault Tolerance", Swirlds Tech
  Report 2016.

See also [`HISTORY.md`](../HISTORY.md), section "2015 to
2019".
