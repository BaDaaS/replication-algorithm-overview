# Module 0042 Exercises

## Exercise 1 [T]: PBFT safety

Prove PBFT's safety: no two honest replicas commit different
operations at the same sequence number. Identify the role of
the 2f + 1 prepare-cert and the f + 1 quorum intersection.

## Exercise 2 [T]: liveness under partial synchrony

Prove PBFT's liveness: under partial synchrony with `f < n /
3`, after GST every client's request is eventually
committed.

## Exercise 3 [P]: implement view change

Extend the crate with view change: VIEW-CHANGE messages,
new-view election, in-progress request migration.

## Exercise 4 [P]: byzantine primary

Construct a scenario where the primary equivocates (sends
different pre-prepares to different replicas). Show that
PBFT's prepare phase detects the equivocation.

## Exercise 5 [F]: pseudo-Lean PBFT

Reference Velisarios's Coq formalisation. Sketch the Lean
analogue using cslib's LTS framework.

## Exercise 6 [V]: zk-PBFT

Estimate the per-request prover cost for verifiable PBFT
with BLS aggregation. Compare to HotStuff's per-request
cost (module 0055).

## Exercise 7 [P]: MAC vs signature

PBFT 1999 uses MACs; the 2002 journal version supports
signatures. Discuss the trade-offs (latency, accountability,
verifiability).
