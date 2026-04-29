# 0016: Cachin-Kursawe-Shoup 2000 and CKPS 2001 Async BFT

## Historical context

The Cachin-Kursawe-Shoup PODC 2000 paper "Random Oracles in
Constantinople: Practical Asynchronous Byzantine Agreement
Using Cryptography" was the first practical asynchronous BFT
protocol with a *cryptographic* common coin and constant
expected rounds. The follow-up CKPS 2001 (CRYPTO) "Secure and
Efficient Asynchronous Broadcast Protocols" extends the
toolkit to atomic broadcast and validates the security-game
proofs in the random oracle model.

The CKS 2000 ABA is structurally simpler than Bracha 1987:

- *One coin per round* instead of two (bias plus progress).
- *Threshold-DSA or threshold-Schnorr* rather than the abstract
  common-coin oracle (CKS 2000 uses a specific
  threshold-coin-flipping scheme based on RSA).

The protocol is the practical descendant of Rabin 1983 +
Bracha 1987 and is the direct ancestor of HoneyBadger BFT
(2016, module 0070).

## System and threat model

- **Network.** Asynchronous, reliable channels.
- **Failures.** Byzantine, `f < n / 3`.
- **Cryptography.** Threshold signatures (typically threshold
  Schnorr or threshold-RSA in the original; modern deployments
  use threshold BLS).
- **Trusted setup.** A DKG produces shares of a common signing
  key.
- **Goal.** Async binary Byzantine agreement with `O(1)`
  expected rounds and provable security in the random oracle
  model.

## Theory

### Algorithm structure

CKS 2000 ABA in pseudo-code:

```
for r = 1, 2, 3, ...

  step 1 (vote):
    broadcast (Vote, r, p_i, partial-sig_i(r)) where p_i is
    current preference and partial-sig_i is the threshold-share
    for round r.
    Wait for n - f Vote messages.
    Combine partial-sigs into the round-r common coin.

  step 2 (aux):
    let bias := majority of received votes
    broadcast (Aux, r, bias)
    Wait for n - f Aux messages.

  step 3 (decide):
    if a single value v in 2f + 1 Aux messages:
      decide v
    else:
      p_{i+1} := if any v dominates -> v else -> coin(r)
```

### Theorem (CKS 2000 correctness)

The protocol satisfies validity, agreement, and termination
with probability 1 and expected `O(1)` rounds, against a
Byzantine adversary with `f < n / 3`, in the random oracle
model assuming threshold-cryptography security.

*Proof.* Cachin-Kursawe-Shoup 2000, Theorems 4.1 to 4.3. The
proof reduces termination to the unbiasability of the
threshold coin, which itself reduces to the underlying
threshold-signature scheme's unforgeability.

### Theorem (CKPS 2001 broadcast)

The CKPS 2001 paper extends the CKS 2000 ABA to a broadcast
primitive with `O(n)` amortised cost per delivery via batching
and threshold cryptography. The broadcast satisfies validity,
integrity, agreement, and total order under `f < n / 3`
Byzantine.

### Improvements over Bracha 1987

| Property            | Bracha 1987  | CKS 2000        |
| ------------------- | ------------ | --------------- |
| Number of coins     | 2 (bias + progress) | 1 (one)  |
| Coin construction   | abstract     | threshold-RSA   |
| Voting layer        | Bracha RB    | direct + sigs   |
| Termination bound   | `<= 4`       | `<= 4`          |
| Provable security   | abstract     | RO model + threshold-sig EUF-CMA |

## Practice

### HoneyBadger BFT lineage

HoneyBadger BFT (Miller-Xia-Croman-Shi-Song 2016, module 0070)
is the direct production descendant. It composes:

- *CKS-style ABA* for per-proposer agreement.
- *Bracha RB* for proposer dispersal.
- *Threshold encryption* for fairness and front-running
  resistance.
- *Threshold-BLS coin* (replacing CKS 2000's threshold-RSA).

Dumbo and Speeding Dumbo (modules 0073, 0074) refine
HoneyBadger by parallelising and amortising further.

### Production deployment status

- *Drand.* Implements the threshold-BLS common coin used by
  CKS-style protocols.
- *Filecoin.* Uses drand as part of its randomness.
- *Aleph.* DAG-based BFT building on similar primitives.
- *DFINITY's Internet Computer.* Uses threshold-cryptography
  with a CKS-style consensus layer.

## Formalisation aspects

```text
class CksAba (n f : Nat) (TS : ThresholdSig) (CC : CommonCoin) where
  protocol : ...
  correctness :
    forall (sched : AsyncSched),
      Validity protocol sched /\
      Agreement protocol sched /\
      Pr[Termination protocol sched] = 1 /\
      ExpectedRounds protocol sched <= 4
```

The formalisation requires modelling:

- The random oracle model (`Mathlib.Probability` plus an
  oracle abstraction).
- Threshold signature security (EUF-CMA against `f`-bounded
  adversaries).
- The protocol's combination of these into an ABA.

This is one of the harder formalisation targets in the course;
no existing Lean formalisation covers it.

## Verifiability and circuit encoding

**Tag: `friendly`.**

CKS 2000 with threshold-BLS coin maps cleanly to a SNARK
circuit. Per round:

- *Vote phase.* Verify `n - f` signatures, count by value,
  collect `n - f` partial-coin-shares. ~`200k` constraints in
  Schnorr-over-Pasta for `n = 100`.
- *Coin combination.* Lagrange interpolation on `t + 1`
  shares: `~700k` constraints.
- *Aux phase.* Same as Vote, `~200k`.
- *Decision rule.* Comparisons, ~100.

Total per round: `~1.1 * 10^6` constraints. Slightly less than
Bracha + separate coin combine because the coin shares are
piggybacked on Vote messages.

## Known attacks and limitations

- *Trusted setup.* The threshold-DKG must be run honestly. A
  corrupt setup compromises the coin and breaks termination.
- *Adaptive corruption.* Without proactive refresh, an
  adversary corrupting `f` validators after the DKG can extract
  the threshold key. Modern deployments use ephemeral keys or
  proactive secret sharing.
- *RO model dependence.* The proof uses random oracles. The
  protocol is conjectured secure in the standard model with
  appropriate hash functions, but the formal proof requires
  more work.

## Implementation notes

The crate provides a CKS-style ABA with the common coin
abstracted as a `CoinFn` (as in modules 0014-0015). The
critical structural difference from Bracha is the simpler
single-coin decision rule: when `S_r = {v}`, decide `v`
directly without comparing to the coin (the coin is used only
as a fallback for the empty/two-value cases).

Tests verify the validity case for `n = 4, f = 1`.

## References

- Cachin, Kursawe, Shoup, "Random Oracles in Constantinople:
  Practical Asynchronous Byzantine Agreement Using
  Cryptography", PODC 2000.
- Cachin, Kursawe, Petzold, Shoup, "Secure and Efficient
  Asynchronous Broadcast Protocols", CRYPTO 2001.
- Cachin, Kursawe, Lysyanskaya, Strobl, "Asynchronous
  Verifiable Secret Sharing and Proactive Cryptosystems",
  CCS 2002.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
