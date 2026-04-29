# 0014: Rabin 1983 Randomised Byzantine Agreement

## Historical context

Rabin's FOCS 1983 paper "Randomized Byzantine Generals" gave a
randomised protocol for Byzantine agreement that terminates in
*expected constant* number of rounds, dramatically improving on
Ben-Or's expected exponential local-coin variant. The price:
Rabin's protocol assumes a *common coin* (also called a *global
coin* or *shared coin*), an oracle that returns the same fair
random bit to every honest process per query.

The result is structural: by making the coin global rather than
local, the protocol's termination probability per round goes
from `2^{-(n - f)}` to `1/2`, and expected rounds drop to `O(1)`.
This is one of the most influential ideas in async BFT.

Implementing the common coin is the next question. The standard
constructions, all developed after Rabin's paper, use threshold
cryptography:

- *Threshold-BLS coin* (CKS 2000, Cachin-Kursawe-Shoup). `t + 1`
  validators sign a per-round nonce; the resulting threshold
  signature, when hashed, gives the coin.
- *VRF-based coin* (Goldberg-Naor-Reyzin 2017): each process
  computes a VRF on the round number; an agreed-upon function of
  the outputs (e.g. the minimum) is the coin.
- *VDF-based coin* (Boneh-Bonneau-Bunz-Fisch 2018): a verifiable
  delay function ensures unbiasability.

## System and threat model

- **Network.** Asynchronous.
- **Failures.** Byzantine, `f < n / 3`.
- **Common coin.** A trusted oracle returns a uniformly random
  bit per round, the same to every honest process.
- **Cryptography.** Authenticated channels (signatures).
- **Goal.** Binary Byzantine agreement, terminating in expected
  `O(1)` rounds.

## Theory

### Algorithm (Rabin 1983, simplified)

```
for r = 1, 2, 3, ...

  step 1 (proposal):
    broadcast (Propose, r, p_i) where p_i is current preference
    wait for n - f Propose messages from this round
    let majority(r) := the value present in > 2/3 of the messages,
                       or NULL if no such value

  step 2 (echo):
    if majority(r) = Some(v): broadcast (Echo, r, v)
    else: broadcast (Echo, r, NULL)
    wait for n - f Echo messages
    if some v has > 2/3 Echos: decide v

  step 3 (coin):
    coin := common-coin-for-round(r)
    if no decision: p_{i+1} := majority(r) if Some, else coin
```

### Theorem (Rabin correctness, with common coin)

For `f < n / 3` Byzantine and a common coin, the protocol
satisfies:

- *Validity.* If all honest inputs are `v`, every honest process
  decides `v` in `O(1)` rounds (in fact, in round 1).
- *Agreement.* No two honest processes decide differently.
- *Probabilistic termination.* Expected rounds = `O(1)`.

*Proof sketch.*

- *Validity.* All honest broadcast `Propose(v)`; `majority(r) =
  Some(v)` for every honest. Phase 2: all honest broadcast
  `Echo(v)`. Phase 2 collects > `2/3` Echos for `v`. Decide.
- *Agreement.* If some honest decides `v` in round `r`, then
  `> 2/3` Echos have value `v`. By quorum intersection in `n =
  3f + 1`, every honest process saw at least one Echo with `v`,
  setting their next-round preference to `v` regardless of
  coin. Inductively, no honest process can flip to `1 - v`.
- *Probabilistic termination.* In each round, with probability
  `1/2`, the common coin matches the value (if any) preferred by
  the honest *coordinator* of the round. When matched, the
  protocol terminates within two rounds. Expected rounds = 4
  (Cachin-Kursawe-Shoup 2000 give the tight constant).

QED (sketch).

### Comparison to Ben-Or

| Feature              | Ben-Or                 | Rabin                  |
| -------------------- | ---------------------- | ---------------------- |
| Failure model        | crash, f < n/2         | Byzantine, f < n/3     |
| Coin                 | local                  | global (common)        |
| Expected rounds      | `O(2^n)`               | `O(1)`                 |
| Crypto needed        | none                   | threshold or VRF       |
| Probabilistic term.  | yes                    | yes                    |

### Why the common coin breaks FLP

FLP rules out *deterministic* termination. The common coin is
*randomised*, but it is also *agreed-upon*: every honest process
sees the same outcome. This combination is what makes
termination almost-sure with constant expected rounds.

The proof of escape: in each round, the adversary has at most
half the schedules in which the coin matches the protocol's
needs. With a common coin, the protocol's correctness is
preserved over the *expected* schedule, not over every schedule.

## Practice

### Production examples

- *HoneyBadger BFT* (Miller 2016) implements Rabin-style ABA
  with a threshold-BLS coin per instance. The full BFT is
  built atop ABA.
- *Dumbo and Dumbo2* (Guo 2020, 2022) optimise HoneyBadger,
  reducing the common-coin overhead via batched ABA.
- *Drand* (Cloudflare's randomness beacon) provides a public
  threshold-BLS common coin every 30 seconds. Used by Filecoin,
  Internet Computer, and various L2 protocols for randomness.

### Common-coin DKG

The trusted-setup phase of a threshold-BLS coin requires a
*distributed key generation* (DKG). Standard DKG protocols:

- *Pedersen DKG* (Pedersen 1991).
- *Gennaro-Jarecki-Krawczyk-Rabin DKG* (GJKR 2007).
- *Resharing DKG* for proactive secret sharing (Wong-Wing-Wang).

DKGs are themselves multi-party protocols. They are usually
run once per epoch (e.g. once per validator-set change in a
production chain). The complexity of the DKG is part of the
Rabin-style approach's deployment cost.

## Formalisation aspects

```text
class CommonCoin (n : Nat) where
  query : Round -> Bool
  honest_agreement :
    forall (r : Round), forall i j : NodeId,
    IsHonest i -> IsHonest j ->
    process_view i (query r) = process_view j (query r)
  unbiased :
    forall (r : Round) (b : Bool),
    Pr[query r = b] = 1/2

theorem rabin_termination
    (n f : Nat) (h : 3 * f < n) (CC : CommonCoin n) :
    forall (sched : AsyncSchedule),
      Pr[round_to_termination Rabin <= 4] >= ... := by
  sorry
```

The formalisation requires probability theory (`MeasureTheory`)
plus a model of the common-coin oracle. cslib's
`InferenceSystem` could host the protocol's per-round rules; the
probabilistic argument lives in Mathlib's `Probability`.

## Verifiability and circuit encoding

**Tag: `friendly`.**

Rabin's protocol with a threshold-BLS coin is highly SNARK-
friendly. Per round in circuit:

- Phase 1 (Propose): verify `n - f` signatures, count by value.
  ~`200k` constraints in Schnorr-over-Pasta for `n = 100`.
- Phase 2 (Echo): same.
- Common coin: one threshold-BLS pairing check, `~10^6`
  constraints.
- Decision rule: comparisons, ~100 constraints.

Total: `~1.4 * 10^6` constraints per round. With recursion
across rounds, a multi-round termination proof is constant-size.

This is the structural form that HoneyBadger BFT's verifiable
variants (zk-HoneyBadger) take. Production zk-rollup sequencers
that adopt async ABA would follow this template.

## Known attacks and limitations

- *Trusted setup.* The threshold-BLS coin requires a DKG. If
  the DKG is compromised, the coin can be biased. Production
  uses observable, multi-party DKGs (drand) and proactive
  refresh.
- *Coin biasability under adaptive corruption.* If the
  adversary can corrupt validators *during* a round, it can
  hold back partial signatures and bias the coin. Mitigation:
  ephemeral keys, single-secret leader election, or proactive
  refresh.
- *DKG denial-of-service.* A faulty validator can refuse to
  participate in the DKG, halting the setup. Production DKGs
  use timeouts and exclude non-participants.

## Implementation notes

The crate provides a Rabin-style protocol using
`MockThresholdAccumulator` (from module 0011) as the common
coin. The flow:

- Each round: Propose, then Echo, then collect partial coin
  signatures, then combine to derive the coin bit.
- Tests: `n = 4, f = 1` with all-equal inputs (validity case)
  decide in round 1.

A mixed-input termination test relies on the seed; a property-
based test (Exercises) is the appropriate instrument.

## References

- Rabin, "Randomized Byzantine Generals", FOCS 1983.
- Cachin, Kursawe, Shoup, "Random Oracles in Constantinople:
  Practical Asynchronous Byzantine Agreement Using
  Cryptography", PODC 2000.
- Cachin, Kursawe, Petzold, Shoup, "Secure and Efficient
  Asynchronous Broadcast Protocols", CRYPTO 2001.
- Boneh, Boyen, "Short Signatures Without Random Oracles",
  Eurocrypt 2004.

See also [`HISTORY.md`](../HISTORY.md), section "1980 to 1985"
and "2000 to 2008".
