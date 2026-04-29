# 0004: The Byzantine Generals Problem

## Historical context

Pease, Shostak, and Lamport began the formal study of agreement
under arbitrary faults in their 1980 JACM paper "Reaching
Agreement in the Presence of Faults" [`psl1980`]. They proved a
tight `n > 3f` lower bound for synchronous unauthenticated
agreement and gave matching upper-bound algorithms. Two years
later, Lamport, Shostak, and Pease published "The Byzantine
Generals Problem" [`lsp1982`], the canonical pedagogical
treatment, popularising the metaphor of Byzantine generals
besieging a city. The 1982 paper also introduced the *signed
messages* variant which lifts the resilience to `f < n` (any
number of faulty followers, given a single honest commander).

The Byzantine Generals problem remains the cleanest synchronous
benchmark for BFT protocols. PBFT (1999) and every streamlined
BFT descendant (HotStuff, Tendermint, DiemBFT, HotStuff-2) inherit
its `n > 3f` threshold and adapt it to partial synchrony.

## System and threat model

- **Network.** Synchronous (DLS Variant 0): every honest message
  is delivered within a known bound `D`. We coarsen to lockstep
  rounds: in round `r`, every honest process sends its outbound
  messages, all such messages are delivered, and round `r+1`
  begins.
- **Failures.** Up to `f` of the `n` processes are Byzantine.
- **Cryptography.** Two variants:
  - *Oral messages (OM).* No authentication. The receiver of a
    message cannot tell which process originated a forwarded
    rumour.
  - *Signed messages (SM).* Each process has a signing key whose
    public counterpart is known to all. Messages are signed; an
    honest process can verify any received signed message.

## Theory

### Definition (Byzantine agreement)

A distinguished *commander* `C` holds an input `v in V`. The
remaining `n - 1` *lieutenants* must agree on a value
`d in V` such that:

- *(IC1) Agreement.* All honest lieutenants choose the same `d`.
- *(IC2) Validity.* If `C` is honest and inputs `v`, then `d = v`.

### Theorem 1 (PSL 1980 lower bound)

If `n <= 3f`, no protocol satisfies (IC1) and (IC2) for arbitrary
inputs.

*Proof (sketch).* For `f = 1, n = 3`, two scenarios are
indistinguishable to honest processes (the ones in which the
identity of the Byzantine differs). One scenario forces decision
`v`, the other forces decision `not v`. The honest lieutenant's
view is identical in both, so it must decide the same value, but
the protocol's correctness conditions force opposite choices.
Generalising: any `n <= 3f` instance reduces to the three-process
case by treating `n - 3` processes as silent. See LSP 1982,
Theorem 4.5. QED.

### Theorem 2 (PSL 1980 upper bound, oral messages)

For `n > 3f`, there exists a synchronous oral-message Byzantine
agreement protocol `OM(f)` with the following recursive
structure:

```
algorithm OM(0):
  C sends its value v to every lieutenant.
  Each lieutenant decides the value it received from C.

algorithm OM(m), m > 0:
  step 1.
    C sends its value v to every lieutenant.
  step 2.
    Each lieutenant L_i, having received value v_i from C, acts
    as the commander in a recursive call OM(m-1) to send v_i to
    the n - 2 other lieutenants.
  step 3.
    For each L_i and each value v_j received in step 2 from L_j,
    L_i records v_j. After all recursive calls return, L_i
    decides the *majority* of (v_i and the recorded v_j's), or a
    default if no majority exists.
```

*Correctness.* By induction on `m`. Base case `m = 0` is trivial
when there are no Byzantine lieutenants. Inductive step: assuming
`OM(m-1)` is correct for any system with at most `m-1` faults,
one shows `OM(m)` is correct for systems with at most `m` faults
and `n > 3m`. The non-trivial bookkeeping is in the majority
step; LSP 1982 Theorem 4.1 carries it out.

*Complexity.* `OM(m)` runs in `m + 1` rounds and exchanges
`O(n^{m+1})` messages. The exponential blow-up is the cost of the
recursion.

### Theorem 3 (LSP 1982 upper bound, signed messages)

With unforgeable signatures, the *signed-messages* protocol
`SM(m)` solves Byzantine agreement for any `n > f + 1` (i.e.
arbitrary fraction of faulty followers, provided at least one
honest follower exists in addition to the commander).

```
algorithm SM(m):
  step 1.
    C signs v and sends (v, sigma_C(v)) to each lieutenant.
  step 2.
    Each lieutenant maintains a set V of received values. On
    receiving (v, sigma_C(v) :: sigma_L1(...) :: ... :: sigma_Lk(...)),
    if all signatures verify and L_i's signature is not in the
    chain, L_i adds v to V and forwards
    (v, sigma_C(v) :: ... :: sigma_Lk(v) :: sigma_Li(v)).
  step 3.
    After f + 1 rounds, each L_i decides the unique value in V
    if |V| = 1, or a default otherwise.
```

*Correctness.* If `C` is honest, all honest lieutenants receive
exactly `(v, sigma_C(v))` and `V = {v}`; they decide `v`. If `C`
is Byzantine, any value with a valid signature chain of length
`f + 1` must contain a signature from at least one honest
lieutenant, who then forwards the chain to every other honest
lieutenant in time for round `f + 1`. So all honest lieutenants
have the same `V`. (LSP 1982 Theorem 4.6.)

*Complexity.* `f + 1` rounds; `O(n^2 * f)` signatures total in
the worst case. Each lieutenant signs at most once per value per
forward, and the chain length is bounded by `f + 1`.

### Round complexity is tight

The Dolev-Strong 1983 lower bound (module 0006) shows that any
deterministic synchronous Byzantine agreement protocol requires
at least `f + 1` rounds. `SM(f)` matches this bound.

## Practice

### From the Byzantine Generals to PBFT and HotStuff

The `n > 3f` threshold survives every move from synchrony to
partial synchrony in the BFT family:

- *PBFT 1999.* `n > 3f` under partial synchrony. Castro-Liskov
  use authenticated channels (MAC tags, then signatures in later
  variants) and a three-phase exchange (pre-prepare, prepare,
  commit) to replace the recursive OM protocol's exponential
  cost with quadratic.
- *Tendermint and HotStuff.* `n > 3f` partial synchrony. The
  authenticated structure becomes a quorum certificate (QC):
  `2f + 1` signatures aggregate the `prepare` and `commit`
  decisions.
- *Linear authenticator complexity.* HotStuff's chained variant
  uses BLS signature aggregation to compress a QC into a single
  signature. The `n > 3f` threshold remains the safety boundary.

### Authenticated vs unauthenticated in production

All production BFT systems (CometBFT, Aptos, Sui, Aleo, Diem)
use *authenticated* messages because signatures are essentially
free given modern hardware. The `n > 3f` threshold is preferred
over `n > f + 1` because it gives a quadratic message
complexity per view (with QCs) rather than exponential signed-
chain length.

### "Byzantine Generals" as folklore

The term has bled into general engineering usage to refer to any
distributed agreement under arbitrary failures. Lamport himself
documents the choice of metaphor in his memoir; it was selected
because the Soviet (USSR) and Albanian generals were the only
naming options in 1981 that did not offend an active state actor.

## Formalisation aspects

### Pseudo-Lean signatures

```text
structure ByzantineProblem where
  n        : Nat
  f        : Nat
  V        : Type
  -- each process has an identity in [0, n)
  Process  : Type := Fin n
  is_honest: Process -> Bool

structure ByzantineProtocol (BP : ByzantineProblem) where
  state    : BP.Process -> Type
  init     : forall p, state p
  step_round : forall p, state p -> Round -> Inbox -> state p * Outbox
  decide   : forall p, state p -> Option BP.V

theorem psl_lower_bound (BP : ByzantineProblem) :
    BP.n <= 3 * BP.f ->
    forall (P : ByzantineProtocol BP),
      not (forall (sched : Schedule), AgreesAndIsValid P sched) := by
  sorry  -- bivalence-by-extension argument
```

### Reuse of cslib's `LTS` and `InferenceSystem`

A round-by-round Byzantine protocol is a labelled transition
system whose labels are *round transitions* (collections of
in-round messages). cslib's `LTS` directly captures this. The
recursive structure of `OM(m)` is a *fixed-point definition* over
the natural-number index; it fits Mathlib's `Nat.rec` and is a
promising target for a structural induction proof of correctness.

The signed-message protocol `SM(m)` requires modelling
*signature chains*. The natural representation is a list of
`(value, signature, signer)` triples with verification predicates
inductive on chain length. Mathlib's `List.Sublist` and a custom
`SignedChain` typeclass would suffice.

## Verifiability and circuit encoding

**Tag: `friendly`.**

The signed-messages variant is, after a SNARK-friendly signature
substitution, encodable in a circuit. The two ingredients are:

- A SNARK-friendly signature scheme (Schnorr over Pasta in Mina;
  BLS over BLS12-381 in Ethereum). The circuit verifies a chain
  of `f + 1` signatures over a value.
- A SNARK-friendly hash for transmitting the value's commitment
  through the chain (Poseidon-2, Rescue-Prime).

The OM variant (no authentication) is *not* circuit-friendly:
without signatures, the verifier cannot distinguish a valid chain
of forwarded values from a fabricated one, so there is nothing to
check in circuit. Signed BFT is thus the canonical
SNARK-friendly Byzantine agreement.

A practical example: Mina's *consensus snapshot* in Pickles
includes a verifiable check of the (Praos-style) leader's
signature chain. The same template applies to a Byzantine-Generals
encoding: the circuit verifies that `f + 1` distinct validators'
signatures attest the same value and that the chain's prefix
relations are consistent. Cost: `f + 1` Schnorr-over-Pasta
verifications, ~3k constraints each (Mina Pickles), ~`(f+1) * 3k`
total constraints for a one-shot proof.

## Known attacks and limitations

- *Adaptive corruption against signed messages.* If signatures are
  long-lived, an adaptive adversary corrupting a process at time
  `t` learns its private key and can forge backdated signatures.
  Production protocols use ephemeral keys (Algorand) or
  proactive secret sharing (Ouroboros) to bound the adaptive
  window.
- *Unauthenticated OM in partial synchrony.* OM is correct in
  synchrony, but the round-based structure does not survive
  partial synchrony directly: a delayed message can move from one
  round into the next, breaking the majority count. PBFT's
  three-phase rewrite and HotStuff's view-change machinery are
  the partial-synchrony analogues.
- *Dependence on a known commander.* The Byzantine-Generals
  formulation has a distinguished commander; SMR-style consensus
  with rotating leaders inherits the threshold but has additional
  liveness concerns (leader bias, view changes, leader rotation
  fairness).

## Implementation notes

The crate provides a synchronous round-based simulation of `OM(1)`
with `n = 4, f = 1`. The default scenario shows that with one
Byzantine commander sending `Attack` to two lieutenants and
`Retreat` to one, all three honest lieutenants agree on `Attack`
(majority of received reports). A second test exhibits the
failure of OM(0) on the same input distribution.

`SM(1)` is sketched in the test module as an extension exercise
(see `Exercises.md`).

## References

- Pease, Shostak, Lamport, "Reaching Agreement in the Presence of
  Faults", JACM 1980. [`psl1980`].
- Lamport, Shostak, Pease, "The Byzantine Generals Problem",
  TOPLAS 1982. [`lsp1982`].
- Dolev and Strong, "Authenticated Algorithms for Byzantine
  Agreement", SICOMP 1983. [`dolev1983`].

See also [`HISTORY.md`](../HISTORY.md), section "1980 to 1985".
