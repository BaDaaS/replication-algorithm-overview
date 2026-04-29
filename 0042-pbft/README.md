# 0042: PBFT -- Practical Byzantine Fault Tolerance

## Historical context

Castro and Liskov's "Practical Byzantine Fault Tolerance"
(OSDI 1999, journal TOCS 2002) was the breakthrough that
made BFT consensus practical: throughput within an order of
magnitude of unreplicated systems, on commodity hardware,
under the partial-synchrony model. PBFT is the structural
ancestor of every later production BFT (Tendermint, HotStuff,
DiemBFT, Aptos's consensus).

The 1999 OSDI paper is one of the most-cited systems papers
in distributed systems. Every modern BFT protocol is a
delta against PBFT.

## System and threat model

- **Network.** Partially synchronous (DLS Variant B).
- **Failures.** Byzantine, `f < n / 3`.
- **Cryptography.** Authenticated channels (MAC vectors in
  the original; signatures in modern variants).
- **Goal.** Linearisable SMR.

## Theory

### Three phases

PBFT runs three phases per request:

1. *Pre-prepare.* The primary assigns a sequence number `n`
   to the request and broadcasts `<PRE-PREPARE, v, n, m>`.
2. *Prepare.* Each replica that accepts the pre-prepare
   broadcasts `<PREPARE, v, n, d, i>` (`d` = digest of `m`,
   `i` = replica id).
3. *Commit.* On collecting `2f + 1` matching prepares
   (including its own), each replica broadcasts `<COMMIT, v,
   n, d, i>`. On collecting `2f + 1` matching commits, the
   replica executes `m`.

The `2f + 1` quorum (`prepare-certificate`) is the load-
bearing object: any two prepare-certificates intersect in at
least one honest replica, ensuring consistency.

### View change

When the primary is suspected, replicas trigger a view
change to view `v + 1`:

```
each replica -> all: <VIEW-CHANGE, v + 1, n, P, i>
  where P is the set of prepare-certificates the replica has
  collected since the last stable checkpoint.

new primary of view v + 1: collects 2f + 1 view-change
  messages, computes the new pre-prepare set, broadcasts
  <NEW-VIEW, v + 1, V, O>.
```

The view-change protocol is the most subtle part of PBFT;
its correctness ensures that committed requests are
preserved across view changes.

### Theorem (PBFT correctness)

PBFT satisfies linearisable SMR under partial synchrony with
`f < n / 3` Byzantine faults.

*Proof.* Castro-Liskov 1999, Section 4.4. Outline:

- *Safety.* Two `2f + 1`-quorums in `n = 3f + 1` intersect
  in `f + 1`, of which at least 1 is honest. So no two
  conflicting commits can be reached.
- *Liveness.* After GST, view changes terminate; an honest
  primary's pre-prepares reach all honest replicas in
  bounded time.

### Optimisations

- *Tentative execution.* Replicas execute the request
  speculatively after `2f` prepares (one short of the full
  quorum), reverting if the commit phase fails.
- *Read-only optimisation.* Read-only requests skip the
  three-phase exchange when the cluster is stable.
- *MAC-based authentication.* The 1999 paper uses MACs (per-
  pair shared keys) instead of signatures, an order of
  magnitude faster than RSA at the time.
- *Batching.* The primary batches multiple client requests
  into one pre-prepare.

### Complexity

- *Messages per request:* `O(n^2)` (each replica broadcasts
  prepare and commit).
- *Authenticator complexity:* `O(n^2)` MACs or signatures.
- *Round complexity:* 3 phases (constant).

Modern variants (HotStuff) reduce the message complexity to
`O(n)` per phase using BLS aggregation.

## Practice

### Production implementations

- *BFT-SMaRt* (module 0054). Modern PBFT-equivalent in Java.
- *Tendermint / CometBFT.* PBFT-derived, consensus engine for
  Cosmos. Adds explicit weighted voting for PoS.
- *Hyperledger Fabric.* Optional BFT ordering service
  PBFT-derived.
- *Aptos / Sui.* HotStuff-family descendants.
- *PBFT-Go.* Reference implementation in Go.

### Why PBFT was a breakthrough

Pre-PBFT BFT (Rampart, SecureRing) ran at ~100 ops/sec.
PBFT's MAC-based authentication and tentative execution
brought throughput to ~10000 ops/sec, making BFT practical
for production storage systems.

### Castro-Liskov journal version (TOCS 2002)

Adds:

- Asynchronous checkpointing.
- Detailed proof of safety and liveness.
- Performance evaluation on the BFS distributed file system.

## Formalisation aspects

PBFT has been formally verified in Coq (Velisarios, Rahli
2018), in Isabelle/HOL (Berkovits-Lazic-Losa-Padon-Shoham
2019, "Verification of Threshold-Based Distributed
Algorithms by Decomposition to Decidable Logics"), and in
TLA+ (Lamport-Lampson 2007).

```text
structure PbftState where
  view         : Nat
  sequence     : Nat
  log          : List Request
  prepares     : Map (View, Seq) (Set NodeId)
  commits      : Map (View, Seq) (Set NodeId)

theorem pbft_safety
    (n f : Nat) (h : 3 * f + 1 = n)
    (sched : PartiallySynchronousSchedule)
    (corrupt : ByzantineCorruption f) :
    Linearisable PBFT sched corrupt := by
  -- 2f + 1 quorum intersection.
  sorry
```

## Verifiability and circuit encoding

**Tag: `friendly`.**

Per request, with BLS aggregation:

- Pre-prepare: 1 BLS sig from primary, ~3k constraints.
- Prepare cert: 1 BLS aggregate from `2f + 1` replicas,
  ~10^6 constraints.
- Commit cert: same.

Total per request: ~2 * 10^6 + 3k = ~2 * 10^6 constraints.

The view-change cert: BLS aggregate of `2f + 1` view-change
messages, ~10^6 per view change.

Production verifiable BFT (Aptos, Sui, Mina-bridge proofs)
uses this template directly.

## Known attacks and limitations

- *Quadratic message complexity.* `O(n^2)` per request.
  HotStuff (module 0055) reduces to `O(n)`.
- *View-change cost.* `O(n^2)` per view change. PBFT's view-
  change message includes prepare-certificates back to the
  last stable checkpoint, which can be large.
- *Adversarial slowdown.* A malicious primary can slow the
  protocol to its timeout limit without triggering a view
  change. Aardvark (module 0046) addresses this.

## Implementation notes

The crate provides a minimal three-phase PBFT simulator:

- 4 replicas (`f = 1`).
- Static primary at NodeId(0).
- Pre-prepare/prepare/commit flow.

Tests verify all replicas commit the same operation sequence.

## References

- Castro and Liskov, "Practical Byzantine Fault Tolerance",
  OSDI 1999.
- Castro and Liskov, "Practical Byzantine Fault Tolerance
  and Proactive Recovery", TOCS 2002.
- Rahli et al., "Velisarios: Byzantine Fault-Tolerant
  Protocols Powered by Coq", ESOP 2018.

See also [`HISTORY.md`](../HISTORY.md), section "1986 to 1999".
