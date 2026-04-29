# Module 0022 Exercises

## Exercise 1 [T]: safety proof in detail

Reproduce the safety proof: if `b_1 < b_2` both succeed,
`v_1 = v_2`. Identify the role of (i) majority intersection,
(ii) acceptor's promise, (iii) proposer's choice in phase 2.

## Exercise 2 [T]: dueling proposers

Construct a schedule where two proposers with adjacent ballots
keep stealing each other's quorum, preventing decision. Show
that this is consistent with FLP: synchronous progress is not
guaranteed.

## Exercise 3 [P]: leader election

Add a leader-election layer: only one process at a time may
propose. Show that termination follows from the leader's
liveness.

## Exercise 4 [F]: pseudo-Lean Synod

Define `SynodAcceptor` and `SynodProposer` in pseudo-Lean.
State the safety theorem.

## Exercise 5 [V]: verifiable single-decree

Sketch a SNARK encoding of one Synod decision: BLS-aggregated
phase-1 promises, BLS-aggregated phase-2 accepteds. Estimate
constraint count.
