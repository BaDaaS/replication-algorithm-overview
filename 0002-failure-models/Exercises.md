# Module 0002 Exercises

Tags: `[T]` theory, `[P]` practice, `[F]` formalisation aspects,
`[V]` verifiability and circuit encoding.

## Exercise 1 [T]: refinement of failure models

Prove formally that crash-stop refines authenticated Byzantine
(every protocol that tolerates `f` authenticated Byzantine
faults tolerates `f` crashes). Deduce that a protocol's resilience
threshold against authenticated Byzantine is a lower bound for
its resilience against crashes.

## Exercise 2 [T]: tight `f < n/3` lower bound (unauthenticated)

Reproduce the `f < n/3` lower bound for synchronous unauthenticated
Byzantine consensus in the three-process case (PSL 1980, Theorem
2). Show that with `n = 3, f = 1` no protocol can guarantee
agreement under all Byzantine schedules.

## Exercise 3 [T]: crash-recovery without stable storage

Argue that crash-recovery without stable storage is *not* a
refinement of crash-stop. Give a two-process scenario in which the
recovering process makes a local decision inconsistent with its
pre-crash decision, breaking SMR safety.

## Exercise 4 [P]: build a crash-recovery node

Extend the crate's `Beacon` test process to support crash-recovery:
on a tick after the crash time, replay any persisted state from a
mock disk. Discuss what subset of the state must be persisted.

## Exercise 5 [P]: instrument an equivocation detector

Use `EquivocatingAdversary` with a Byzantine sender that emits the
value `0` to recipient `1` and the value `1` to recipient `2`.
Implement a simple detector at recipient `3` (the honest, non-
target receiver) that suspects the sender after observing a
contradiction. Note that this detector cannot work without
communication between recipients; develop it into a one-round
gossip protocol.

## Exercise 6 [F]: failure mode as a refinement

In ASCII pseudo-Lean, define a refinement order `<=` on
`FailureMode` such that the resilience theorem is monotone: if
`mode1 <= mode2` and the protocol tolerates `mode2`, it tolerates
`mode1`. State the precise conditions on the protocol's
transition relation that justify the monotonicity.

## Exercise 7 [V]: slashable behaviour as a circuit predicate

For each of the following, write a *predicate* over signed
messages that captures the slashable offence, and discuss its
SNARK-friendliness:

- Casper FFG double-vote (two votes from the same validator at the
  same height).
- Casper FFG surround-vote (one vote's source-target span strictly
  contains another's).
- Tendermint double-prevote.

Identify the cryptographic primitives needed (signature scheme,
hash) and the production system that uses them.

## Exercise 8 [V]: rational deviation under verifiability

Verifiable replication does not by itself prevent rational
deviations. Read Eyal-Sirer 2014 (selfish mining) and discuss
whether a SNARK-based light-client proof of Bitcoin's longest-
chain rule would detect or prevent selfish mining. (It would not.
Argue why and identify the protocol-level mitigations: increased
block reward symmetry, FruitChains 2017, uncle inclusion in
GHOST.)
