# 0029: Stoppable and Vertical Paxos

## Historical context

Two reconfiguration-focused Paxos variants:

- *Stoppable Paxos* (Lamport-Malkhi-Zhou 2008): introduces a
  "stop" command that cleanly halts the current configuration
  to allow a new one to take over.
- *Vertical Paxos* (Lamport-Malkhi-Zhou 2009): extends
  Stoppable Paxos with a *master* that orchestrates
  reconfigurations. The master itself is replicated by another
  Paxos instance.

Both address production needs that the original Paxos left
implicit: how to add and remove replicas safely while the
protocol is running.

## System and threat model

- **Network.** Asynchronous, reliable.
- **Failures.** Crash-recovery; `f < n / 2` per configuration.
- **Goal.** SMR with safe online reconfiguration.

## Theory

### Stoppable Paxos

Add a special *stop* operation to the SMR. When `stop` is
committed at slot `s`, the current configuration ceases to
accept proposals for slots beyond `s`. A new configuration
takes over starting at slot `s + 1`.

Reconfiguration phases:

1. *Stop.* Current config commits `stop`. Slots beyond `s`
   are forbidden in the current config.
2. *Install.* Master broadcasts the new configuration, with
   slots starting at `s + 1`.
3. *Resume.* New config begins normal operation.

### Vertical Paxos

The *master* is a separate replicated state machine that
tracks the *current configuration* over time. Each
configuration `C_i` is a tuple `(epoch_i, replicas_i,
start_slot_i)`.

The master decides when to install a new configuration; the
data SMR runs under the master's current configuration.

```
master SMR (config-replica): decides on configuration changes
data SMR (data-replicas):    runs under master's current config
```

The master's SMR can use any consensus protocol (Paxos,
Multi-Paxos, etc.); typically it is itself Multi-Paxos with a
small fixed replica set.

### Theorem (Vertical Paxos correctness)

Under crash-recovery and `f < n / 2` per configuration,
Vertical Paxos satisfies SMR linearisability across
configuration changes.

*Proof sketch.* The two SMRs (master + data) compose: the
master's safety ensures replicas agree on the current
configuration; within each configuration, data SMR safety
applies.

### Why two papers

Stoppable Paxos handles the "stop" mechanism; Vertical Paxos
adds the master orchestration. The two together give a clean
production reconfiguration story.

## Practice

### Production

- *Spanner's Paxos groups* are reconfigured using a
  master-coordinated protocol structurally similar to Vertical
  Paxos.
- *etcd's Raft* uses a different reconfiguration approach
  (joint consensus); the goal is the same.

### Why operationally important

Real deployments add and remove machines constantly:

- Hardware failures.
- Capacity changes.
- Region migrations.
- Software upgrades.

Without a clean reconfiguration protocol, operators must take
the system offline for changes. Stoppable + Vertical Paxos
allows online changes.

## Formalisation aspects

```text
structure VerticalPaxosState where
  master_smr   : MasterSmrState
  data_smr     : DataSmrState
  current_cfg  : Configuration

theorem vertical_paxos_safety
    (master_safe : MasterSmrSafety)
    (data_safe : forall cfg, DataSmrSafety cfg) :
    VerticalPaxosSmrSafety := by
  -- Compose master safety with per-cfg data safety.
  sorry
```

## Verifiability and circuit encoding

**Tag: `friendly`.**

Vertical Paxos's verifiability composes:

- Master proof: per-configuration commit ~10^6 constraints.
- Data proof: per-data-slot ~10^6 constraints.
- Configuration commitment: included as public input.

Each data proof carries the configuration ID it was made
under. The verifier checks that the data proof's signers
match the master's currently-installed configuration.

This is the canonical pattern for Ethereum's beacon-chain
validator-set rotation: the chain commits to the validator
set; subsequent data proofs reference that set.

## Known attacks and limitations

- *Concurrent reconfigurations.* The master serialises them.
- *Master failure.* The master itself is replicated; it
  satisfies SMR safety internally.
- *Configuration drift.* Replicas may temporarily run under
  different configurations during transition; the protocol
  handles this with epoch numbers.

## Implementation notes

This module is conceptual. The crate exposes the
configuration tuple type:

```rust
struct Configuration {
    epoch: u32,
    replicas: Vec<NodeId>,
    start_slot: u32,
}
```

Production reconfiguration requires careful state-transfer
protocols; we leave this to module 0030 (Disk Paxos) and
module 0033 (Raft, with joint consensus).

## References

- Lamport, Malkhi, Zhou, "Stoppable Paxos", MSR-TR-2008-192.
- Lamport, Malkhi, Zhou, "Vertical Paxos and Primary-Backup
  Replication", PODC 2009.
- Lamport, "Reconfigurable Paxos", 2009.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
