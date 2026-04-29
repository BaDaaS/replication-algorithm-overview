# Module 0019 Exercises

## Exercise 1 [T]: 2PC blocking

Prove formally that 2PC blocks under coordinator failure: a
participant in PREPARED cannot deduce the global decision
without communicating with the coordinator or another
participant who has the decision.

## Exercise 2 [T]: 3PC non-blocking under synchrony

Sketch the 3PC recovery protocol: how does a surviving
participant elect a new coordinator and decide based on
PRE_COMMIT state?

## Exercise 3 [T]: 3PC unsafe under asynchrony

Construct an asynchronous schedule under which 3PC violates
agreement.

## Exercise 4 [P]: termination detector

Implement a timeout-based termination detector: if a
participant in PREPARED state does not hear from the
coordinator within `T` ticks, it queries other participants.
Compare to 3PC's design.

## Exercise 5 [P]: extend to 3PC

Extend the crate's 2PC simulator to 3PC. Verify that under a
crash-after-PRE_COMMIT scenario, the participants can deduce
the decision from each other.

## Exercise 6 [F]: pseudo-Lean atomic-commit

State the atomic-commit specification (agreement + validity)
in Lean. Show that 2PC implements it under the
no-coordinator-crash assumption.

## Exercise 7 [V]: verifiable atomic commit

Sketch a verifiable 2PC: signed PREPARE, VOTES aggregate
into a BLS quorum, signed DECIDE. Estimate the SNARK
constraint count for one commit.

## Exercise 8 [V]: cross-chain atomic commit

Discuss zk-bridges as an analogue of atomic commit across
two chains: each chain's "vote" is a state-root commitment;
the "decision" is when both chains observe each other's
commit. What replaces the coordinator?
