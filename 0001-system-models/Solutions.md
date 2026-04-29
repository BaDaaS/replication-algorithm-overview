# Module 0001 Solutions

## Solution 1 [T]: equivalence of DLS variants

(A implies B.) Given an algorithm correct under Variant A with
unknown bounds `D, Phi`, schedule the algorithm against Variant B
with the same bounds and any choice of `T_GST`. The Variant A
hypothesis holds at all times, in particular after `T_GST`.

(B implies A.) Given an algorithm `Alg_B` correct under Variant B
with known `D`, build `Alg_A` for Variant A as follows. Run the
*doubling timeout* schedule: for `k = 0, 1, 2, ...`, run a copy of
`Alg_B` with `D := 2^k` and a fresh internal state. Each copy
either terminates (and we accept its output) or fails to make
progress within its timeout. The unknown true bound `D_true` is
finite, so eventually `2^k > D_true` and the corresponding copy
satisfies the Variant B premise from time `0`. By soundness of
`Alg_B`, that copy terminates correctly. The cost is at most
`O(log(D_true))` rounds wasted before `Alg_A` terminates.

## Solution 2 [T]: synchronous to partial synchrony

Pick any synchronous algorithm `Alg_sync` with timeout `D` and
resilience `(n, f)`. Define `Alg_psync` to run `Alg_sync` with the
doubled timeout `2 * D_k`, where `D_k = 2^k` is increased after
every failed round. Safety of `Alg_psync` is inherited from
`Alg_sync`, because the safety argument of `Alg_sync` does not
depend on the timeout's correctness, only on the messaging order
within a round. After `T_GST`, the doubling reaches a bound that
exceeds the true delay; from then on, every round terminates
within its timeout, and `Alg_psync` matches the resilience of
`Alg_sync`.

The transferred bounds are the synchronous bounds:

- Crash: `f < n/2`.
- Authenticated Byzantine: `f < n/2`.
- Unauthenticated Byzantine: `f < n/3`.

These are tight in both models (DLS Theorems 5.1 to 5.4 give the
upper bounds; matching lower bounds carry over from the
synchronous case).

## Solution 3 [T]: separation between asynchrony and partial
synchrony

Deterministic binary consensus is solvable under partial synchrony
(by DLS Theorem 5.2 with `f < n/2` crash failures) and unsolvable
under asynchrony (by FLP, even with `f = 1`). Module 0005 develops
FLP in full.

## Solution 4 [P]: clock-skew adversary

Implement `BoundedSkewAdversary { phi }` that delivers each message
at `now + delay + skew`, where `skew` is uniformly random in
`[0, phi - 1]`. Tests show:

- For `phi <= D / 4` (say, `phi = 1, delay = 5`), the
  leader-broadcast SMR converges as before; the local timeouts
  must accommodate the worst-case end-to-end delay
  `D + 2 * (phi - 1)`.
- For `phi` allowed to grow unboundedly, the network is no longer
  synchronous in the DLS sense: there is no global bound on the
  difference between any two processes' local times. Some
  "synchronous" algorithms (e.g. lockstep round protocols) lose
  their safety argument because the round-by-round invariant
  breaks.

## Solution 5 [P]: tune timeouts experimentally

A representative trace (depending on RNG seed):

- For `t in [1, gst)`, response latency is uniformly distributed
  in `[1, async_max]`, mean about `async_max / 2 = 500`.
- For `t in [gst, end)`, response latency is exactly `sync_delay`.

The bimodal distribution is the practical signature of partial
synchrony: pre-GST tail behaviour is wide, post-GST behaviour is
tight. Production stacks use percentile latency monitoring (`p99`,
`p99.9`) to detect when GST has plausibly ended and to size their
view-change timeouts accordingly.

## Solution 6 [F]: formalise the GST modality

The pseudo-Lean shown in the exercise is the natural definition.
The choice of filter matters:

- `Filter.atTop` says "for all sufficiently large `t`". This is
  the right modality for "from some `T_GST` onwards".
- `Filter.cofinite` says "for all but finitely many `t`". This
  permits a finite set of post-GST "blips" of asynchrony. DLS
  forbids them (Variant B) but the cofinite version still admits
  consensus, because the cofinite blips only delay liveness, not
  break safety. The resulting model is sometimes called *eventually
  synchronous with finitely many violations* and is studied in
  Aguilera et al. 2003.

To compose this predicate across sequential protocol stages, one
needs `Filter.Eventually.and` (already in Mathlib) and a
"monotone-eventual" lemma stating that if synchrony holds eventually
for `D_1` and eventually for `D_2`, then it holds eventually for
`max(D_1, D_2)`.

## Solution 7 [F]: LTL specification

Let `timely(t, e, D)` be the atomic proposition "envelope `e`,
sent at time `t_send`, was delivered at some `t_deliver` with
`t_deliver - t_send <= D`". The DLS Variant B assumption is

```
F G ( forall e : Envelope, timely(now, e, D) )
```

read as "eventually it is always the case that every envelope is
timely". `F G P` says `P` holds from some point onwards forever;
this matches GST exactly.

The CTL counterpart `EF AG timely(D)` says "there exists a path
along which eventually `timely(D)` holds forever". This is
strictly weaker because there might be alternative branches in
which `timely(D)` never stabilises. In a deterministic timing model
the two coincide, but in models with adversarial branching (e.g.
adaptive adversaries that can choose to delay further) they
diverge. The LTL formulation is the operationally meaningful one.

## Solution 8 [V]: timing and SNARK soundness

A SNARK proof of a chain prefix `(s_0, s_1, ..., s_k)` says
"there exists an honest execution producing this prefix". It does
*not* say "this execution made real-time progress between `s_0`
and `s_k`". Liveness is an LTL `F` claim about the *physical
schedule*, which is not part of the SNARK statement.

To enforce a liveness predicate inside the verifier, one adds a
public input `current_time` (or `current_slot`) and asserts that
the prefix's last block timestamp is within `delta_max` of
`current_time`. Mina does this via the "slot witness": the
verifier compares the chain's slot to the local slot and rejects
proofs whose slots are too far in the past. Ethereum beacon-chain
light clients use the slot field of the latest signed header for
the same purpose. The trust assumption shifts: the verifier now
trusts its local clock, not just the SNARK.
