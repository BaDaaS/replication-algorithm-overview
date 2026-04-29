# 0030: Disk Paxos

## Historical context

Gafni and Lamport's 2003 paper "Disk Paxos" (Distributed
Computing) recasts Paxos for shared-disk systems: instead of
acceptors being processes that exchange messages, *acceptors
are disks*. Each process reads from and writes to a set of
shared-storage disks. Communication is via shared memory
rather than messages.

The protocol is structurally identical to classic Paxos but
operationally suited to *disk arrays* and *shared-storage
SANs*, where messaging between processes is expensive but
disk I/O is the natural primitive.

## System and threat model

- **Storage.** A set of `n` disks accessible by all processes.
  Each disk supports atomic block read/write.
- **Processes.** Any number; can crash and recover.
- **Failures.** Up to `f` of the `n` disks may fail (silent
  read errors). Process crashes are unbounded.
- **Goal.** Single-decree consensus among processes via the
  shared disks.

## Theory

### Algorithm

Each process owns a region on each disk where it writes its
state. Other processes read these regions to learn ballot
status.

Per-process per-disk record: `(ballot, accepted_value)`.

```
proposer p, ballot b:
  phase 1:
    write (b, accepted) to my block on every disk
    read all other processes' blocks from a majority of disks
    compute the highest accepted value across reads
  phase 2:
    if my ballot is still the highest in the majority:
      write (b, chosen_value) to my block
    poll majority for confirmation
```

The disk read/write is the analogue of message send/receive.
Quorum intersection on disks (any majority of `n` disks
intersect) gives the same safety property as message-passing
Paxos.

### Theorem (Disk Paxos correctness)

For `n` disks with `f < n / 2` disk failures, Disk Paxos
satisfies Synod safety.

*Proof.* Reduces to Synod: the "promise" and "accepted"
state lives on disk; majority intersection gives the same
guarantees as message-passing Paxos.

### Why it matters

Disk Paxos was the first formalisation of consensus over
shared storage. It informed:

- *SAN-based clustered databases.* Oracle RAC, IBM PowerHA,
  HPE Serviceguard.
- *Shared-disk virtualisation.* VMware vSphere FT.
- *NVMe over Fabrics.* Modern storage networks revisit the
  shared-disk model with much lower latency.

## Practice

### When shared-disk consensus makes sense

- *Pre-existing SAN.* If the deployment already has a
  shared-disk fabric, Disk Paxos avoids extra messaging
  overhead.
- *Latency parity.* When disk I/O is faster than RPC (rare
  in modern cloud, common in HPC).
- *Hardware-redundant disks.* RAID-style disks naturally
  provide partial fault tolerance.

### Modern variants

- *NVMe-over-RDMA.* Disk reads are sub-microsecond; the
  message-vs-disk latency gap closes.
- *Persistent memory consensus.* Persistent-memory devices
  (Intel Optane) offer byte-addressable shared storage; the
  theoretical model resembles Disk Paxos.

## Formalisation aspects

```text
structure DiskPaxosState where
  disks         : Vec DiskBlock
  process_id    : NodeId
  current_ballot: Nat

theorem disk_paxos_safety
    (n f : Nat) (h : 2 * f + 1 = n) :
    SynodSafety := by
  -- Reduce to message-passing Paxos via disk-as-acceptor.
  sorry
```

## Verifiability and circuit encoding

**Tag: `friendly`.**

Verifiable Disk Paxos: each disk's commitment is a Merkle root
of the per-process blocks. A verifiable proof attests that

- The proposer wrote `(b, v)` to its block.
- A majority of disks have the proposer's `(b, v)` block.
- No higher ballot exists on a majority.

Per-decision constraint count: ~10^6 (BLS aggregate of disk
signatures) + ~k (Merkle inclusion proofs).

## Known attacks and limitations

- *Disk fragmentation.* Updates must be atomic at the block
  level; failure mid-write must be handled.
- *Process recovery.* On recovery, a process reads its own
  block to recover its state.
- *Disk failure detection.* Silent corruption requires
  checksums.

## Implementation notes

The crate provides a logical "shared disk" simulator: an
in-memory map representing per-process per-disk blocks, with
atomic read/write semantics. Tests verify decision via shared-
disk reads.

## References

- Gafni and Lamport, "Disk Paxos", Distributed Computing 2003.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
