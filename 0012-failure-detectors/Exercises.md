# Module 0012 Exercises

## Exercise 1 [T]: weak-to-strong completeness

Show how to transform a `<>W`-detector (weak completeness +
eventual weak accuracy) into a `<>S`-detector (strong
completeness + eventual weak accuracy) by gossiping suspect
sets. Identify the messaging cost.

## Exercise 2 [T]: `Omega` is the weakest

Outline the CHT 1996 proof that `Omega` is the weakest detector
solving consensus. Identify the role of the *eventually-honest
leader*.

## Exercise 3 [T]: Byzantine-fault detector classes

Discuss why the gossip-based weak-to-strong transformation
breaks under Byzantine faults. Cite Doudou-Schiper 1998's
muteness detectors.

## Exercise 4 [P]: SWIM membership

Read SWIM (Das-Gupta-Motivala 2002) and discuss how it differs
from a vanilla heartbeat detector: piggybacked dissemination,
indirect probes, randomised peer selection. What guarantees does
SWIM provide?

## Exercise 5 [P]: tune timeouts experimentally

Vary `timeout` and `interval` in `HeartbeatNode`; measure the
detection latency for a crashed node and the false-suspicion
rate under partial synchrony.

## Exercise 6 [F]: pseudo-Lean detector classes

Define the `<>S` and `<>P` typeclasses in pseudo-Lean. Identify
the LTL form: `<>S` is `F G (forall faulty f, F suspected f)`.

## Exercise 7 [V]: verifiable detector outputs

A signed-heartbeat detector produces verifiable output: each
heartbeat is a signed message. Sketch a circuit that takes a
signed heartbeat and the current time and accepts iff the
heartbeat is fresh. Application: cross-chain liveness proofs.

## Exercise 8 [V]: relate to slashing

Casper FFG slashing implements a verifiable accuracy
predicate: it identifies validators who have *wrongly accused*
honest peers (e.g. by signing equivocating votes). Discuss the
mapping from `<>S` accuracy to FFG slashing.
