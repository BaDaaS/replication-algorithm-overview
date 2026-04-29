# 0110: Ouroboros Classic

## Historical context

Aggelos Kiayias, Alexander Russell, Bernardo David, and Roman
Oliynykov published "Ouroboros: A Provably Secure Proof-of-Stake
Blockchain Protocol" at Crypto 2017. Ouroboros is the first
provably-secure proof-of-stake protocol with formal reductions
to the GKL backbone properties (CP / CG / CQ).

Ouroboros Classic uses *epoch-based* slot leader election:
each epoch is divided into slots; a slot leader is chosen by
multi-party random sampling (using a coin-flipping protocol)
weighted by stake. The selected leader produces a block; if
they're offline, the slot is empty.

Ouroboros Classic is the first member of the Ouroboros family,
which has grown to include Praos (module 0111), Genesis (0112),
Crypsinous (0113), and others. The family powers the Cardano
blockchain (mainnet 2017; PoS upgrade 2020).

## System and threat model

- **Network.** Synchronous in safety; partially synchronous in
  liveness.
- **Failures.** Byzantine; safety threshold `alpha > 1/2`
  honest stake.
- **Cryptography.** Coin-flipping protocol (publicly verifiable
  secret sharing); standard signatures.
- **Goal.** Provably secure proof-of-stake.

## Theory

### Epoch and slot structure

Time is divided into epochs of `R` slots each. Within an
epoch, slot leaders are pre-determined by a *seed* generated
by the previous epoch's coin-flip protocol.

The seed plus stake distribution determine, for each slot,
the eligible leader. If the leader is honest and online, they
produce a block; otherwise the slot is empty.

### Leader election by stake-weighted lottery

Each stakeholder has stake fraction `s_i / S`, where `S =
sum(s_i)`. A slot is assigned to stakeholder `i` with
probability `s_i / S`. The lottery is verifiable: given the
seed, anyone can recompute the slot-to-stakeholder mapping.

### Coin-flipping for next-epoch seed

At the start of each epoch, the slot leaders run a
*coin-flipping* protocol (publicly verifiable secret sharing
based) to generate the seed for the *next* epoch. This makes
seed generation Byzantine-resilient: an adversary controlling
< 1/2 stake cannot bias the seed.

### Theorem (KKDO 2017, informal)

Under the synchronous model with honest stake `alpha > 1/2`:
Ouroboros Classic satisfies CP / CG / CQ with parameters
depending on `alpha` and the epoch length `R`.

### Security parameter

The security parameter `k` (depth of common prefix) must satisfy
`k = O(log(1/epsilon) * 1/alpha)` for failure probability
`epsilon`. In Cardano practice, `k = 2160` (~12 hours).

### Comparison: PoW Bitcoin, Ouroboros Classic, Algorand

| property              | Bitcoin       | Ouroboros Classic | Algorand   |
| --------------------- | ------------- | ----------------- | ---------- |
| Sybil resistance      | hash power    | stake             | stake      |
| network model         | bounded delay | synchronous       | partial sync |
| leader election       | PoW           | seeded lottery    | VRF sortition |
| committee             | n/a           | per-slot leader   | per-step   |
| seed generation       | n/a           | coin-flip         | VRF chain  |
| nothing-at-stake      | physical      | slashing          | slashing   |
| year                  | 2008          | 2017              | 2017/2019  |

Ouroboros Classic was the first PoS protocol with a formal
proof of equivalence to Bitcoin's GKL backbone properties.

### Properties

- *Provable security* (formal CP/CG/CQ).
- *Synchronous safety* with `alpha > 1/2`.
- *Deterministic finality* via depth `k`.
- *Permissionless* (any stakeholder can join lottery).

### Limitations

- *Synchronous network requirement* (relaxed in Praos).
- *Online stakeholders only.* If a slot leader is offline, the
  slot is empty.
- *Stake-weighted bias.* Wealthy stakeholders win more slots.
- *Coin-flip cost.* The PVSS protocol scales as `O(n^2)`.

### Subsequent work

- *Ouroboros Praos* (module 0111). Adds VRF for slot-leader
  selection, removes coin-flip, supports semi-synchronous
  network.
- *Ouroboros Genesis* (0112). Bootstrapping from genesis
  without trusted-setup.
- *Ouroboros Crypsinous* (0113). Privacy-preserving Ouroboros.

## Practice

- *Cardano mainnet.* Production Ouroboros (Praos variant) since
  2020. Block time 20 seconds, slot time 1 second, epoch length
  ~5 days.
- ~3000 stake pools.
- Coin-flip protocol replaced by VRF in Praos.

### Production-implementation notes

- *Slot interval.* Cardano: 1 second.
- *Epoch length.* Cardano: 432000 slots (~5 days).
- *Active slot coefficient.* `f = 1/20` (~20-second blocks).
- *Common-prefix depth.* `k = 2160` (~12 hours).

## Verifiability and circuit encoding

**tag: `friendly`.**

Ouroboros circuits encode VRF (Praos) or coin-flip seeds
(Classic), signature verification, and the chain-protocol
state machine. Praos (and onward) use BLS aggregate signatures
which are SNARK-friendly. Cardano's *Mithril* protocol provides
SNARK-style succinct certificates for stake distributions.

## Known attacks and limitations

- *Stake-grinding.* Adversaries with adaptive stake distribute
  bias the seed; mitigated by coin-flip.
- *Long-range attacks.* Mitigated by weak subjectivity.
- *Posterior corruption.* Mitigated by forward-secure
  signatures (Praos).

## References

- Kiayias, Russell, David, Oliynykov, "Ouroboros: A Provably
  Secure Proof-of-Stake Blockchain Protocol", Crypto 2017.
- David, Gazi, Kiayias, Russell, "Ouroboros Praos: An
  Adaptively-Secure, Semi-synchronous Proof-of-Stake
  Blockchain", Eurocrypt 2018.

## Implementation notes

The crate provides a deterministic stake-weighted slot-leader
lottery: given a seed, slot, and stake distribution, picks the
single leader. Tests verify deterministic output and stake-
proportional empirical selection.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
