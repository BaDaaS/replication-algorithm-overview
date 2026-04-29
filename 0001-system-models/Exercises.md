# Module 0001 Exercises

Tags: `[T]` theory, `[P]` practice, `[F]` formalisation aspects,
`[V]` verifiability and circuit encoding.

## Exercise 1 [T]: equivalence of DLS variants

DLS define partial synchrony two ways: (A) bounds `D, Phi` exist
at all times but are unknown, and (B) bounds `D, Phi` are known
but only hold from some unknown finite `T_GST` onwards. Prove
that a problem is solvable in Variant A iff it is solvable in
Variant B. Use the doubling-timeout reduction in DLS section 3.2.

## Exercise 2 [T]: from synchrony to partial synchrony preserves
upper bounds

Show that any algorithm correct in the synchronous model with
`(n, f)` resilience is correct in the partially synchronous model
with the same `(n, f)` only after enlarging timeouts. Deduce that
the resilience tight bound `n > 3f` (Byzantine, unauthenticated)
transfers from synchronous to partially synchronous.

## Exercise 3 [T]: separation between asynchrony and partial
synchrony

Exhibit a problem solvable under partial synchrony but unsolvable
under asynchrony. (FLP gives the canonical example: deterministic
binary consensus.)

## Exercise 4 [P]: implement a clock-skew adversary

Extend `SynchronousAdversary` to a `BoundedSkewAdversary` that
introduces per-process clock skew up to `Phi - 1`. Verify that
the leader-broadcast SMR from module 0000 still terminates
quickly. What if you allow `Phi` to grow unboundedly while still
calling the network "synchronous"?

## Exercise 5 [P]: tune timeouts experimentally

Build a tiny demo in which two processes exchange ping/pong
messages under `PartiallySynchronousAdversary { gst = 100,
sync_delay = 1, async_max = 1000 }`. Plot the response latency
versus time and show the bimodal distribution before and after
GST.

## Exercise 6 [F]: formalise the GST modality

In ASCII pseudo-Lean, write the predicate "the network
eventually becomes synchronous with delay `D`" using only Mathlib
primitives (`Filter`, `Filter.atTop`, `Filter.Eventually`).

```text
def IsEventuallySynchronous
    (sched : Time -> Envelope -> Prop) (D : Nat) : Prop :=
  Filter.Eventually
    (fun t : Time =>
      forall e : Envelope, sched t e ->
        e.delivery_time - e.send_time <= D)
    Filter.atTop
```

Discuss what changes if we replace `Filter.atTop` with
`Filter.cofinite`. Discuss what additional Mathlib infrastructure
you would need to prove that this predicate composes well across
sequential protocol stages.

## Exercise 7 [F]: an LTL specification of partial synchrony

Write the partial-synchrony assumption as an LTL formula
(Pnueli 1977) over an atomic proposition `timely(t, e, D)`
("envelope `e` delivered within `D` of its sending time"). Show
that the Variant B definition is exactly `F G timely(D)` and
explain why the corresponding CTL formula `EF AG timely(D)` is
strictly weaker.

## Exercise 8 [V]: timing assumptions and SNARK soundness

A SNARK proof of a chain prefix is a *safety object*: the
verifier accepts that the prefix is consistent with the
protocol. Argue that any liveness claim ("the chain made
progress in real time") is outside the scope of the SNARK and
must be re-proven from the chain's clock or from on-chain
heartbeats. Sketch the additional public input a verifier would
need to enforce a minimal liveness predicate, and identify the
production system that takes this approach (Mina's slot witness;
Ethereum's slot timestamp on the beacon chain).
