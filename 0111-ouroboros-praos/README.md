# 0111: Ouroboros Praos

## Historical context

Bernardo David, Peter Gazi, Aggelos Kiayias, and Alexander
Russell published "Ouroboros Praos: An Adaptively-Secure,
Semi-synchronous Proof-of-Stake Blockchain" at Eurocrypt 2018.
Praos extends Ouroboros Classic (module 0110) with three
significant improvements:

1. *VRF-based slot leader election.* Replaces the multi-party
   coin-flip protocol with a Verifiable Random Function. Each
   stakeholder evaluates a VRF on the slot number and their
   private key; the output determines whether they are eligible
   to lead the slot. No interactive coin-flip required.

2. *Adaptive corruption.* Praos handles an adversary that may
   corrupt stakeholders *after* seeing their VRF outputs.
   Mitigated by *forward-secure signatures*: each block-signing
   key is updated immediately after use, so a corrupted
   stakeholder cannot retroactively forge old blocks.

3. *Semi-synchronous network.* Praos relaxes Ouroboros
   Classic's synchronous assumption to a *bounded-delay* model
   with finite (known) `Delta`.

Praos is the variant deployed on Cardano mainnet (since the
PoS upgrade, July 2020).

## System and threat model

- **Network.** Bounded-delay PSS.
- **Failures.** Adaptive Byzantine; safety threshold
  `alpha > 1/2` honest stake.
- **Cryptography.** VRF (Verifiable Random Function), forward-
  secure signatures, standard hashing.
- **Goal.** Adaptively-secure semi-synchronous PoS with
  formal proof.

## Theory

### VRF-based slot leader election

Each stakeholder `i` with stake fraction `f_i` computes
`(value, proof) = VRF(secret_key_i, slot)`. They are *eligible*
to be the leader for that slot if `value < threshold(f_i)`,
where `threshold` is a function calibrated so:

```
P[i is eligible] = 1 - (1 - f)^(f_i)
```

i.e., for the *active slot coefficient* `f`, each stakeholder
is eligible with probability proportional to their stake share.
Multiple stakeholders may be eligible for the same slot (in
which case the slot has a fork) or none (in which case the slot
is empty).

### Forward-secure signatures

Each stakeholder maintains a signing key that updates after
every block. The update is *one-way*: the new key cannot derive
old keys. So if an adversary corrupts a stakeholder, they can
sign future blocks but not retroactively re-sign old blocks.

This blocks *posterior-corruption attacks*: even if the
adversary controls past stake, they cannot rewrite history.

### Theorem (David-Gazi-Kiayias-Russell 2018, informal)

Under bounded-delay PSS with adaptive Byzantine adversary
controlling `< 1/2` stake (and `f` calibrated for `Delta`):
Praos satisfies CP / CG / CQ with parameters depending on
`alpha, Delta, f`.

### Comparison: Classic vs Praos vs Genesis

| property             | Classic          | Praos            | Genesis        |
| -------------------- | ---------------- | ---------------- | -------------- |
| seed generation      | coin-flip        | VRF              | VRF + bootstrap |
| network model        | synchronous      | semi-sync (bounded) | semi-sync   |
| adaptive adversary   | no               | yes              | yes            |
| forward-secure sigs  | no               | yes              | yes            |
| trusted bootstrap    | required         | required         | not required   |
| year                 | 2017             | 2018             | 2018           |

Praos is the practical default for production deployment;
Genesis adds the bootstrapping property without trusted setup.

### Properties

- *Adaptive-corruption resistant* via forward-secure signatures.
- *Semi-synchronous* (bounded-delay).
- *No coin-flip*; faster slot start.
- *Provably secure* with formal CP / CG / CQ proofs.

### Subsequent work

- *Ouroboros Genesis* (module 0112). Bootstrapping from genesis
  without trusted setup.
- *Ouroboros Crypsinous* (0113). Privacy-preserving stake
  proofs.
- *Ouroboros Chronos* (0114). Time-aware, fault-tolerant
  clock synchronisation.
- *Ouroboros Leios* (0115). Throughput improvements.
- *Ouroboros Peras* (0116). Faster finality gadget.

## Practice

- *Cardano mainnet.* Production Praos variant since July 2020.
- ~3000 stake pools. ~25 billion ADA staked.
- Block time: 20 seconds (active slot coefficient `f = 1/20`).
- Slot time: 1 second.
- Epoch length: 432000 slots (~5 days).

### Production-implementation notes

- *VRF:* Cardano uses the Praos-specific VRF construction
  (originally `KES` for key-evolving signatures).
- *Forward-secure signatures:* Implemented via key-evolving
  signatures (KES); each KES key has ~256 sub-keys.
- *Stake distribution snapshot:* Stake is snapshotted at the
  start of each epoch and used for slot-leader determination
  in the next epoch (3 epochs ahead, for the snapshot delay).

## Verifiability and circuit encoding

**tag: `friendly`.**

Praos circuits encode VRF verification, KES (forward-secure)
signature verification, and the chain-protocol state machine.
VRFs and KES on SNARK-friendly curves (e.g., BLS12-381) reduce
constraint counts to ~10^4 per VRF + signature.

Cardano's *Mithril* protocol provides aggregated stake-based
certificates that are SNARK-style succinct.

## Known attacks and limitations

- *Posterior corruption.* Mitigated by forward-secure
  signatures.
- *Stake-grinding.* Mitigated by VRF (output is unbiased given
  unpredictable seed).
- *Long-range attacks.* Mitigated by weak subjectivity.

## References

- David, Gazi, Kiayias, Russell, "Ouroboros Praos: An
  Adaptively-Secure, Semi-synchronous Proof-of-Stake
  Blockchain", Eurocrypt 2018.
- Cardano Foundation, "Ouroboros Praos Specification", 2020.

## Implementation notes

The crate provides a `praos_eligible` predicate computing
whether a stakeholder is eligible for a given slot via a
deterministic pseudo-VRF with stake-weighted threshold.
Tests verify the eligibility distribution.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
