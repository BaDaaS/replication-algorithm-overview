# 0006: The Dolev-Strong Round-Complexity Lower Bound

## Historical context

Dolev and Strong published "Authenticated Algorithms for
Byzantine Agreement" in SICOMP 1983 [`dolev1983`]. The paper
contains both an `f + 1`-round upper bound (the canonical
signed-message Byzantine broadcast) and the matching lower bound:
*every* deterministic synchronous Byzantine agreement protocol
requires at least `f + 1` rounds, even with idealised
authentication.

The lower bound is the round-complexity counterpart of FLP. FLP
rules out *termination*; Dolev-Strong rules out *fast*
termination. Together they bracket what is achievable in
synchronous and asynchronous Byzantine settings.

The bound is tight: PSL/LSP's `SM(f)` and the namesake Dolev-
Strong broadcast both run in `f + 1` rounds. PBFT's three-phase
structure does not contradict this lower bound: PBFT runs in
*partial synchrony*, where the lower bound is on view-change
recovery, not on the per-view phase count.

## System and threat model

- **Network.** Synchronous, lockstep rounds.
- **Failures.** Up to `f` Byzantine processes among `n`.
- **Cryptography.** Idealised authentication: every signed
  message can be verified, signatures are unforgeable, the public
  keys of all processes are known a priori.
- **Goal.** Byzantine broadcast: a designated commander has a
  value `v`; every honest process must output `v`. Lower bound
  is on the number of rounds before every honest process commits.

## Theory

### Theorem (Dolev-Strong 1983 lower bound)

Any deterministic synchronous Byzantine broadcast protocol that
tolerates `f` faults requires `f + 1` rounds.

*Proof (sketch via adversary strategies).* Suppose, for
contradiction, a protocol `P` runs in `r <= f` rounds. We
construct two execution scenarios indistinguishable to some
honest process at the end of round `r`, in which `P` must produce
different outputs.

The adversary maintains, throughout rounds `1, ..., r`, a
"hidden" set of size at most `f`. In round `i`, the adversary
schedules one Byzantine process to *withhold* its message from a
specific honest target, while sending it normally to all others.
After `r <= f` rounds, the adversary has used at most `r <= f`
distinct Byzantine processes. The honest target sees a strictly
shorter message history than its peers.

Two scenarios:

- *Scenario A.* Commander broadcasts `0`. Adversary withholds
  per-round forwards toward the honest target so that, after
  round `r`, the target has not seen evidence sufficient to
  decide `0`.
- *Scenario B.* Commander is Byzantine and broadcasts `1` to
  some honest peers and `0` to others (specifically, to the
  honest target). Adversary mirrors A's withholding pattern but
  sources the differences from the actual `1`/`0` ambiguity.

Indistinguishability is the key: the honest target's view in A
and B is identical at the end of round `r`. So `P` must produce
the same output in both. But validity requires output `0` in A
(commander honest, value `0`) and at least one of `0` or `1` in
B (depending on the commander's actions). One of the two
constraints is violated: contradiction.

The construction is carried out in detail in Dolev-Strong 1983,
Section 3, and reproduced in Lynch's "Distributed Algorithms"
1996, Section 6.4. Pelc and Smid 1986 extended the bound to
crash-recovery and to authenticated message-passing variants.

QED (sketch).

### Theorem (matching upper bound)

The `SM(f)` protocol from module 0004 (LSP 1982) runs in `f + 1`
rounds.

The Dolev-Strong paper's namesake broadcast also runs in `f + 1`
rounds. It is structurally similar to `SM(f)` but uses a
signature-chain *threshold*: an honest process accepts a value as
"sealed" once it has received signature chains of length `i` from
`i` distinct co-signers in round `i`.

### Variations

- *Crash-failures.* The lower bound becomes `min(f + 1, t + 1)`
  where `t` is the number of crashes. With pure crash-stop, the
  Dolev-Strong argument applies directly; pure timing-failure
  (omission) admits `O(log f)`-round protocols (Garay-Moses
  1998).
- *Randomised protocols.* Feldman-Micali 1988 gave a randomised
  protocol that terminates in `O(1)` expected rounds, breaking
  the deterministic bound. Module 0015 develops this.
- *Authenticated unbounded crypto.* Dolev-Strong assumes
  idealised authentication, which is essentially as strong as
  authenticated channels can be. Real signature schemes provide
  this only computationally; the resulting lower bound applies
  with overwhelming probability against a polynomial-time
  adversary.

## Practice

### What `f + 1` means in production

PBFT, Tendermint, HotStuff, and HotStuff-2 all complete a *single
view's* commit in `O(1)` rounds (3 phases for PBFT/Tendermint, 4
chained for HotStuff, 2 for HotStuff-2). They do not contradict
Dolev-Strong because the *view-change recovery* path is what
inherits the lower bound: in the worst case, `f + 1` view changes
may be needed before the protocol reaches a stable view.

Production timeouts:

- *PBFT* doubles the view-change timeout each time, expecting at
  most `f + 1` such doublings before stabilisation. The expected
  total recovery time is `O(2^f * D)` in the worst case.
- *HotStuff-2* refines this to expected `O(D)` recovery once the
  network is stable, by using the QC chain's stability rather
  than per-view timeouts.

### Communication complexity

Dolev-Strong's bound is on *rounds*. The communication-complexity
lower bound is `Omega(n * f)` per Dolev-Reischuk 1985 (module
0007). Modern protocols match the round bound but improve on the
naive `O(n^{f+1})` message count by using signature aggregation
(BLS quorum certificates) and gossip-based dissemination.

## Formalisation aspects

### Pseudo-Lean structure

```text
def IsByzantineBroadcastProtocol
    (P : SyncProtocol) (n f : Nat) : Prop :=
  forall (input : Value),
  forall (sched : SyncSchedule)
         (corrupt : Set (Fin n))
         (h_corrupt : corrupt.card <= f),
    Agreement P input sched corrupt /\
    Validity P input sched corrupt /\
    TerminatesIn P (f + 1) input sched corrupt

theorem dolev_strong_lower_bound
    (n f : Nat) (h_nf : n > f) (h_pos : f >= 1) :
    forall (P : SyncProtocol),
      IsByzantineBroadcastProtocol P n f ->
      forall (r : Nat) (h_r : r < f + 1),
        not (TerminatesIn P r input sched corrupt) := by
  sorry  -- adversary strategy construction
```

### Proof structure

The Dolev-Strong argument is naturally an *adversary strategy
game*. cslib's `LTS` framework can be augmented with an adversary
turn (per round, the adversary picks which messages to withhold).
The proof becomes a strategy stealing-style argument:
indistinguishable plays force the protocol to an inconsistent
output. Lean's coinductive game machinery (currently in early
development in cslib) may suffice. Alternatively, Mathlib's
`PFun` / `Game` infrastructure could be reused.

The Dolev-Strong upper bound (the namesake broadcast protocol)
is more straightforward to formalise: it is a structural
induction on the round count and a count of received signature
chains. Velisarios's Coq formalisation of PBFT contains a
related but distinct construction.

## Verifiability and circuit encoding

**Tag: `na`** for the lower bound; **`friendly`** for the matching
upper-bound protocol.

The lower bound is a non-existence statement: there is no
protocol below `f + 1` rounds, so there is nothing to encode.

The matching upper-bound protocol (Dolev-Strong broadcast,
`SM(f)`) is the natural template for a SNARK-friendly Byzantine
broadcast. Each round contributes signature-chain checks; with
Schnorr-over-Pasta the per-round constraint count is on the
order of `(f + 1) * 3k = O(f * 3k)` constraints. Aggregated BLS
signatures cut this to a single pairing check per round. The
chain commitment is a vector of signatures of constant size per
round.

Production reference: Mina's Pickles uses a Praos-style chain
proof that is structurally similar to a Dolev-Strong chain. The
verifier checks `f + 1` signature steps recursively; recursion
gives constant-size proofs across many slots.

## Known attacks and limitations

- *Timing assumptions.* The bound is for synchronous protocols.
  Partial-synchrony BFT (PBFT et al.) inherits a different
  per-view bound but `f + 1` view-changes in the worst case.
- *Authenticated vs unauthenticated.* The lower bound holds even
  with idealised authentication. Without authentication, the
  bound is the same; PSL's `n > 3f` constraint kicks in
  earlier.
- *Crypto-implementable bound.* Real signatures are
  computationally unforgeable; the bound holds with overwhelming
  probability against polynomial-time adversaries.

## Implementation notes

The crate provides a small synchronous round-counter Byzantine
broadcast (`SM(1)` from module 0004 with `n = 4, f = 1`) and a
test that:

- Confirms termination in 2 rounds (= `f + 1`) under a
  non-faulty schedule.
- Demonstrates that, if we shorten the protocol to 1 round
  (drop the second forwarding step), an adversary that withholds
  the commander's signed message from one honest target breaks
  agreement.

This is not a formal counterexample for Dolev-Strong; it is a
constructive illustration that the bound binds.

## References

- Dolev and Strong, "Authenticated Algorithms for Byzantine
  Agreement", SICOMP 1983. [`dolev1983`].
- Lamport, Shostak, Pease, "The Byzantine Generals Problem",
  TOPLAS 1982. [`lsp1982`].
- Lynch, "Distributed Algorithms", Morgan Kaufmann 1996,
  Section 6.4 (textbook proof).
- Pelc and Smid, "Variants of Byzantine Agreement",
  SIAM J. Discrete Math. 1986.
- Garay and Moses, "Fully Polynomial Byzantine Agreement for `n
  > 3t` Processors in `t + 1` Rounds", SIAM J. Comp. 1998.

See also [`HISTORY.md`](../HISTORY.md), section "1980 to 1985".
