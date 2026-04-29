# 0013: Ben-Or 1983 Randomised Async Consensus

## Historical context

Ben-Or's PODC 1983 paper "Another Advantage of Free Choice:
Completely Asynchronous Agreement Protocols" introduced the
first *randomised* protocol that solves binary consensus in the
asynchronous crash-stop model with `f < n / 2` faults. By
adding local coin flips, Ben-Or escaped the FLP impossibility
(deterministic) at the cost of *expected exponential* rounds.

Rabin 1983 published an `O(1)`-round randomised Byzantine
agreement assuming a *common coin* (a shared random source);
Ben-Or's protocol uses only local coins and trades expected
running time for the simpler assumption.

The paper is a milestone: it shows that local randomness alone
suffices to solve async consensus, decoupling the impossibility
from the *deterministic* qualifier of FLP.

## System and threat model

- **Network.** Asynchronous, reliable channels, no upper bound
  on delay.
- **Failures.** Crash-stop with `f < n / 2`.
- **Inputs.** Each process has an input bit `b_i in {0, 1}`.
- **Local randomness.** Each process has access to a private
  fair coin.
- **Goal.** Binary consensus: agreement, validity, termination
  with probability 1.

## Theory

### Algorithm

```
for r = 1, 2, 3, ...

  step 1 (proposal phase):
    broadcast (Phase1, r, p_i)  where p_i is current preference
    wait for n - f Phase1 messages from this round
    if some value v appeared in >= n/2 + 1 Phase1 messages:
      bias := Some(v)
    else:
      bias := None

  step 2 (decision phase):
    if bias = Some(v):
      broadcast (Phase2, r, v)
    else:
      broadcast (Phase2, r, NULL)
    wait for n - f Phase2 messages from this round
    if some value v appeared in >= f + 1 Phase2 messages:
      decide v
    if some value v appeared in >= 1 Phase2 message:
      p_{i+1} := v
    else:
      p_{i+1} := flip a fair coin
```

### Theorem (Ben-Or correctness)

The algorithm satisfies, for `f < n / 2`:

- *Validity.* If all honest processes start with the same input
  `v`, every honest process decides `v` in round 1.
- *Agreement.* No two honest processes decide differently.
- *Termination with probability 1.* Every honest process
  eventually decides.

*Proof sketch.*

- *Validity.* If all honest input is `v`, in round 1 every
  honest process sends `(Phase1, 1, v)`. The honest count `>= n
  - f > n / 2`, so `bias = Some(v)` for every honest. In step 2
  every honest broadcasts `(Phase2, 1, v)` and decides on
  `>= f + 1` `v`-votes (`n - f > f + 1` for `f < n / 2`).
- *Agreement.* If process `i` decides `v` in round `r`, then
  `>= f + 1` Phase2 messages with value `v` were received in
  round `r`. Of these, `>= 1` is from an honest process. By
  step-1's `bias` rule, `>= n/2 + 1` Phase1 messages with `v`
  were seen, ruling out a Phase1 majority for the other value.
  So no honest process can compute `bias = Some(1 - v)` in
  round `r`, hence no honest process decides `1 - v` in round
  `r`. Inductively, this propagates to subsequent rounds via
  the preference-update rule.
- *Termination with probability 1.* In each round, with
  probability `>= 2^(-n)`, all `f + 1` honest coin flips agree
  on the same bit, leading every honest process to decide in
  the next round. So termination occurs in expected time
  `O(2^n)`.

QED (sketch). Ben-Or 1983 gives the full proof.

### Optimality and improvements

- *Round complexity.* Expected `O(2^n)` rounds is exponential in
  the number of processes. Rabin 1983 reduces this to `O(1)` by
  introducing a *common coin*, at the cost of needing a
  cryptographic primitive.
- *Aspnes 1998* gives a survey of randomised consensus
  variants achieving polynomial expected rounds without a
  common coin (using local randomness more cleverly).
- *Bracha 1984* (module 0014) extends the technique to
  Byzantine faults.

### Why the protocol escapes FLP

FLP rules out *deterministic* termination. Ben-Or terminates
with probability 1, not deterministically. There exist
schedules in which the protocol runs forever (the same
schedules that FLP constructs), but they have probability 0
under the protocol's local-randomness distribution.

This is the essential message: randomness changes the modality
of termination from deterministic to almost-sure.

## Practice

### Where Ben-Or-style protocols are used

- *HoneyBadger BFT* (Miller 2016, module 0070) uses a
  Bracha-style ABA (Byzantine generalisation of Ben-Or) with a
  cryptographic common coin to achieve `O(1)`-round agreement
  in async Byzantine.
- *Dumbo* and follow-ups optimise HoneyBadger's ABA further.
- *Ouroboros's input endorsement protocol* uses a similar
  randomised quorum mechanism for slot leader selection.

Modern production protocols prefer common-coin variants over
pure-local-randomness Ben-Or; the pedagogical value of Ben-Or
is to illustrate the local-randomness escape from FLP.

### Common-coin tradeoff

Ben-Or pays exponential rounds for protocol simplicity. A
common-coin variant uses one threshold-BLS signature per round
to give every process the same coin output, reducing rounds to
`O(1)`. The common coin requires:

- A trusted setup (DKG) to distribute shares.
- One threshold BLS combine per round.

In practice this is acceptable, and modern async BFT (Module
0070) all use common coins.

## Formalisation aspects

```text
structure BenOrState where
  round       : Nat
  preference  : Bool
  decided     : Option Bool
  phase1_msgs : List (Round, NodeId, Bool)
  phase2_msgs : List (Round, NodeId, Option Bool)

theorem ben_or_correctness :
    forall (n f : Nat) (h : 2 * f < n),
    forall (sched : AsyncSchedule),
      Validity ben_or sched /\
      Agreement ben_or sched /\
      Pr[Termination ben_or sched] = 1 := by
  -- Inductive proof of agreement; probabilistic argument for
  -- termination using the coin-agreement lower bound.
  sorry
```

The probabilistic-termination argument requires Mathlib's
`Probability.Martingale` infrastructure and the
`Filter.Eventually.atTop` modality. This is one of the more
demanding formalisation targets in the course.

## Verifiability and circuit encoding

**Tag: `partial`.**

The deterministic part of Ben-Or (Phase 1 collection, Phase 2
collection, decision rule) is SNARK-friendly: the verifier
checks that `n - f` signed messages of each phase exist. The
*coin flip* is local randomness, not naturally circuit-
encodable.

A verifiable variant: replace the local coin with a
*verifiable coin* (threshold-BLS over the round number or a VRF
output). This becomes Rabin 1983 with cryptographic
randomness. Mina, Aleo, and zk-rollup designs that adopt
async BFT use the verifiable-coin form.

The circuit cost per round (with threshold-BLS coin):

- Phase 1: verify `n - f` signatures and count by value: ~`(n -
  f) * 3k` constraints in Schnorr-over-Pasta.
- Phase 2: same.
- Coin: one threshold-BLS verification, ~`10^6` constraints.

Total per round: `~10^6 + 6kn` constraints. For `n = 100, f =
33`: `~1.5 * 10^6`.

## Known attacks and limitations

- *Expected exponential rounds.* The local-coin protocol is
  pedagogical, not production-ready. Use common-coin variants in
  practice.
- *Local-coin distribution attacks.* If the local coin is
  biased (e.g. an attacker knows or influences the coin), the
  termination guarantee weakens. The protocol assumes a *fair*
  coin; production must use a cryptographically secure RNG.

## Implementation notes

The crate provides a synchronous-friendly Ben-Or implementation:

- `BenOrNode { input, n, f }`.
- Each process broadcasts Phase1, then Phase2, then advances.
- The simulator drives rounds via a `Round` message.
- A test runs `n = 4, f = 1` with all-equal inputs and checks
  termination in round 1 (validity case).

The general termination test (with mixed inputs) is omitted
because the simulator's deterministic seed makes
"probabilistic termination" a single sample; a property-based
test with `proptest` would be the right instrument and is
mentioned in `Exercises.md`.

## References

- Ben-Or, "Another Advantage of Free Choice: Completely
  Asynchronous Agreement Protocols", PODC 1983.
- Rabin, "Randomized Byzantine Generals", FOCS 1983.
- Bracha, "Asynchronous Byzantine Agreement Protocols",
  Information and Computation 1987.
- Aspnes, "Randomized Protocols for Asynchronous Consensus",
  Distributed Computing 2003.

See also [`HISTORY.md`](../HISTORY.md), section "1980 to 1985".
