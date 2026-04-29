# 0115: Ouroboros Leios

## Historical context

Ouroboros Leios was published by Aggelos Kiayias, Alexander
Russell, Adam Schoulder, and Filippo Spagnuolo (Cardano
research, IOG 2024). Leios extends the Ouroboros family with
a *throughput-first* design: parallel input-block (IB),
endorser-block (EB), and ranking-block (RB) layers, similar in
spirit to Prism (module 0097) but adapted to PoS.

Leios's headline claim: throughput approaching network bandwidth
(thousands of tx/s) while preserving Praos-level security.
The protocol is in active development as of 2024-2026; a
testnet release is planned for the Cardano network.

## System and threat model

- **Network.** Bounded-delay PSS.
- **Failures.** Adaptive Byzantine `< 1/2`.
- **Cryptography.** VRF + KES (as Praos).
- **Goal.** PoS throughput approaching network bandwidth.

## Theory

### Three block types

| layer        | role                            | rate         |
| ------------ | ------------------------------- | ------------ |
| Input blocks  | bandwidth-saturating tx batches | very high   |
| Endorser blocks | endorse input blocks         | medium      |
| Ranking blocks  | order endorser blocks        | low (Praos) |

This is the same decoupling idea as Prism but adapted to
slot-based PoS.

### Slot lottery for each layer

Each layer has its own VRF-based slot lottery (Praos-style).
The thresholds are calibrated such that:

- Input blocks: many per slot.
- Endorser blocks: ~10 per slot.
- Ranking blocks: ~1 per slot.

### Comparison: Ouroboros throughput variants

| protocol  | structure       | tx throughput | latency  | year |
| --------- | --------------- | ------------- | -------- | ---- |
| Praos     | linear chain    | ~250 tx/s     | ~20 sec  | 2018 |
| Hydra     | L2 channels     | high          | ~seconds | 2020 |
| Leios     | three-layer DAG | ~thousands/s  | ~minutes | 2024 |

### Properties

- *High throughput* (network-bandwidth-bound).
- *Same Praos security* via the ranking-block layer.
- *Compatible* with Praos-based Cardano.
- *Incremental upgrade* (deployable as a soft fork).

### Limitations

- *Latency.* Confirmation latency is higher than plain Praos
  due to multi-layer aggregation.
- *Storage.* Multiple block layers grow chain storage faster.
- *Specification stability.* As of 2024-2026, Leios is still
  evolving; final spec subject to change.

### Subsequent work / pairing

- *Ouroboros Peras* (module 0116). Faster finality gadget,
  pairs naturally with Leios.
- *Hydra* (Cardano L2). Off-chain throughput.

## Practice

Leios is in development as of 2024-2026; planned for Cardano
testnet and eventual mainnet integration.

## Verifiability and circuit encoding

**tag: `friendly`.**

Leios circuits encode three-layer VRF + signature checks plus
inter-layer reference structure. Cost ~3x Praos's per-block
cost. Mithril-style aggregate certificates can summarise stake
across layers.

## Known attacks and limitations

- *Layer DoS.* An adversary specialised in input blocks could
  spam the network; mitigated by stake-weighted lottery.
- *Endorsement gaming.* Endorsers can selectively endorse
  certain input blocks; analysis bounds this.

## References

- Kiayias, Russell, Schoulder, Spagnuolo, "Ouroboros Leios:
  Increased Cardano Throughput", IOG research, 2024.

## Implementation notes

The crate provides three block-type structs and a `LeiosLedger`
that stores them per layer. Tests verify each layer can grow
independently.

See also [`HISTORY.md`](../HISTORY.md), section "2024 to 2026".
