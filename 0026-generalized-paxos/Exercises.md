# Module 0026 Exercises

## Exercise 1 [T]: commutativity correctness

Prove: if all pairs of commands in a `C-Struct` commute, then
all topological sorts produce the same final state.

## Exercise 2 [P]: KV with range queries

Modify `KvWrite` to a richer command set including range
queries. Define which commands commute (point reads commute
with all writes to other keys; range queries conflict with
writes to any key in the range).

## Exercise 3 [F]: pseudo-Lean Commutes typeclass

Define `class Commutes (alpha : Type) where commute_eq : ...`
in pseudo-Lean. Prove that `KvWrite` instantiates it with the
"different keys" predicate.

## Exercise 4 [V]: in-circuit C-Struct verification

Sketch a SNARK encoding of a small C-Struct (k = 10
commands) and the constraint cost.
