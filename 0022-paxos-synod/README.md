# 0022: Paxos Synod (Lamport 1998)

## Historical context

Leslie Lamport submitted "The Part-Time Parliament" to ACM TOCS
in 1990; it was rejected for being too whimsical, sat in
limbo, and was finally published in 1998 (TOCS 16, 2). The
paper described the *Synod* protocol (single-decree Paxos)
through a parable about archaeologists reconstructing the
parliamentary procedures of a fictitious ancient Greek island.

The Synod protocol decides on a *single value* via a sequence
of *ballots*. Multi-Paxos (module 0023) extends Synod to a
sequence of decisions, giving full SMR. Paxos became the
canonical fault-tolerant agreement protocol of the 2000s and
the structural ancestor of every later SMR system (VR, Raft,
HotStuff, Tendermint).

## System and threat model

- **Network.** Asynchronous, reliable channels.
- **Failures.** Crash-recovery with stable storage; `f < n / 2`.
- **Cryptography.** Authenticated channels (no signatures
  required for Paxos's correctness).
- **Goal.** Agreement on a single value among `n` acceptors.

## Theory

### Roles

Every Paxos process can play one or more roles:

- *Proposer.* Initiates a ballot proposing a value.
- *Acceptor.* Votes on proposals and stores accepted values.
- *Learner.* Observes accepted decisions.

In production, a single process typically plays all three.

### Algorithm: Synod (single-decree)

Each ballot has a unique number `b`. A proposer with ballot `b`:

```
phase 1 (prepare):
  proposer -> all acceptors: PREPARE(b)
  acceptor: if b > any seen ballot: respond PROMISE(b, last_v_a, last_b_a)
            else: ignore (or NACK)
  proposer: collect a majority of PROMISE responses

phase 2 (accept):
  proposer: if any PROMISE had a non-null last_v_a:
              v := the last_v_a from the highest last_b_a
            else: v := the proposer's preferred value
  proposer -> all acceptors: ACCEPT(b, v)
  acceptor: if b is still the highest seen: respond ACCEPTED(b, v),
            stable-store (b, v)
  proposer: collect a majority of ACCEPTED responses; v is decided
```

### Theorem (Paxos Synod safety)

For any two ballots `b_1 < b_2` that succeed in deciding values
`v_1, v_2`, `v_1 = v_2`.

*Proof.* The Phase-2 quorum of `b_1` (majority of acceptors)
intersects the Phase-1 quorum of `b_2` (also majority). Some
acceptor `a` is in both. `a` accepted `(b_1, v_1)` in phase 2
of `b_1`. In phase 1 of `b_2`, `a` reports `last_v_a = v_1`
(or higher). The proposer of `b_2` therefore proposes `v_1` in
phase 2. So `v_2 = v_1`. QED.

### Theorem (Paxos liveness, under partial synchrony)

Under partial synchrony with at most `f < n / 2` crashes,
Paxos eventually decides a value, provided some proposer keeps
trying ballots with sufficiently high numbers.

The "single proposer eventually" condition is a leader
election; it is the FLP-escape mechanism.

### Why it works: invariants

The proof rests on three invariants:

1. *Monotone ballot progression.* Each acceptor only ever
   responds to ballots higher than any it has seen.
2. *Promise prevents conflicting accept.* Once `a` promises
   ballot `b`, it refuses any `accept(b', v')` with `b' < b`.
3. *Accepted values propagate forward.* Phase 1 of a higher
   ballot extracts the accepted value from the last successful
   ballot.

These three invariants give the safety theorem above.

### Comparison with VR

VR (module 0020) is a primary-backup framing of essentially
the same protocol:

- Paxos's "ballot" = VR's "view".
- Paxos's "proposer" = VR's "primary".
- Paxos's "phase 1" = VR's "view change".
- Paxos's "phase 2" = VR's "normal-case operation".

The technical content is identical; the exposition differs.

## Practice

### Real Paxos deployments

- *Google Chubby* (2006). Lock service over Paxos. The
  reference deployment.
- *ZooKeeper* (2008). ZAB is structurally a Paxos variant
  (see module 0029).
- *etcd* (2014). Initially Paxos-style, switched to Raft.
- *Spanner* (2012). Paxos per replica group.

### "Paxos Made Live" (Chandra-Griesemer-Redstone 2007)

A widely-cited paper documenting the engineering challenges of
implementing Paxos in Google Chubby. Issues addressed:

- *Disk failure.* Stable storage may corrupt; need checksums
  and recovery.
- *Concurrent ballots.* Multiple proposers can prevent
  progress; need leader election.
- *Snapshot management.* The log grows unboundedly; need
  snapshots.
- *Performance.* Naive Paxos has high per-operation latency;
  need pipelining (Multi-Paxos).

### Why "Paxos is hard" folklore

Paxos's reputation for being hard to understand drove Ongaro
and Ousterhout's Raft (2014, module 0028). The mathematical
content is the same; Raft's contribution is exposition and
explicit state-machine design.

## Formalisation aspects

```text
structure SynodAcceptor where
  promised  : Option Nat             -- highest seen ballot
  accepted  : Option (Nat × Value)   -- (ballot, value)

structure SynodProposer where
  ballot    : Nat
  value     : Value
  promises  : Set (NodeId × Option (Nat × Value))
  accepteds : Set NodeId

theorem synod_safety :
    forall (b1 b2 : Nat) (v1 v2 : Value),
    Decided b1 v1 -> Decided b2 v2 -> v1 = v2 := by
  -- quorum intersection + invariant 3
  sorry
```

The Paxos Made Simple paper has been formalised in TLA+ (by
Lamport himself) and in Coq (IronFleet, Velisarios). Lean
formalisations are work in progress.

## Verifiability and circuit encoding

**Tag: `friendly`.**

A verifiable Paxos commit:

- Phase-1 quorum certificate (BLS-aggregated promises): one
  pairing, ~10^6 constraints.
- Phase-2 quorum certificate (BLS-aggregated accepteds):
  another pairing, ~10^6.

Total per commit: ~2 * 10^6 constraints (two pairings).
Multi-Paxos amortises Phase 1 across many commits, dropping
the per-commit cost to ~10^6.

Production: Aptos's DiemBFT (a HotStuff descendant) uses
Multi-Paxos-style amortisation; the QC structure is
SNARK-friendly.

## Known attacks and limitations

- *Dueling proposers.* Two proposers can repeatedly outbid
  each other, preventing progress. Mitigation: leader election
  (Multi-Paxos's "distinguished proposer").
- *Stable-storage assumption.* Acceptor must persist
  `(promised, accepted)` before responding. Production
  systems use fsync'd write-ahead logs.
- *Single-decree limitation.* Synod decides on one value;
  practical SMR needs many decisions. Multi-Paxos addresses
  this (module 0023).

## Implementation notes

The crate provides a Synod simulator with three nodes (all
proposer-acceptors). One initial proposer drives a ballot;
acceptors respond. Test verifies decision on the proposer's
preferred value when all acceptors are alive.

A second test demonstrates "dueling proposers" by having two
proposers race; the test shows that only one of them
eventually decides.

## References

- Lamport, "The Part-Time Parliament", TOCS 16(2) 1998.
- Lamport, "Paxos Made Simple", SIGACT News 2001.
- Chandra, Griesemer, Redstone, "Paxos Made Live: An
  Engineering Perspective", PODC 2007.
- van Renesse and Altinbuken, "Paxos Made Moderately Complex",
  ACM Computing Surveys 2015.

See also [`HISTORY.md`](../HISTORY.md), section "1986 to 1999".
