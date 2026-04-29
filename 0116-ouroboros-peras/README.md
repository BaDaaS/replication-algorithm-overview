# 0116: Ouroboros Peras

## Historical context

Ouroboros Peras was published by IOG (Cardano research, 2024) as
a *fast finality gadget* for the Ouroboros family. Praos
(module 0111) provides probabilistic finality with `k = 2160`
block depth (~12 hours); Peras adds a deterministic, BFT-style
finality overlay that finalises blocks in ~30-60 seconds when
the network and validators are healthy.

Peras is conceptually similar to Casper FFG (module 0106) but
designed natively for the Ouroboros chain protocol. It is
compatible with Praos and Leios; eventual mainnet integration
is planned for Cardano.

## System and threat model

- **Network.** Bounded-delay PSS.
- **Failures.** Adaptive Byzantine; safety threshold `f < 1/3`
  voting committee.
- **Cryptography.** BLS aggregate signatures (Mithril-style).
- **Goal.** Fast deterministic finality on Praos chain.

## Theory

### Voting committee

Each round, a Praos-style VRF lottery selects a *finality
committee* (~hundreds of stakeholders). Committee members vote
on the most recent stable block.

### Two-phase commit

Peras runs PBFT-style prepare/commit on the latest stable block:

1. *Prepare.* Committee members sign "I support block `B`".
2. *Commit.* On 2/3 prepare signatures, members sign commit.
3. *Finalise.* On 2/3 commit signatures, block `B` is final.

### Aggregate signatures via Mithril

The committee's prepare/commit votes are aggregated using
Mithril (Chaum-Pedersen-style threshold signatures).
Aggregate is verified in constant time, scaling to ~thousand-
member committees.

### Theorem (IOG 2024, informal)

Under bounded-delay PSS with adaptive Byzantine `< 1/3` of
the finality committee: Peras finalises blocks in `O(1)` rounds
and `O(committee_size)` BLS signature verifications.

### Comparison: Cardano finality options

| layer       | mechanism            | latency | safety threshold | year |
| ----------- | -------------------- | ------- | ----------------- | ---- |
| Praos alone | probabilistic depth  | ~12 hours | alpha > 1/2     | 2018 |
| Mithril     | aggregate stake cert | ~minutes | n/a              | 2022 |
| Peras       | BFT finality overlay | ~30-60s | f < 1/3 committee | 2024 |

Peras gives Cardano deterministic finality without sacrificing
Praos's open membership.

### Properties

- *Deterministic finality* in ~30-60 seconds.
- *Compatible* with Praos and Leios.
- *Aggregate-signature-based* (scalable).
- *Permissionless* committee selection (VRF).

### Limitations

- *Committee selection latency.* Each finality round needs a
  fresh VRF-selected committee.
- *Aggregate-signature scalability.* Mithril requires careful
  parameter tuning for large committees.
- *Adversarial committee.* If a single round's committee has
  `> 1/3` Byzantine, finality stalls.

### Subsequent work

- *Mithril* (2022). Aggregate-signature certificates for
  light clients; foundation for Peras.
- *Hydra* (Cardano L2). Off-chain throughput.

## Practice

Peras is in active development; testnet integration with
Praos and Leios is planned for 2025-2026.

## Verifiability and circuit encoding

**tag: `friendly`.**

Peras circuits encode BLS aggregate-signature verification on
SNARK-friendly curves, similar to Casper FFG (~10^4 to 10^6
constraints per aggregate). Mithril certificates are themselves
SNARK-friendly.

## Known attacks and limitations

- *Long-range attacks.* Mitigated by weak subjectivity
  + Genesis (module 0112).
- *Posterior corruption.* Mitigated by forward-secure
  signatures.
- *Committee bias.* Stake-weighted selection can centralise.

## References

- IOG, "Ouroboros Peras: A Fast Finality Gadget for Cardano",
  IOG research blog, 2024.
- Chaidos, Kiayias, Mithril whitepaper, 2021.

## Implementation notes

The crate provides a minimal Peras committee voting state:
prepare and commit counts per block, and a `Status` enum
(`Pending`/`Prepared`/`Finalised`). Tests verify the
status transitions.

See also [`HISTORY.md`](../HISTORY.md), section "2024 to 2026".
