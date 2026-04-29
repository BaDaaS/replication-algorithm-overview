# Module 0029 Solutions

## Solution 1 [T]: composition safety

Master SMR safety: all honest replicas agree on the sequence
of configurations and their start slots.

Per-configuration data SMR safety: within configuration `C_i`
(for slots `[start_i, start_{i+1})`), the data SMR's quorum
intersection ensures decisions are unique.

Global safety: any committed slot `s` lies within exactly one
configuration `C_i`. By data SMR safety in `C_i`, the
decision at `s` is unique. By master SMR safety, all replicas
agree this slot is in `C_i`. So the decision is globally
unique.

## Solution 2 [P]: install protocol

```
master commits new config (C_{i+1}, start_slot s + 1)

old replicas: stop accepting proposals for slots > s
new replicas: contact old quorum to fetch state up to slot s
old replicas: respond with snapshot + log entries [..s]
new replicas: install snapshot, replay log
new replicas: begin Phase 2 for slot s + 1 onwards
```

State transfer is the operationally tricky part: snapshots
must include all committed state, the log must include any
in-flight unaccepted operations, and the new replicas must
verify integrity (e.g. via Merkle hashes).

Production: etcd's snapshot transfer + Raft log catch-up
handles this; Vertical Paxos formalises a similar pattern.

## Solution 3 [F]: pseudo-Lean Configuration

```text
structure Configuration where
  epoch       : Nat
  replicas    : Finset NodeId
  start_slot  : Nat

inductive ConfigSequence where
  | nil
  | cons (cfg : Configuration) (rest : ConfigSequence)
    (h : forall earlier in rest, earlier.epoch < cfg.epoch /\
                                  earlier.start_slot < cfg.start_slot)

theorem master_smr_safety
    (master : MasterSmr) (sched : AsyncSched) :
    forall (i j : Nat),
      ConfigAt master i sched = some C_i ->
      ConfigAt master j sched = some C_j ->
      i = j -> C_i = C_j := by
  -- Master SMR is itself an SMR; per-slot synod safety.
  sorry
```

## Solution 4 [V]: cross-config proof

A chain proof for slots `s_1, ..., s_k` crossing configuration
boundaries:

- For each slot, public input includes the configuration ID
  valid at that slot.
- The configuration ID is itself a chain commitment (Merkle
  root of the configuration sequence).
- The verifier checks two things per slot: (i) the slot's
  data proof is valid against the named config; (ii) the
  named config is in the master's chain.

The recursive SNARK combines these checks across slots; final
proof is O(1).

Production analogue: Ethereum's beacon-chain validator-set
rotation. Each block's data proofs reference the validator
set committed in some prior epoch. The L1 contract checks the
validator-set commitment matches the chain's history.
