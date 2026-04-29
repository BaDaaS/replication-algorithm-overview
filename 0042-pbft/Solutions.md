# Module 0042 Solutions

## Solution 1 [T]: PBFT safety

Suppose two honest replicas commit different operations
`op_1, op_2` at sequence `n` in view `v`.

Each commit requires a 2f + 1 commit-cert. By prepare-cert
intersection: each commit-cert requires a 2f + 1 prepare-
cert; two such certs in `n = 3f + 1` intersect in `f + 1`,
of which at least 1 is honest. The honest replica only
prepares one value per (v, n), so the two commit-certs share
at least one honest "preparer" who prepared both `op_1` and
`op_2`. Contradiction.

QED.

## Solution 2 [T]: PBFT liveness

After GST, all messages between honest replicas arrive
within bound `D`. View-change timeouts are tuned to be
larger than `2D`. So:

- Honest primary's pre-prepares reach all honest replicas
  within `D`.
- Prepare-certs form within `2D`.
- Commit-certs form within `3D`.

If primary is faulty: view-change triggered by `5D`-style
timeouts; new primary (eventually honest by round-robin)
makes progress.

Castro-Liskov 1999 Theorem 3.

## Solution 3 [P]: view change

Add VIEW_CHANGE message carrying prepare-certificates from
the last stable checkpoint. New primary collects 2f + 1
view-change messages, computes new pre-prepare set
preserving committed operations, broadcasts NEW_VIEW.

Test: kill primary at time T; verify a new primary takes
over and continues the log.

## Solution 4 [P]: equivocating primary

Byzantine primary sends pre-prepare(op_1) to replicas {1, 2}
and pre-prepare(op_2) to replicas {3, ...}. In the prepare
phase:

- Replicas 1, 2 broadcast prepare(op_1).
- Replicas 3, ... broadcast prepare(op_2).
- No 2f + 1 prepare-cert forms for either value (in n = 4,
  f = 1, need 3 prepares but only have 2 + 2).

The protocol stalls; view-change triggers, new primary
takes over.

PBFT detects the equivocation indirectly via the missing
prepare-cert. To detect explicitly, replicas can exchange
prepares and present two-pre-prepare evidence as proof of
primary equivocation.

## Solution 5 [F]: pseudo-Lean PBFT

Velisarios's Coq formalisation models PBFT as an LTS with
~5000 lines of Coq, proving safety and liveness. A Lean
port would reuse cslib's `LTS` framework.

```text
structure PbftState where
  view         : Nat
  log          : List (View × Seq × Op)
  prepares     : Map (View × Seq × Op) (Set NodeId)
  commits      : Map (View × Seq × Op) (Set NodeId)
  status       : Status

theorem pbft_safety
    [HonestQuorum f n] [BLS Sig] :
    forall (sched : PartiallySync) (corrupt : Byz f),
    SmrSafety := by
  sorry
```

## Solution 6 [V]: zk-PBFT

Per request, with BLS aggregation:

- Pre-prepare signature: ~3k.
- Prepare cert: ~10^6.
- Commit cert: ~10^6.

Total: ~2 * 10^6 per request.

HotStuff (module 0055) reduces this to ~10^6 per request via
a chained QC pipeline. The structural improvement: HotStuff
collapses the three phases into a single QC chain, so each
"phase" amortises across many requests.

## Solution 7 [P]: MAC vs signature

| Property            | MAC         | Signature       |
| ------------------- | ----------- | --------------- |
| Speed (1999 hw)     | ~us         | ~ms             |
| Accountability      | shared key  | non-repudiable  |
| Verifiability (zk)  | hard        | natural         |
| Storage             | n keys      | 1 key           |

PBFT 1999 used MACs for speed. Modern variants use Ed25519
or BLS, taking advantage of hardware acceleration. For
verifiability and slashing, signatures are essential.
