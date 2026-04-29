# 0040: Rampart

## Historical context

Reiter's 1994 CCS paper "The Rampart Toolkit for Building
High-Integrity Services" introduced the first practical
Byzantine-fault-tolerant group communication system. Rampart
provided reliable broadcast, atomic broadcast, and group
membership in the Byzantine model, all in user-space C
libraries.

Rampart predates PBFT (1999) by five years and is the first
practical implementation of Byzantine consensus that achieved
acceptable performance on commodity hardware of the time.

## System and threat model

- **Network.** Asynchronous (with eventual delivery),
  authenticated.
- **Failures.** Byzantine, `f < n / 3`.
- **Cryptography.** Public-key signatures (RSA in the
  original).
- **Goal.** Byzantine atomic broadcast and group membership.

## Theory

### Architecture

Rampart is a *toolkit* with three layers:

1. *Group communication.* Reliable, FIFO, causal, atomic
   broadcast primitives.
2. *Membership service.* Detects faulty members; manages
   group view changes.
3. *Replication service.* Application-level replication
   built atop the primitives.

### Atomic broadcast

Reiter's atomic broadcast uses a designated *sequencer* that
orders messages and signs the order. Other replicas verify
and broadcast acknowledgements. The protocol is structurally
similar to PBFT's pre-prepare/prepare/commit but with a
single sequencer and signature-based ordering.

```
sender -> sequencer: send (msg, payload)
sequencer -> all: order (msg, seq_num, sender)
each replica -> all: ack (seq_num)
on f + 1 acks: deliver
```

### Group membership

Rampart's membership service detects faulty members via
timeouts and signed evidence. A view change triggers when
the sequencer is suspected; new sequencer is the next in
round-robin order.

### Theorem (Rampart correctness)

Atomic broadcast: under `f < n / 3` Byzantine and eventual
delivery, all honest replicas deliver the same set of
messages in the same order.

Membership: an unsuspected (eventually-honest) member is
not removed.

### Performance

Reiter reports throughput of `~100` ops/sec on early-1990s
hardware, with `~10ms` per-op latency for small messages.
Modest by modern standards but a significant achievement at
the time.

## Practice

### Rampart's legacy

- *PBFT (1999).* Castro-Liskov drew explicitly on Rampart's
  design choices, particularly the membership-aware view-
  change. PBFT's improvements: better throughput, MAC-based
  authentication (faster than RSA signatures).
- *SINTRA (2001).* Cachin-Kursawe's threshold-cryptography-
  based BFT inspired by Rampart's structure.
- *Modern BFT.* CometBFT, HotStuff, Tendermint inherit
  Rampart's "evidence-based suspicion" pattern for
  validator slashing.

### Why Rampart matters today

- *Toolkit pattern.* Modular layering of group communication,
  membership, replication. Modern BFT platforms (BFT-SMaRt,
  Aptos's components) follow the same architecture.
- *Public-key cryptography.* The first BFT system to use
  public-key signatures for accountability rather than
  shared-key MACs.

## Formalisation aspects

```text
class RampartGroupCommunication where
  reliable_broadcast : ...
  atomic_broadcast   : ...
  membership         : ...

theorem rampart_atomic_broadcast :
    AtomicBroadcast := by
  -- Use the sequencer + ack quorum.
  sorry
```

## Verifiability and circuit encoding

**Tag: `friendly`.**

Rampart's signature-based atomic broadcast is naturally
verifiable. Per-broadcast:

- Sequencer signature: ~3k constraints.
- f + 1 ack signatures (BLS-aggregated): ~10^6.

Total: ~10^6 per delivery. Standard for BFT.

## Known attacks and limitations

- *Sequencer bottleneck.* All ordering through one
  sequencer. Multi-sequencer designs (HotStuff's leader
  rotation) address this.
- *RSA signature cost.* Rampart's original implementation
  was signature-bound. Modern variants use ed25519 or BLS.

## Implementation notes

The crate is conceptual; full Rampart is in PBFT (module
0042). This module provides reading-list pointers.

## References

- Reiter, "The Rampart Toolkit for Building High-Integrity
  Services", CCS 1994.
- Reiter, "Secure Agreement Protocols: Reliable and Atomic
  Group Multicast in Rampart", CCS 1994.

See also [`HISTORY.md`](../HISTORY.md), section "1986 to 1999".
