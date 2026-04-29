# 0107: Casper CBC

## Historical context

Vlad Zamfir developed Casper CBC ("Correct By Construction")
between 2015 and 2018 (multiple Ethereum-research write-ups,
github.com/ethereum/cbc-casper). CBC takes a different approach
than FFG (module 0106): rather than starting with PBFT-style
voting and proving it correct, CBC starts with safety
*specifications* and builds the protocol such that any
valid execution satisfies them by construction.

The key tool is the *consensus safety oracle*: a function that
inspects the current "view" (set of seen messages) and returns
the set of *consistently-decidable* values. A value is
*finalised* once it appears in every honest validator's
oracle output, i.e., when no Byzantine subset can change the
oracle's verdict.

CBC was an alternative finality gadget for Ethereum, eventually
not adopted in favour of FFG. CBC's intellectual contribution
was its abstract treatment of consensus and the
correct-by-construction design philosophy.

## System and threat model

- **Network.** Asynchronous (FLP-resilient).
- **Failures.** Byzantine; safety threshold parameterised by
  the *fault tolerance threshold* `t`.
- **Cryptography.** Standard signatures.
- **Goal.** Composable, correct-by-construction consensus.

## Theory

### Consensus oracles

A *consensus oracle* `O` is a function from a *view* (set of
messages) to a set of decidable values. Properties:

1. *Consistency.* If `o in O(V)`, then `o in O(V')` for any
   superset `V'`.
2. *Liveness.* `O(V)` is non-empty after enough messages.

CBC defines safety in terms of oracles: a value is *safe*
when the oracle decides it.

### Estimator function

The *estimator* maps a view to a candidate value (e.g., the
heaviest sub-tree in a block tree). The protocol generates a
new message by:

1. Reading the validator's current view.
2. Computing the estimator's value.
3. Signing and broadcasting that value.

Validators are honest if they never publish a message
inconsistent with their estimator output.

### Safety theorem

If `>= 2t/3` validators are honest, any two honest validators'
estimators converge eventually. Equivalently: with high
probability, the oracle eventually decides.

### Comparison: FFG vs CBC

| property              | Casper FFG          | Casper CBC          |
| --------------------- | ------------------- | ------------------- |
| design approach       | bottom-up (PBFT-style) | top-down (oracles) |
| finality semantics    | checkpoint-based    | view-based          |
| accountable safety    | yes (slashing)      | yes (slashing)      |
| underlying chain      | external            | internal            |
| ease of analysis      | concrete            | abstract            |
| adopted by Ethereum   | yes                 | no                  |
| year                  | 2017                | 2015-2018           |

FFG's concrete design made it easier to implement and verify;
CBC's abstract design was philosophically appealing but
practically harder.

### Subsequent influence

- *FFG and Gasper.* Adopted instead.
- *DAG-BFT abstractions.* Some DAG protocols (Aleph, DAG-Rider)
  use CBC-style "safe value" reasoning.
- *Formal verification.* CBC's correct-by-construction style
  inspired later attempts at formally verified consensus
  (Ouroboros formalisation, Algorand TLA+).

## Practice

CBC was not deployed in production. The *correct-by-construction*
philosophy continues to influence research consensus designs.

## Verifiability and circuit encoding

**tag: `partial`.**

CBC circuits would encode the oracle predicate plus signature
verification. The oracle predicate depends on the specific
estimator (e.g., GHOST tree, longest chain); cost varies
accordingly. CBC's abstract framing does not naturally fit
SNARKs, which prefer concrete arithmetic.

## Known attacks and limitations

- *Implementation complexity.* The abstract framework requires
  concrete instantiation of the estimator, oracle, and
  scoring rules.
- *Liveness under partition.* Same as all async protocols.
- *No deployed production system.*

## References

- Zamfir, "Casper the Friendly Ghost", Ethereum Research,
  2015-2018.
- Zamfir, "A Template for Correct-by-Construction Consensus
  Protocols", 2017 working paper.
- ethereum/cbc-casper Github repository, 2018.

## Implementation notes

The crate provides a minimal `Estimator` trait and an
implementation that picks the most-frequent value in a view.
Tests verify that the estimator returns the majority value.

See also [`HISTORY.md`](../HISTORY.md), section "2014 to 2017".
