# 0132: zk-Rollup Sequencer Consensus

## Historical context

zk-rollups are Layer-2 scaling solutions for Ethereum (and
other base layers): transactions are executed off-chain,
batched, and a SNARK proves the batch's validity to the L1.
The *sequencer* is the role responsible for:

1. *Ordering.* Receiving transactions and choosing their
   execution order.
2. *Execution.* Running them and producing state transitions.
3. *Proving.* Generating the validity SNARK.
4. *Posting.* Publishing the SNARK + state root to L1.

Most production zk-rollups have *centralised sequencers*: a
single party orders all transactions. This is fast but
introduces a censorship/MEV risk.

The frontier of research is *decentralised sequencer
consensus*: the rollup runs its own BFT or DAG-BFT among
multiple sequencers, often inheriting consensus from the L1
or from a separate chain.

Production rollups: StarkNet, Polygon zkEVM, zkSync Era,
Scroll, Linea.

## System and threat model

- **Network.** Bounded-delay PSS at the rollup level; L1
  finality at the base layer.
- **Failures.** Centralised: trust the sequencer.
  Decentralised: Byzantine `f < n/3`.
- **Cryptography.** SNARK validity proofs (Groth16, PLONK,
  STARK depending on system).
- **Goal.** Scale L1 throughput while inheriting L1 security.

## Theory

### Centralised sequencer

The simplest model: one operator runs the sequencer. They
receive transactions, order them, execute, generate a SNARK,
and post to L1. Users trust the operator for liveness and
ordering; safety is enforced by the SNARK.

Most production zk-rollups (Polygon zkEVM, StarkNet, zkSync)
operate this way as of 2024-2026, with plans for
decentralisation.

### Decentralised sequencer designs

Several approaches:

1. *L1-driven sequencing.* Use the L1 as the sequencer (every
   tx posted to L1). Low throughput.
2. *Rollup BFT.* Sequencer set runs CometBFT or HotStuff
   among themselves. Used by Astria, Espresso.
3. *DAG-BFT.* Sequencer set runs Narwhal-Bullshark.
   Demonstrated in research; production deployment ongoing.
4. *Shared sequencer.* A separate chain (e.g., Astria, Espresso
   Network) provides ordering for multiple rollups. MEV-aware.
5. *Based rollups.* Sequenced by L1 validators directly.

### MEV considerations

Centralised sequencer can extract maximal MEV (frontrun,
sandwich, etc.). Decentralisation aims to disperse MEV among
many sequencers. Designs include encrypted mempools (Espresso,
SUAVE), MEV auctions, and threshold-encryption-based pre-
commitment.

### Comparison: rollup sequencer designs

| design               | safety              | latency       | MEV resistance |
| -------------------- | ------------------- | ------------- | --------------- |
| Centralised          | trust sequencer     | seconds       | low             |
| L1-driven            | L1                  | minutes       | high            |
| Rollup BFT           | f < n/3 sequencers  | seconds       | medium          |
| Shared sequencer     | shared BFT          | seconds       | medium          |
| Based rollup         | L1 validators       | L1 block time | medium          |

### Subsequent work

- *Astria.* Shared-sequencer chain for rollups.
- *Espresso Network.* Shared-sequencer with threshold-
  encryption MEV protection.
- *SUAVE.* MEV-aware sequencer infrastructure.
- *EigenLayer-secured rollups.* Restaking-based sequencer
  security.

## Practice

- *Polygon zkEVM, StarkNet, zkSync Era, Scroll, Linea.*
  Production rollups, mostly centralised sequencer.
- *Astria.* Shared-sequencer testnet/mainnet.
- *Espresso Network.* Shared-sequencer with HotShot consensus.

### Production-implementation notes

- *State posting.* Sequencer posts state-root + validity proof
  to L1.
- *Force inclusion.* Users can submit transactions directly to
  L1 if sequencer censors them.
- *Sequencer fees.* Sequencer earns gas premium for ordering.

## Verifiability and circuit encoding

**tag: `deployed`.**

zk-rollups *are* verifiability in production: every batch
posts a SNARK validity proof. The sequencer-consensus layer is
the next frontier; it can also be SNARK-verified.

## Known attacks and limitations

- *Sequencer censorship.* Mitigated by force-inclusion.
- *MEV.* Active in centralised; addressed in decentralised.
- *Trusted setup.* Some SNARK systems require setup; STARK-
  based avoids it.
- *Liveness halt.* If sequencer is offline, force-inclusion
  via L1 is the fallback.

## References

- Buterin, "Incentives in Ethereum's Hybrid Casper Protocol",
  2017.
- StarkNet Foundation, "StarkNet Decentralisation Roadmap",
  2024.
- Astria, "The Astria Shared Sequencer", 2023.
- Espresso, "HotShot: A Pipelined HotStuff", 2023.

## Implementation notes

The crate provides a `Sequencer` enum with three variants
(centralised, BFT-set, shared) and a `route` function. Tests
verify routing.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to 2024".
