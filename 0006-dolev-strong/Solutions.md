# Module 0006 Solutions

## Solution 1 [T]: full adversary strategy at r = 1

We construct two schedules `S_0` and `S_1` indistinguishable to
some honest follower `L_3` after one round, in which a
deterministic protocol must produce different outputs.

*Setup.* `n = 4`, processes `c, L_1, L_2, L_3`. `f = 1`. Inputs
are 0 or 1 at the commander; the commander broadcasts the input.

*Schedule S_0:* commander honest, value 0. The commander
broadcasts `0` to all three followers. The Byzantine is `L_2`,
who in round 1 forwards nothing to anyone. Every honest
follower sees `(0, [c])`.

*Schedule S_1:* commander Byzantine. The commander broadcasts
`1` to `L_1, L_2` and `0` to `L_3`. The Byzantine is `c`. In
round 1, `L_2` is honest and would normally forward `1` to
`L_3`, but the adversary's withhold strategy drops this
forward (Byzantine `c` is silent in round 1; the withhold is
realised by `c` not signing the cross-validate). `L_3`'s view
in `S_1`: `(0, [c])`. Identical to `S_0`.

A deterministic protocol's output for `L_3` is a function of its
view at the end of round `r = 1`. So output in `S_0` equals
output in `S_1`:

- *Validity in `S_0`:* commander honest with value 0, so output
  must be 0.
- *Validity in `S_1`:* commander Byzantine, so any output is
  acceptable. *But agreement:* `L_1, L_2` saw `1` from `c`
  (chain `(1, [c])`) plus a missing forward; their decision is
  some function of `(1, [c])`, and standard tie-breaking would
  decide `1`. `L_3` decides `0` by its view; `L_1, L_2` decide
  `1`. Agreement fails.

In one of the two schedules the protocol is forced to violate
its specification, so no protocol with `r = 1` exists. The
construction generalises to `r = f`.

## Solution 2 [T]: SM(f) tightness

`SM(f)` from module 0004:

- Round 0: commander signs and sends.
- Round `i in 1..=f`: every lieutenant who has just received a
  new value signs and forwards it.
- Round `f + 1`: lieutenants apply the decision rule.

Total rounds: `f + 1`. Matches Dolev-Strong's lower bound
exactly.

The proof of correctness shows that any value with a chain of
length `f + 1` must include at least one honest signer (since
there are at most `f` Byzantine signers and they cannot
duplicate signatures). The honest signer's forward propagates
the value to every honest peer in time for round `f + 1`.

## Solution 3 [T]: crash-only Dolev-Strong

The same adversary-strategy game applies, with the modification
that the adversary now controls *crash timing* rather than
arbitrary behaviour. The argument:

- Round `i`: the adversary crashes one specific process between
  the moment it sends to one honest target and the moment it
  sends to another. The honest target who did not receive sees
  a strictly shorter view than its peers.
- After `f` rounds, the adversary has used `f` crashes. The
  inductive view-divergence argument yields the same
  contradiction as in the Byzantine case.

Authentication does not help in the crash-only setting because
crashes do not equivocate; signing buys no extra power.
Reference: Aguilera, Toueg 1996, "Failure Detection and Consensus
in the Crash-Recovery Model".

## Solution 4 [P]: rounds = 2, 3, 4

Increasing `rounds` adds more forwarding rounds but does not
change the eventual decision. With `rounds = 2`, each honest
follower's `seen` set converges to the union of all values the
Byzantine commander told any honest follower; the tie-break
decides the conservative `false`. Higher `rounds` give
redundancy (each honest follower hears the union via multiple
paths), reducing the probability that an honest follower misses
a value due to a single dropped forward, but the protocol's
decision rule is unchanged.

## Solution 5 [P]: chain commitment

Replace `chain: Vec<NodeId>` with `chain: BlakeHash` and a
separate Merkle path for each forward. The bandwidth per
forward becomes `O(log r)` rather than `O(r)`. The verifier
checks the path; the cost is `O(log r)` hash verifications.

In production, this is essentially what BLS-aggregated QCs
achieve: a single 96-byte signature (BLS12-381 G1) replaces a
chain of `f + 1` signatures, with the signer set encoded as an
`n`-bit bitmap. The bandwidth saving is `O(f)` per QC.

## Solution 6 [F]: pseudo-Lean lower-bound theorem

```text
structure SyncProtocol (n : Nat) where
  state    : Type
  init     : state
  round    : Nat -> state -> Inbox -> state * Outbox
  decide   : state -> Option Value

def IsByzAgreement (P : SyncProtocol n) (f : Nat) : Prop :=
  forall (input : Value) (corrupt : Set (Fin n)),
    corrupt.card <= f ->
    forall (sched : SyncSched),
      Agreement P input sched corrupt /\
      Validity P input sched corrupt

def TerminatesIn (P : SyncProtocol n) (r : Nat) : Prop :=
  forall input sched, P.round r ... = ... \/ AlreadyDecided

theorem dolev_strong_lower_bound :
    forall (P : SyncProtocol n) (f : Nat),
      IsByzAgreement P f -> TerminatesIn P (f + 1) /\
      not (TerminatesIn P r) for r < f + 1 := by
  -- adversary-strategy game
  sorry
```

For the game, you need:

- A turn structure `(protocol_turn ; adversary_turn)*`
- An adversary-input alphabet (which message to withhold)
- A win condition for the adversary (output divergence between
  two indistinguishable plays)

cslib's `LTS` does not directly model this; `Cslib.Foundations.
Game` (in development) is the right home.

## Solution 7 [V]: recursive succinct chain proofs

A Dolev-Strong chain of `f + 1` signatures is a perfect candidate
for IVC (incremental verifiable computation):

- *Step 0.* The commander's signature on `v`.
- *Step `i`.* A previously-verified chain of length `i` plus a
  new signature from a not-yet-included signer.
- *Step `f + 1`.* The completed chain.

A Pickles-style construction: each step is verified by a
recursive SNARK that takes as public input the previous proof
and the new signer's public key, and outputs a new proof of the
extended chain. The recursion accumulates over `f + 1` steps.
Verifier work is constant (Pickles' final-proof verification).

Per-step constraint cost (Schnorr over Pasta in Pickles): about
3k constraints for the signature check, plus the recursive
verifier circuit (about 500 constraints in Pickles 2024). Total
per-step is about 3.5k constraints; total proof is `O(1)` size
regardless of `f + 1`.

## Solution 8 [V]: BLS aggregation flattens verification

`f + 1` Dolev-Strong signatures aggregate into a single BLS
signature `agg = sum sigma_i` over the same value `v`. The
verifier checks one pairing equation:

```
e(agg, G2) = product e(H(v), pk_i)
```

The product on the right has `f + 1` pairings, but in the
*aggregate* form one can pre-compute `pk_agg = sum pk_i` and
check `e(agg, G2) = e(H(v), pk_agg)`: a single pairing.

Round-complexity is unchanged: `f + 1` actual rounds of message
exchange are still required to *collect* the signatures (each
signer must see `v` and produce its share). Only the
*verifier's work* on the resulting attestation is succinct. The
distinction is the difference between the protocol's *runtime*
(round complexity) and its *output's verifier cost* (auditor /
SNARK complexity).

This is also why HotStuff-style protocols can have linear
authenticator complexity per view (constant verifier work per
QC) while still requiring a logarithmic or constant number of
phases per view: aggregation is a verifier-complexity
optimisation, not a protocol-round optimisation.
