# 0041: SecureRing

## Historical context

Kihlstrom, Moser, and Melliar-Smith published "The
SecureRing Group Communication System" at HICSS 1998.
SecureRing extended the Totem ring-based group communication
protocol with Byzantine fault tolerance, becoming an
alternative to Rampart with a different topological
choice (ring instead of all-to-all).

The ring topology has bandwidth advantages for large groups
but introduces single-point bottlenecks per ring rotation.

## System and threat model

- **Network.** Asynchronous, FIFO; eventual delivery.
- **Failures.** Byzantine, `f < n / 3`.
- **Topology.** Logical ring of `n` replicas.
- **Cryptography.** Signatures.
- **Goal.** Byzantine atomic multicast.

## Theory

### Token-based ordering

A *token* circulates around the ring. Whoever holds the token
is the current sequencer; they assign sequence numbers to
pending messages and broadcast them.

```
on receiving token from previous-in-ring:
  for each pending local message m:
    assign sequence_num
    broadcast (m, seq, signed_by_self)
  pass token to next-in-ring
```

Each replica verifies signatures and adds the message to its
local log.

### View change

When the token-holder fails or the ring is partitioned,
SecureRing runs a view-change protocol:

- Suspect the token-holder via timeout.
- Broadcast suspicion-evidence to all replicas.
- Form a new ring excluding the suspected member.

### Theorem (SecureRing correctness)

Under `f < n / 3` Byzantine and eventual delivery,
SecureRing's atomic multicast satisfies validity, integrity,
agreement, and total order.

### Performance

- *Throughput.* Limited by token-rotation rate.
- *Latency.* `O(n)` per delivery (token must reach the
  sender's position).
- *Bandwidth.* Lower than all-to-all schemes for large `n`.

## Practice

### Where SecureRing's ideas show up

- *Totem.* The non-Byzantine predecessor; widely deployed.
- *RDS / RDMA-based group communication.* Modern variants
  use RDMA broadcast over a logical ring.
- *Production BFT.* Most production BFT (PBFT, HotStuff,
  Tendermint) use all-to-all topologies, not rings. Rings
  resurface in some SAN-based deployments.

### Why rings persist

Ring topologies are bandwidth-efficient for large groups
(linear, not quadratic). For groups of `n > 50`, ring
overhead is competitive.

## Formalisation aspects

```text
structure RingState where
  token_holder : NodeId
  ring_order   : List NodeId
  log          : List (Seq × Message)

theorem securering_atomic :
    AtomicBroadcast := by
  -- Token ensures unique ordering authority.
  sorry
```

## Verifiability and circuit encoding

**Tag: `friendly`.**

Per-message: signed by token-holder, plus a chain of
signatures attesting ring rotation. ~3k constraints per
signature; for a ring of 5: ~15k constraints per delivery.

## Known attacks and limitations

- *Token loss.* If the token is lost (sender crashes after
  receiving), the protocol must regenerate the token via
  view change.
- *Ring partition.* Split rings violate single-token
  invariant; view-change resolves.

## Implementation notes

Conceptual module; full ring implementation is non-trivial
and beyond per-module scope.

## References

- Kihlstrom, Moser, Melliar-Smith, "The SecureRing Group
  Communication System", HICSS 1998.
- Amir et al., "The Totem Single-Ring Ordering and
  Membership Protocol", TOCS 1995.

See also [`HISTORY.md`](../HISTORY.md), section "1986 to 1999".
