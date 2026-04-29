# Module 0037 Exercises

## Exercise 1 [T]: external consistency proof

Prove that commit-wait + TrueTime soundness implies external
consistency.

## Exercise 2 [P]: epsilon tuning

For epsilons of 1ms, 7ms, 50ms, 500ms, compute the per-
commit latency overhead and discuss the trade-off.

## Exercise 3 [F]: pseudo-Lean TrueTime

Define `TrueTime` as a typeclass with `now`, `after`,
`before`. State the soundness axiom.

## Exercise 4 [V]: verifiable timestamping

Replace TrueTime with a VDF-based timestamp source. Sketch
the verifiable commit protocol and estimate prover cost.
