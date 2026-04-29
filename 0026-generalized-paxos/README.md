# 0026: Generalized Paxos

## Historical context

Lamport's 2005 technical report "Generalized Consensus and
Paxos" extended Fast Paxos by exploiting *commutativity* of
operations. If two operations `op_1` and `op_2` commute (e.g.
"increment counter A" and "increment counter B"), they can be
applied in either order without affecting the final state.
Generalized Paxos lets replicas decide on a *command
structure* (a partial order of commands) rather than a strict
total order, eliminating the need for the leader to serialise
commuting operations.

The result: fewer fast-path conflicts, higher throughput.

## System and threat model

- **Network.** Asynchronous, reliable.
- **Failures.** Crash-recovery; `f < n / 3` for the fast path.
- **State machine.** The state machine specifies which
  operations commute (a *command-equivalence* predicate).
- **Goal.** Decide on a partial order such that any
  linearisation gives the correct state.

## Theory

### Command structures

A *command structure* `C-Struct` is a partial order over
commands satisfying:

- *Reflexivity, antisymmetry, transitivity* (it's a partial
  order).
- *Compatibility with state machine.* Any topological sort of
  `C-Struct` gives the same final state.

Two commands `c_1, c_2` are *compatible* (or commutative) if
there exists no ordering of state machine operations such that
applying `c_1` then `c_2` produces a different state than
`c_2` then `c_1`.

### Algorithm sketch

Replicas exchange *proposals* (commands) and build their local
`C-Struct`s.

```
client -> all acceptors: PROPOSE(command)
acceptor: appends command to local C-Struct (extending the
          partial order)
acceptor -> proposer: ACCEPTED(C-Struct)

proposer: collects f + 1 ACCEPTEDs
  if all C-Structs agree: decided
  if some disagree: leader runs classic recovery
```

The "decided" `C-Struct` is the union of the per-replica
structures, possibly with extra dependencies added to break
ambiguity.

### Theorem (Generalized Paxos correctness)

For any state machine with a well-defined commutativity
predicate, Generalized Paxos satisfies linearisability via the
decided `C-Struct`.

### Performance

In the contention-free case (all clients propose commuting
operations), Generalized Paxos commits each operation in 1
round-trip without conflicts. Under contention, it falls back
to classic Paxos for the conflicting commands.

Compared to Fast Paxos (module 0025), Generalized Paxos has
*fewer* slow-path triggers because commuting commands don't
conflict.

## Practice

### Where it shows up

- *EPaxos* (Moraru-Andersen-Kaminsky 2013, module 0027) is the
  best-known Generalized Paxos descendant, with explicit
  multi-leader design.
- *Bolt-on databases.* Some KV stores exploit operation
  commutativity for higher throughput.
- *CRDT-based systems.* Conflict-free replicated data types
  bake commutativity into the data structure itself, achieving
  similar effects without explicit consensus.

### Engineering challenges

- *Commutativity inference.* Determining whether two commands
  commute is application-specific. Production: precomputed
  conflict relations or coarse-grained "all writes to the same
  key conflict" rules.
- *C-Struct overhead.* Maintaining and exchanging partial
  orders is more expensive than a flat sequence.

## Formalisation aspects

```text
class StateMachine where
  apply : State -> Command -> State
  commute :
    forall s c1 c2,
    apply (apply s c1) c2 = apply (apply s c2) c1

structure CStruct where
  commands : Set Command
  order    : Command -> Command -> Prop  -- partial order

theorem generalized_paxos_correctness :
    forall (S : StateMachine) (CS : CStruct),
    Linearisable CS = SmrCorrect := by
  sorry
```

The formalisation requires modelling the state machine's
commutativity as a typeclass hypothesis. Mathlib's `Order`
infrastructure supports the partial-order machinery.

## Verifiability and circuit encoding

**Tag: `friendly`** for the structure; `partial` for the
in-circuit C-Struct verification.

A SNARK proof of a Generalized Paxos commit must encode the
C-Struct's partial order. For `k` commands, encoding is
`O(k^2)` constraints (transitive closure plus
compatibility-with-state-machine checks per pair).

Per commit (assuming small commit batches):

- BLS aggregate of `f + 1` accepteds: ~10^6 constraints.
- C-Struct edge encoding for the batch: `O(k^2)` constraints,
  ~10k per pair.

Total per batch: ~10^6 + k^2 * 10^4 constraints. For `k <
20`, this is dominated by the BLS pairing.

## Known attacks and limitations

- *Commutativity errors.* If the commutativity predicate is
  wrong, the protocol's correctness fails. Production must
  verify it carefully.
- *Worst-case throughput.* Under all-conflict workloads,
  Generalized Paxos degenerates to classic Paxos.

## Implementation notes

This module is purely conceptual; the crate exposes only the
`commute` predicate trait. The full protocol is in EPaxos
(module 0027).

## References

- Lamport, "Generalized Consensus and Paxos", MSR-TR-2005-33.
- Pirleski, "Generalized Paxos in TLA+", 2007 thesis.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
