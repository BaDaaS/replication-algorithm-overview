# 0108: Algorand

## Historical context

Jing Chen and Silvio Micali published "Algorand: A Secure and
Efficient Distributed Ledger" at SOSP 2017 (extended in TCS
2019). Algorand introduced *cryptographic sortition*: a
verifiable random function (VRF) selects a small committee of
proposers and voters per round from the entire stake-weighted
population. Committee membership is *secret* until the member
reveals their VRF proof; this prevents adaptive corruption.

Algorand's headline claim was achieving *unconditional* safety
under network partitions (no fork, ever) at *responsive*
latency (seconds) under good conditions. The protocol is named
after Micali (Turing Award 2012) and reflects deep theoretical
roots: the BA* (Byzantine Agreement-star) protocol it uses is
itself a result from Chen-Micali's earlier game-theoretic
work.

Algorand mainnet launched in 2019; throughput ~1000 tx/s,
finality in seconds.

## System and threat model

- **Network.** Partial synchrony (safety always; liveness during
  good periods).
- **Failures.** Byzantine, adaptively corruptible; safety
  threshold `< 1/3` of stake.
- **Cryptography.** VRF (Verifiable Random Function), Ed25519
  or BLS signatures.
- **Goal.** Unconditional safety + responsive liveness +
  permissionless PoS.

## Theory

### Cryptographic sortition

Each user evaluates a VRF on the round number and their
private key. The output is a (value, proof) pair; the user is
*selected* into the committee if their VRF value is below a
threshold proportional to their stake.

Selection is *self-confirmable*: anyone can verify the VRF
proof, but the selection is *secret* until the user reveals
their proof. Adversaries cannot pre-target committee members.

### BA* (Byzantine Agreement *)

Algorand's three-phase BA* protocol:

1. *Proposal.* Selected proposers broadcast their proposed
   blocks.
2. *Soft vote.* Selected committee members vote on the
   block with the lowest VRF.
3. *Cert vote.* Selected committee members vote to certify the
   block.

If 2/3 of cert votes agree, the block is certified (final).
Otherwise, the protocol enters a recovery loop.

### Adaptive-corruption resistance

Cryptographic sortition makes the committee members secret
until they speak. Even an adaptive adversary cannot corrupt
them in time: by the time the adversary identifies a member,
their vote is already broadcast. After voting, the member is
no longer needed (the vote is signed; corrupting them later
doesn't help).

This is called *player replaceability*: each step uses a fresh,
independently-selected committee, so the adversary cannot
pre-corrupt them.

### Theorem (Chen-Micali 2019, informal)

Under partial synchrony with `< 1/3` adaptively-corruptible
stake: Algorand BA* achieves agreement with negligible
disagreement probability and `O(1)` rounds in good conditions.

### Comparison: Algorand vs PBFT vs Avalanche

| property            | Algorand      | PBFT           | Avalanche     |
| ------------------- | ------------- | -------------- | ------------- |
| committee selection | VRF sortition | fixed          | random sample |
| adaptive adversary  | yes           | no             | yes (Frosty) |
| safety              | always        | partial sync   | async         |
| finality            | deterministic | deterministic  | probabilistic |
| latency             | seconds       | seconds        | seconds        |
| open membership     | yes (PoS)     | no             | yes (PoS)     |
| messages per round  | O(c) (c = committee) | O(n^2) | O(c * log n)  |

Algorand's main advantage is unconditional safety and
adaptive-adversary resistance via player replaceability.

### Properties

- *Unconditional safety* (no fork, even in partition).
- *Responsive liveness* (seconds under good conditions).
- *Adaptive-corruption resistant* (player replaceability).
- *Permissionless PoS.*

### Limitations

- *VRF dependency.* Requires a secure VRF (existing
  constructions are reasonably efficient but not standard).
- *Liveness halts during partition.* Like all partition-
  intolerant designs.
- *Stake-weighted.* Wealthy validators get more committee
  selection probability.

### Subsequent work

- *Vault* (Leung-Suresh-Hart 2019). Algorand variant for
  rollups.
- *Snow White* (Daian-Pass-Shi 2019, module 0124). Sleepy-PoS
  with similar VRF-based selection.
- *Ouroboros Praos* (module 0126). PoS with VRF-based slot
  leader election.

## Practice

- *Algorand mainnet.* Production since 2019.
- ~1000 to 6000 tx/s, sub-5-second finality.
- Used for asset tokenisation, payments, DeFi.

### Production-implementation notes

- *VRF construction.* Algorand uses the Goldberg VRF (sometimes
  called "Algorand VRF") on Ed25519.
- *Committee size.* Variable, ~3000 to 10000 stake-weighted
  units per step.
- *Round time.* ~3-5 seconds per block.
- *Stake distribution.* Top stakeholders effectively dominate
  committee selection.

## Verifiability and circuit encoding

**tag: `partial`.**

Algorand circuits encode VRF verification, signature
verification, and the BA* state machine. VRFs over Ed25519
are not SNARK-friendly (~10^6+ constraints); a SNARK-friendly
VRF (e.g., over BLS12-381) would reduce this to ~10^4. zk-
bridges to Algorand have been proposed but require careful VRF
substitution.

## Known attacks and limitations

- *Long-range attacks.* Mitigated by weak subjectivity.
- *Stake centralisation.* Cryptographic sortition is fair given
  stake, but stake itself can centralise.
- *VRF leak.* If a validator's VRF private key leaks, their
  past selections become forgeable; forward-secure VRFs
  mitigate.

## References

- Chen, Micali, "Algorand: A Secure and Efficient Distributed
  Ledger", TCS 2019.
- Gilad, Hemo, Micali, Vlachos, Zeldovich, "Algorand: Scaling
  Byzantine Agreements for Cryptocurrencies", SOSP 2017.

## Implementation notes

The crate provides a deterministic stand-in for the VRF: a
hash-based selection function that picks committee members
based on a seed and stake. Tests verify selection
proportionality.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
