# Module 0027 Solutions

## Solution 1 [T]: acyclicity

EPaxos's dependency relation is built such that each command's
`deps` set contains only commands that have been *previously*
seen by the leader. Time-monotonicity of submission gives a
partial order; cycles would require time-travel.

Formally: assign each command a creation timestamp. Each
`deps` entry refers to a strictly earlier timestamp. By
transitive closure, no cycle can exist.

## Solution 2 [P]: geo-distributed simulation

Setup: 5 replicas, simulator with `PartiallySynchronousAdversary`
and per-pair latencies (US-East <-> US-West: 70ms, US-East
<-> EU: 80ms, etc.).

For 10% conflict rate, typical commit latency ~= local
round-trip (~30ms) plus 10% * cross-region round-trip
(~80ms * 0.1 = 8ms) = ~38ms.

Compare to Multi-Paxos with leader in US-East: clients
worldwide pay full cross-region RTT, ~120ms typical.

## Solution 3 [F]: pseudo-Lean

```text
structure DepGraph where
  commands : Set Command
  deps     : Command -> Set Command
  acyclic  : forall c, c ∉ transitive_closure deps c

theorem topological_sort_correct
    {cmds : Set Command} {graph : DepGraph}
    (h_commute : forall c1 c2 ∉ deps_related c1 c2,
                 commute c1 c2) :
    forall (T1 T2 : List Command),
    IsTopologicalSort graph T1 -> IsTopologicalSort graph T2 ->
    apply T1 = apply T2 := by
  -- Use the Generalized Paxos lemma (module 0026 Solution 1).
  sorry
```

## Solution 4 [V]: verifiable EPaxos at scale

For 10000 commands, 5 deps each:

- Per-command BLS aggregate cert: ~10^6.
- Dependency-edge encoding (5 edges each): ~5 * 1k = 5k.
- Total per command: ~10^6.
- Total for 10000 commands without recursion: ~10^10
  constraints.
- With Pickles-style recursion: ~10^6 per step plus 500 for
  the recursive verifier; total ~10^10 sequential prover work,
  parallelisable in a tree.

Compare to Multi-Paxos: ~10^6 per command * 10000 = ~10^10.
Roughly equivalent. EPaxos's overhead is the dependency-graph
encoding, but the per-command cert dominates.

The wins of EPaxos (geographic latency) are not directly
reflected in proving cost; the verifier is on L1 and doesn't
care about which leader proved each command.
