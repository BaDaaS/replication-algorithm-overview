# Module 0032 Solutions

## Solution 1 [T]: refinement

Define a *projection* `proj : CompartmentalisedState ->
ClassicPaxosState` that combines per-role state into the
monolithic equivalent (e.g. proj.acceptor = the union of all
acceptor pool states; proj.proposer = the active proposer's
state).

Refinement theorem: forall scheds, proj(compartmentalised
trace) is a valid classic-Paxos trace.

Proof: by induction on protocol steps. Each compartmentalised
message corresponds to a classic-Paxos message via the role
identity; safety of each step follows from classic Paxos's
safety.

## Solution 2 [P]: role-pool sizing

For 100k QPS, 1000 clients:

- Acceptors: f = 2 -> 5 (mandatory).
- Proposers: 100k QPS / 10k QPS-per-proposer = 10.
- Batchers: 1000 clients / 100 clients-per-batcher = 10.
- Unbatchers: ~1 per replica.
- Replicas: 5 (matches f).
- Read-only replicas: scaled with read load, e.g. 20.

Total: ~50 machines for 100k QPS replicated SMR.

## Solution 3 [F]: pseudo-Lean roles

```text
inductive Role where
  | client | proposer | acceptor | batcher | unbatcher
  | replica | read_replica

structure CompartmentalisedState where
  proposers   : List ProposerState
  acceptors   : List AcceptorState
  batchers    : List BatcherState
  -- etc.

def project (S : CompartmentalisedState)
    : ClassicPaxosState := {
  acceptors := S.acceptors  -- unchanged
  proposer  := chooseLeader S.proposers
  -- ...
}

theorem refines :
    forall (sched : Schedule),
      project (CompTrace sched) =
      ClassicTrace (relabel sched) := by
  sorry
```

## Solution 4 [V]: per-role verifiability

Each role can produce an attestation:

- Proposer: signed proposal.
- Acceptor: signed acceptance.
- Batcher: signed batch hash.
- Replica: signed execution receipt.

Composed proof: aggregate per-role attestations using BLS.
Per commit: ~3 sig verifications (one per role contribution),
~10k constraints.

Compared to monolithic Paxos (~10^6 for one BLS aggregate),
the composed approach is similar order. The decomposition
buys *parallelism* (each role generates its part
independently), not raw constraint reduction.

Production: zk-rollup sequencers naturally decompose
batchers, proposers, executors, and provers into independent
pools.
