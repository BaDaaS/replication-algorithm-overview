# Module 0004 Exercises

## Exercise 1 [T]: PSL impossibility, three-process case

Reproduce the bivalence-by-extension proof of `n > 3f` for the
three-process unauthenticated synchronous case (PSL 1980, Theorem
2). State the two indistinguishable scenarios precisely, and
identify the place in the proof where determinism of the protocol
forces a contradiction.

## Exercise 2 [T]: prove OM(m) by induction

The README gives `OM(m)` recursively. Prove by induction on `m`
that `OM(m)` solves Byzantine agreement for `n > 3m`. Identify the
inductive hypothesis precisely and the role of the majority count
in the inductive step.

## Exercise 3 [T]: tightness of the round bound

Cite Dolev-Strong 1983 (developed in module 0006) to argue that
`OM(f)` and `SM(f)` both achieve the optimal round complexity
`f + 1` for deterministic synchronous Byzantine agreement.
Sketch why a protocol with fewer rounds cannot exist.

## Exercise 4 [P]: implement SM(1)

Extend the crate to a signed-messages variant: each round-1
forward includes an explicit signer chain, and recipients verify
that the chain's first element is the commander's signature on
the value. Use a simple HMAC-style "signature" (a tagged tuple
`(signer, hmac(key, value))`) for pedagogy. Show that with a
malicious commander who forks `Attack` to L1, L2 and `Retreat` to
L3, all honest lieutenants still decide the *honest commander
intent* if at least one honest lieutenant exists.

## Exercise 5 [P]: relate to PBFT's three phases

Explain how PBFT's `pre-prepare`, `prepare`, `commit` phases map
to the OM(1) structure: the leader's broadcast (round 0), the
echo from `2f + 1` replicas (round 1), and the commit (an extra
phase for partial-synchrony liveness, not present in synchronous
OM). Discuss why PBFT needs an extra phase that synchronous
Byzantine agreement does not.

## Exercise 6 [F]: pseudo-Lean SignedChain

Define a `SignedChain V Sig` typeclass in pseudo-Lean (ASCII
only). State the verification predicate: a chain
`(v, sigma_C(v) :: ... :: sigma_Lk(v))` is valid iff the chain's
inner signers are pairwise distinct and each signature verifies
against the prefix-encoding of `v`. Identify the cslib or Mathlib
abstractions you would reuse.

## Exercise 7 [V]: BLS-aggregated quorum certificate

Modern BFT protocols replace the signed-chain of `SM(f)` with a
*quorum certificate* (QC): a single BLS aggregate signature of
`2f + 1` validators on the same value. Discuss:

- What the SNARK encoding of QC verification looks like (one
  pairing check on BLS12-381).
- Why this encoding is preferable to a chain of `f + 1` Schnorrs
  in production.
- The trade-off in formalisation: aggregate signatures need
  pairing-friendly group machinery (Mathlib has BN254 partially;
  BLS12-381 is in progress).

## Exercise 8 [V]: encode OM(1) in a circuit

Sketch a circuit that verifies an OM(1) execution with `n = 4`,
`f = 1`. Identify the constraints needed (signature checks,
chain-consistency checks, majority count). Estimate the
constraint count using Schnorr-over-Pasta as the signature scheme.
