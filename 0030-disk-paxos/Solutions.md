# Module 0030 Solutions

## Solution 1 [T]: disk-as-acceptor

Mapping:

| Classic Paxos    | Disk Paxos               |
| ---------------- | ------------------------ |
| acceptor's state | disk's per-process block |
| Prepare(b)       | proposer writes new ballot to its block |
| Promise(b, last) | acceptor reads proposer's block, returns its own state |
| Accept(b, v)     | proposer writes (b, v) to its block, polls others |
| Accepted(b, v)   | acceptor reads block, sees the new accept |

The reduction is bijective. Quorum intersection on disks
(majority of n disks) gives the same safety as quorum
intersection on acceptors.

## Solution 2 [P]: NVMe-over-RDMA

Modern shared-disk fabrics (NVMe-oF, Intel Optane, CXL):

| Operation            | Latency       |
| -------------------- | ------------- |
| Local NVMe read      | ~10us         |
| NVMe-oF (RDMA)       | ~5us          |
| Same-DC RPC          | ~50us         |
| Same-DC TCP RPC      | ~100us        |

NVMe-oF is faster than RPC, making Disk Paxos competitive in
modern hardware. Production systems (AWS Aurora, IBM Storage
Scale) have shared-storage consensus internally.

## Solution 3 [F]: pseudo-Lean

```text
structure DiskPaxosState (n_disks : Nat) where
  disks       : Vec DiskBlock n_disks
  process_id  : NodeId
  ballot      : Nat

theorem disk_paxos_safety
    (n_disks : Nat) (f : Nat) (h : 2 * f + 1 = n_disks) :
    forall (b1 b2 : Nat) (v1 v2 : Value),
      DecidedAt b1 v1 -> DecidedAt b2 v2 -> v1 = v2 := by
  -- Reduce to classic Paxos: each disk is an acceptor.
  apply synod_safety
```

## Solution 4 [V]: verifiable shared-disk

Verifier's check:

- Public input: Merkle root of all disk blocks.
- Witness: Merkle path for the proposer's block at majority of
  disks, plus the (ballot, value) tuple.
- Constraint: each Merkle path verifies; the (ballot, value)
  is the highest among the read blocks.

Per-decision constraints: ~k * log(n) hash invocations for
the Merkle paths (~k * 200 in Poseidon for k = 10 disks) +
~10^4 for the highest-ballot comparison.

Total: ~10^4 constraints per Disk Paxos decision, much
smaller than the BLS-aggregate of message-passing Paxos. The
shared-disk model is naturally Merkle-friendly.
