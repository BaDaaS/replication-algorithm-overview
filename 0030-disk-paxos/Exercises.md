# Module 0030 Exercises

## Exercise 1 [T]: disk-as-acceptor reduction

Show formally that a disk in Disk Paxos plays the same role as
an acceptor in classic Paxos. Identify the equivalent of
"promise" and "accept" messages.

## Exercise 2 [P]: NVMe-over-RDMA

Discuss the modern relevance of Disk Paxos: with NVMe-over-
RDMA, disk reads are sub-microsecond. Compare to RPC latency
(typically ~10us in same-DC).

## Exercise 3 [F]: pseudo-Lean Disk Paxos

Define `DiskPaxosState` with disks and process states. State
the safety theorem.

## Exercise 4 [V]: verifiable shared-disk

A verifiable Disk Paxos commits to a Merkle root of all disk
blocks. Sketch the verifier's check.
