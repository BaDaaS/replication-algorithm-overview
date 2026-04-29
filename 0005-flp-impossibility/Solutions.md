# Module 0005 Solutions

## Solution 1 [T]: Lemma 1 in full

Walk along the Hamming sequence
`C_0 = C(0,...,0), C_1 = C(1, 0, ..., 0), C_2 = C(1, 1, 0, ...,
0), ..., C_n = C(1,...,1)`. Adjacent pairs `C_k, C_{k+1}`
differ in input bit `k+1`.

By validity, `C_0` is 0-valent (every honest process inputs 0,
so any decision is 0). Symmetrically, `C_n` is 1-valent.

If every `C_k` is univalent, then there exists a smallest `k^*`
with `C_{k^*}` not 0-valent. By validity / continuity of the
sequence, `C_{k^* - 1}` is 0-valent. Two cases:

- *`C_{k^*}` is bivalent.* Then we have a bivalent
  configuration; done.
- *`C_{k^*}` is 1-valent.* `C_{k^* - 1}` is 0-valent and
  `C_{k^*}` is 1-valent, differing in process `k^*`'s input
  bit. Consider the asynchronous schedule that crashes process
  `k^*` immediately. From `C_{k^* - 1}` and `C_{k^*}`, the
  crashed-process schedules are identical (process `k^*` never
  acts), so they reach the same final configuration. But that
  configuration must be 0-valent (extending `C_{k^* - 1}`) and
  1-valent (extending `C_{k^*}`): contradiction.

The "at most one crash failure" hypothesis is used in case 2:
crashing one process and proceeding with `n - 1` honest
processes is a legal adversary schedule.

## Solution 2 [T]: Lemma 2 case (b) in full

Recall: `e` is the distinguishing step at process `p` with
message `m' != m`, both addressed to `p`. We want to show that
the assumption `D_{e_1}` 0-valent and `D_{e_2}` 1-valent leads
to a contradiction.

Crash `p` *after* delivering `m'` (the distinguishing step) and
*before* delivering `m`. From this point on, `p` is silent. So
both `D_{e_1}` and `D_{e_2}` reduce to a configuration where
`p` no longer acts, and only the other `n - 1` processes
continue.

Critically, the "after delivery, then crash" sequence is a
legal one-fault schedule: the protocol allows at most one crash,
which we are using here. From this crashed configuration, the
remaining `n - 1` processes' execution is identical in both
extensions (their states do not depend on `p`'s action since
`p` has crashed). Hence the final decision is the same in both
extensions, contradicting `D_{e_1}` being 0-valent and
`D_{e_2}` being 1-valent.

The "at most one crash" hypothesis is precisely what authorises
crashing `p` here. Without it, we would have to argue without
crashing, which fails because `p`'s state in `D_{e_1}` and
`D_{e_2}` differs.

## Solution 3 [T]: FLP fails with synchrony

Lemma 2 uses *commutativity* of step deliveries: the schedule
can move `m`'s delivery arbitrarily far in the future without
changing the protocol's behaviour. In synchrony, the schedule
is fixed: round `r` delivers all round-`r` messages
simultaneously. The adversary cannot defer `m` indefinitely
because by round `r + 1` it must be delivered.

Concretely, the case-(a) commutativity argument requires that
delivering `m` and `m'` can be reordered freely. Synchrony
forbids this when `m, m'` are both round-`r` messages (they are
delivered in lockstep) or when one is round-`r` and the other
round-`r+1` (the latter is delivered strictly after the former).

The "asynchronous" hypothesis of FLP is therefore *load-bearing*:
the impossibility is specific to the asynchronous model. PSL
1980 and DLS 1988 give matching positive results in synchrony
and partial synchrony.

## Solution 4 [P]: 3-process bivalence-preserving schedule

A 3-process voter exchanges votes; majority decides. Adversary
strategy:

```
state: pending = list of (sender, receiver, msg) yet to deliver
loop:
  for each candidate next-deliverable message m:
    simulate two extensions of the current global state
    e_0 = next 0-decision from delivering m and continuing
    e_1 = next 1-decision from delivering m and continuing
    if e_0 and e_1 both reachable:
      deliver m
      break
    else:
      mark m as committing-to-univalence
  if all pending m commit-to-univalence:
    use case (a)/(b)/(c) of Lemma 2 to construct a new schedule
```

This is the operational form of FLP's existence proof; a
concrete adversary that does it for `n = 3` is in Aspnes 2003,
"Randomized protocols for asynchronous consensus", section 2.

## Solution 5 [P]: failure probability bound

For a deterministic asynchronous consensus protocol, *no*
finite step bound `k` guarantees termination against an
adversarial scheduler. The probability of non-termination at
step `k` against a worst-case adversary is `1` (Lemma 2 builds
an infinite bivalent execution). Against a uniformly random
schedule, the probability typically decays exponentially: the
adversary's freedom is large enough that almost no random
schedule preserves bivalence past `O(n)` steps.

The contrast is the FLP point: deterministic *worst-case*
guarantees fail; *typical-case* termination is fine. This is
why production systems work in practice while FLP rules them
out in theory.

## Solution 6 [F]: pseudo-Lean Configuration

```text
structure Envelope where
  from : NodeId
  to   : NodeId
  msg  : Message

structure Configuration (n : Nat) where
  states  : Fin n -> ProcessState
  in_flight : Multiset Envelope

inductive Step (n : Nat)
    : Configuration n -> Configuration n -> Prop
  | deliver
      (C : Configuration n) (e : Envelope) (h : e ∈ C.in_flight) :
      Step n C (...the result of delivering e...)

def Reachable (n : Nat) := Relation.ReflTransGen (Step n)
```

cslib's `LTS` could be specialised: an `LTS Configuration
Envelope` with the deliver step. Mathlib provides `Multiset`,
`Relation.ReflTransGen`, and the `Decidable` lemmas for
membership.

## Solution 7 [F]: LTL form of FLP

The LTL formulation:

```
exists a schedule s: AsyncSched, exists honest process p:
    G F not decided_p in s
```

(Pnueli's `G F` is "always eventually"; `not decided_p` is
"`p` has not yet decided".) The negation reads "every schedule
eventually decides everywhere", i.e. termination always holds;
FLP says this fails.

The configuration-graph formulation is:

```
forall protocol P,
  exists schedule s: never_decides P s
```

Both are expressible. The LTL form is closer to the way TLA+
specifies liveness; the configuration-graph form is closer to
Lynch's textbook proof. For Lean, the configuration form is
arguably easier because it reduces to a pure existential
statement about the LTS (no temporal modal logic to import); the
LTL form requires Mathlib's `Filter.Eventually` plus a custom
`Always` modality.

## Solution 8 [V]: randomised escape

Ben-Or 1983 escapes FLP by replacing deterministic decisions
with coin flips. Each round, processes vote and, if no
super-majority emerges, flip a coin and try again. With
probability 1, *some* round eventually has a super-majority
that propagates to a decision. The expected number of rounds is
exponential in the worst case (Ben-Or's original) and constant
with a *common coin* (Rabin 1983).

Verifiable variant. To make the coin SNARK-checkable, the
randomness must be unbiased and verifiable:

- *Threshold-BLS coin.* `t + 1` of `n` validators sign a
  per-round nonce; the resulting threshold signature acts as
  the coin. Verifying the coin in circuit is one BLS pairing
  check.
- *VDF coin.* A delay function (Verifiable Delay Function) is
  evaluated on a public seed; the proof of correct evaluation
  is the coin's witness. drand uses this pattern.
- *Randomness beacon.* drand publishes a per-second beacon over
  BLS12-381; HoneyBadger BFT-style protocols can adopt it
  directly. Production systems: drand (League of Entropy),
  Filecoin's randomness beacon, Mina's slot-VRF (which is
  protocol-internal but conceptually similar).

A SNARK proof of a HoneyBadger-style protocol's terminated run
must include the per-round coin's witness as public input. The
verifier checks the witness once and trusts the coin. This is
how Mina's Pickles handles the per-slot VRF outputs in its
Samasika consensus circuit.
