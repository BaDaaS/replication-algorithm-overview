# 0015: Bracha 1984/1987 Asynchronous Byzantine Agreement

## Historical context

Bracha's 1984 STOC paper "An Asynchronous `[(n - 1) / 3]`-
Resilient Consensus Protocol" gave the first deterministic
*reduction* of asynchronous binary agreement (ABA) to a *common
coin*, plus a local-randomness protocol for the no-coin case.
The 1987 journal version "Asynchronous Byzantine Agreement
Protocols" (Information and Computation) consolidates the
theory and presents the *binary agreement* layer that, paired
with reliable broadcast (module 0009), gives a complete
asynchronous BFT primitive at `f < n / 3`.

The Bracha ABA protocol differs from Rabin's (module 0014) in
three ways:

- *Local coin variant.* Bracha's protocol can use local coins
  (like Ben-Or) at the cost of expected exponential rounds in
  the original 1984 form. The 1987 version with common coin
  achieves `O(1)` expected rounds.
- *Reduction to RB.* Each round's votes are exchanged via
  Bracha RB (module 0009), guaranteeing that all honest
  processes see the same set of votes per round. This is the
  *certified* communication layer.
- *Two coins per round.* Bracha 1987 distinguishes between a
  *bias coin* (decides whether to bias toward `0` or `1`) and a
  *progress coin* (used as a fallback if no bias emerges). The
  two-coin design is later simplified by Cachin-Kursawe-Shoup
  2000 (module 0016).

## System and threat model

- **Network.** Asynchronous, reliable channels.
- **Failures.** Byzantine, `f < n / 3`.
- **Common coin.** A trusted oracle returns a common bit per
  round.
- **Cryptography.** Authenticated channels (signatures); a
  threshold-cryptography common coin (Bracha 1987 modulo
  later constructions).
- **Goal.** Binary Byzantine agreement, expected `O(1)` rounds.

## Theory

### Algorithm structure

```
for r = 1, 2, 3, ...

  step 1 (vote exchange via RB):
    RB-broadcast (Vote, r, p_i) where p_i is current preference.
    Wait for n - f Vote(r) RB-deliveries.
    Let majority(r) := the value with > 2/3 of received votes,
                       or NULL.

  step 2 (auxiliary set via RB):
    RB-broadcast (Aux, r, majority(r)).
    Wait for n - f Aux(r) RB-deliveries.
    Let S_r := { v : v != NULL and v appears in some Aux(r) message
                       and v has been seen as a majority }.

  step 3 (decide or update):
    coin := common-coin-for-round(r)
    if S_r = {v}: p_{i+1} := v
                  if v = coin: decide v
    else if S_r = {0, 1}: p_{i+1} := coin
    else (S_r empty): p_{i+1} := coin
```

The use of RB ensures that every honest process sees the same
set of Vote and Aux messages, eliminating the equivocation
attacks that would otherwise plague async voting.

### Theorem (Bracha 1987 correctness, with common coin)

For `f < n / 3` and a common coin, the protocol satisfies
validity, agreement, and termination with probability 1 in
`O(1)` expected rounds.

*Proof sketch.* The proof structure is similar to Rabin's
(module 0014), with three additional ingredients:

- *RB ensures view agreement.* Every honest process sees the
  same set of Votes and Auxes per round, so `majority(r)` and
  `S_r` are common across honest processes.
- *S_r constraints.* The auxiliary set rule guarantees that
  `S_r ⊆ {0, 1}` and that `S_r` contains a value `v` only if
  `v` was in the majority for some honest process. This rules
  out adversarial values.
- *Coin-matched termination.* When `S_r = {v}` and the coin
  equals `v`, all honest processes decide; this happens with
  probability `1/2`.

Cachin-Kursawe-Shoup 2000 (module 0016) tighten the constants
and give a simpler one-coin variant.

### Local-coin variant (Bracha 1984)

Without a common coin, replace `coin` with `local-coin()` at
each process. The protocol still terminates with probability 1
but expected rounds are `O(2^n)`, like Ben-Or. The local-coin
variant is pedagogical; production uses the common coin.

### Comparison

| Variant         | Coin         | Expected rounds | Failure model |
| --------------- | ------------ | --------------- | ------------- |
| Bracha 1984     | local        | `O(2^n)`        | f < n/3 Byz   |
| Bracha 1987     | common       | `O(1)`          | f < n/3 Byz   |
| Rabin 1983      | common       | `O(1)`          | f < n/3 Byz   |
| Ben-Or 1983     | local        | `O(2^n)`        | f < n/2 crash |
| CKS 2000        | common       | `O(1)` (1 coin)| f < n/3 Byz   |
| MMR 2014/2015   | common (sig-free) | `O(1)`     | f < n/3 Byz   |

## Practice

### Where Bracha ABA shows up

- *HoneyBadger BFT* (Miller 2016, module 0070) uses an ABA
  primitive structurally identical to Bracha 1987 with a CKS-
  style threshold-BLS common coin.
- *Dumbo and Speeding Dumbo* (module 0073, 0074) use parallel
  Bracha ABAs for their batch agreement.
- *Aleph* (module 0030) builds a DAG-based BFT on top of
  Bracha-style RB plus ABA.

### Implementation patterns

- *Pre-aggregation.* Production deploys the Bracha RB layer
  separately from the ABA layer, so RB messages can be batched
  and aggregated. HoneyBadger's RB layer uses AVID
  (asynchronous verifiable information dispersal).
- *Pipelining.* Multiple ABA instances run concurrently
  (HoneyBadger has `n` parallel ABAs per round, one per
  proposer). Pipelining amortises the threshold-BLS coin cost.
- *Coin precomputation.* The threshold-BLS coin can be
  precomputed for upcoming rounds so the per-round latency does
  not include a coin combination phase.

## Formalisation aspects

```text
class AsyncByzAgreement (n f : Nat) (M : Type) where
  validity   : forall (input : Vec M n) (sched : AsyncSched),
               (forall i, IsHonest i -> input i = v) ->
               forall i, IsHonest i -> Eventually (Decide i v)
  agreement  : forall (sched : AsyncSched) (i j : Fin n),
               IsHonest i -> IsHonest j ->
               Decide i v_i -> Decide j v_j -> v_i = v_j
  termination :
    forall (sched : AsyncSched),
    Pr[forall i, IsHonest i -> Eventually (Decide i)] = 1
```

Reuse: the `ReliableBroadcast` typeclass from module 0009 is
the layer below. cslib's `InferenceSystem` can express the
per-round inference rules.

## Verifiability and circuit encoding

**Tag: `friendly`.**

Bracha ABA inherits the verifiability profile of Rabin ABA
(module 0014). Per round (with threshold-BLS coin):

- RB Vote phase: `n - f` RB deliveries; each is a witness of
  `2f + 1` Bracha ready-set certificates. Aggregating, ~`200k`
  constraints in Schnorr-over-Pasta for `n = 100`.
- RB Aux phase: same.
- Coin: threshold-BLS pairing check, ~`10^6`.
- Decision rule: comparisons, negligible.

Total per round: `~1.4 * 10^6` constraints; constant proof
size with recursion. The aggregation of RB witnesses with the
ABA structure is the structural form of zk-HoneyBadger.

## Known attacks and limitations

- *Coin biasability.* Same as Rabin: adaptive corruption can
  bias the coin if the threshold cryptography is not
  proactively refreshed.
- *Local-coin degeneracy.* The 1984 local-coin variant is
  exponential. Use the 1987 common-coin form in production.
- *RB layer cost.* The Bracha RB exchange adds `O(n^2)`
  messages per round. Production amortises via batching.

## Implementation notes

The crate provides a Bracha-style ABA over the simulator,
using `MockThresholdAccumulator` (module 0011) for the common
coin. The protocol assumes Bracha RB at the layer below; we
abstract it with a "deliver Vote/Aux to all honest" primitive
implemented via the simulator's broadcast.

Tests:

- *Validity.* All-equal inputs decide in round 1.
- *Common-coin termination.* With a deterministic coin,
  termination in round 2 for one example mixed input.

## References

- Bracha, "An Asynchronous `[(n - 1) / 3]`-Resilient Consensus
  Protocol", PODC 1984.
- Bracha, "Asynchronous Byzantine Agreement Protocols",
  Information and Computation 1987.
- Bracha and Toueg, "Asynchronous Consensus and Broadcast
  Protocols", JACM 1985.
- Cachin, Kursawe, Shoup, "Random Oracles in Constantinople",
  PODC 2000.

See also [`HISTORY.md`](../HISTORY.md), section "1980 to 1985"
and "1986 to 1999".
