# Module 0021 Solutions

## Solution 1 [T]: reconfiguration safety

The reconfiguration is committed via the SMR like any other
operation. By VR's safety, all honest replicas observe the
reconfiguration commit at the same `op_num`. They then
transition to the new replica set at view `v + 1`, with the
new replicas catching up via recovery.

Linearisability is preserved because:

- All committed operations before the reconfiguration are in
  the log.
- The reconfiguration is a single SMR commit, so it has a
  well-defined position in the linearisation.
- Operations after the reconfiguration are committed by the
  new replica set, with state inherited from the old via
  recovery.

The argument is essentially the same as the safety argument
for view changes: the f + 1 quorum intersection guarantees
that no operation is lost across the transition.

## Solution 2 [P]: client dedup table

```rust
struct ClientTable {
    entries: HashMap<ClientId, (RequestId, Result)>,
}

impl ClientTable {
    fn process(&mut self, client: ClientId, req: RequestId, op: Op)
        -> Result {
        if let Some((latest, result)) = self.entries.get(&client) {
            if *latest >= req {
                return result.clone();
            }
        }
        let result = apply(op);
        self.entries.insert(client, (req, result.clone()));
        result
    }
}
```

Guarantees: at-most-once execution. If a client retries (e.g.
its previous request timed out), the replica returns the
cached result rather than re-executing.

## Solution 3 [F]: pseudo-Lean state diagram

```text
inductive VrStatus where
  | normal
  | view_change
  | recovering
  | reconfiguring (new_set : Finset NodeId)

inductive Transition : VrStatus -> VrStatus -> Prop
  | start_view_change : Transition normal view_change
  | new_view_installed : Transition view_change normal
  | crash : Transition normal recovering
  | recovery_done : Transition recovering normal
  | reconfigure_committed : forall ns,
      Transition normal (reconfiguring ns)
  | reconfigure_done : forall ns,
      Transition (reconfiguring ns) normal
```

Each transition is labelled by the protocol message that
triggers it; the LTS is well-defined.

## Solution 4 [V]: verifiable reconfiguration

The reconfiguration commit is a single VR commit
(~10^6 constraints under BLS aggregation). The new set's
takeover involves:

- Old quorum's signature on the new set: ~10^6 (one BLS
  pairing).
- New quorum's catch-up: f + 1 acknowledgements via signature
  aggregation: ~10^6.

Total: ~3 * 10^6 constraints per reconfiguration. This is
roughly 3x a normal commit but a one-shot operation.

Production parallel: Ethereum's validator-set rotation uses a
similar pattern (each epoch's validator set is committed via
the chain itself, with BLS-aggregated signatures).
