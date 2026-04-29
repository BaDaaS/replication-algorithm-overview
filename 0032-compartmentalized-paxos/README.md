# 0032: Compartmentalized Paxos

## Historical context

Whittaker, Giridharan, Szekeres, Hellerstein, and Stoica
published "Scaling Replicated State Machines with
Compartmentalization" at VLDB 2021. The paper observes that
classic Paxos bundles many functions into each replica:
proposer, acceptor, batcher, unbatcher, log replicator,
state-machine executor. By *decomposing* these into separate
machine roles, each role can be scaled independently.

The result: Multi-Paxos throughput scaled from `~10^4` ops/s
to `~5 * 10^5` ops/s on commodity hardware, a ~50x
improvement.

## System and threat model

Same as Multi-Paxos.

## Theory

### Roles

- *Client.* Submits operations.
- *Proposer.* Receives requests, runs Paxos.
- *Acceptor.* Stores accepted values, votes.
- *Batcher.* Aggregates client requests into batches.
- *Unbatcher.* Splits committed batches for execution.
- *Replica.* Executes operations against state machine.
- *Read-only replica.* Serves linearisable reads from a
  recent committed prefix.

In classic Paxos, one process plays all roles. Compartmental-
ised Paxos puts each role on a separate machine pool,
independently scalable.

### Performance

Whittaker et al. report:

- *Classic Paxos.* ~`10^4` ops/s.
- *Compartmentalized.* ~`5 * 10^5` ops/s.

The bottleneck moves from the single leader to the slowest
role. Each role is independently provisioned.

### Correctness

The decomposition is *behaviour-preserving*: every
compartmentalised execution is a valid classic Paxos
execution (with all roles co-located on virtual machines).
Safety follows directly.

## Practice

### Production-style example

For a 1000-client deployment:

- 5 acceptors (matches f = 2).
- 10 proposers (parallelism).
- 20 batchers (gather requests).
- 5 replicas (execute decisions).
- 10 read-only replicas (linearisable reads).

Total machines: 50, each with focused workload.

### Why production matters

Modern data centres have abundant cheap servers. Vertical
scaling (one fast machine) is more expensive than horizontal
(many cheap machines). Compartmentalised Paxos exploits the
horizontal scaling.

## Formalisation aspects

The role-based decomposition admits a clean Lean
formalisation:

```text
class Compartmentalisation (Total : Type) where
  Proposer  : Type
  Acceptor  : Type
  Batcher   : Type
  -- etc.
  refines :
    forall (sched : Schedule),
      Proj_Total sched = ClassicPaxosState sched
```

The "refinement" is the safety-preservation theorem.

## Verifiability and circuit encoding

**Tag: `friendly`.**

Each role can be verified independently. The composed proof
combines per-role attestations. No new circuit complexity
beyond classic Multi-Paxos.

## Known attacks and limitations

- *Operational complexity.* Many machine pools to manage.
- *Inter-pool latency.* Batcher-to-proposer-to-acceptor adds
  hops; compensates for per-pool speedup.
- *Failure handling.* Each pool needs failure handling;
  cascading failures can be tricky.

## Implementation notes

The crate provides type-level definitions of the roles.
Full implementation is in the authors' Frankenpaxos reference
codebase.

## References

- Whittaker, Giridharan, Szekeres, Hellerstein, Stoica,
  "Scaling Replicated State Machines with
  Compartmentalization", VLDB 2021.
- Frankenpaxos reference implementation,
  github.com/mwhittaker/frankenpaxos.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to 2023".
