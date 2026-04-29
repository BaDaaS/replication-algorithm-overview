# Module 0003 Exercises

Tags: `[T]` theory, `[P]` practice, `[F]` formalisation aspects,
`[V]` verifiability and circuit encoding.

## Exercise 1 [T]: complete the impossibility proof

The README's proof sketch handles the case where the *last*
message is dropped. Complete the proof rigorously:

- Define "minimum-message run" precisely.
- Show that the minimum exists (there is at least one finite run
  in which both decide).
- Carry out the case split on the direction of the last message
  in full.

## Exercise 2 [T]: probabilistic Two Generals

Show that no deterministic protocol achieves agreement with
probability 1 under a channel that drops each message
independently with probability `p > 0`. Bound the failure
probability of an `r`-round protocol by `p^r` and discuss.

## Exercise 3 [P]: practical workarounds in production

Read RFC 793 (TCP), Section 3.4 ("Establishing a connection") and
identify which of TCP's design choices acknowledge the Two
Generals impossibility:

- Why three-way handshake and not two-way?
- Why a SYN cookie?
- Why is the server's accept queue finite?

## Exercise 4 [F]: pseudo-Lean lossy schedule

Model a *lossy schedule* as a partial function `Time -> Envelope
-> Bool` with `false` meaning "dropped". Write the *minimum-
message* induction in pseudo-Lean using `Nat.rec` and discuss
which Mathlib facts are needed (well-foundedness, decidable
equality on messages).

## Exercise 5 [V]: cross-chain finality reduces to Two Generals

A light client of chain `A` watching chain `B`'s finality faces a
generalised Two Generals: it needs to know that `B` has observed
its own update. Discuss how IBC and zk-bridges sidestep the
impossibility through (i) bounded loss assumptions, (ii)
cryptographic commitments (zk receipts), and (iii) economic
finality (slashable witnesses).
