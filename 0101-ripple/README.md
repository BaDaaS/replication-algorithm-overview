# 0101: Ripple Protocol Consensus Algorithm

## Historical context

David Schwartz, Noah Youngs, and Arthur Britto published "The
Ripple Protocol Consensus Algorithm" (RPCA) in 2014 (Ripple
Labs white paper). RPCA is a federated-trust consensus protocol
similar to Stellar SCP (module 0100) but predates it. Each
Ripple validator maintains a *Unique Node List* (UNL): a set
of other validators it trusts.

Like SCP, Ripple is a federated approach: trust is per-node,
not global. Unlike SCP, Ripple defines a fixed quorum threshold
of *80%* on each UNL (i.e., 4/5 of the UNL must agree). Ripple
requires the UNLs of any two correct nodes to overlap by at
least 90%, a strong condition compared to SCP's flexible
quorum-intersection.

The XRP Ledger (Ripple's blockchain) has been running RPCA
since 2012 and is the longest-deployed federated-BFT protocol.

## System and threat model

- **Network.** Asynchronous in safety; partially synchronous
  for liveness.
- **Failures.** Byzantine; assumed `f < 1/5` of UNL faulty.
- **Cryptography.** Standard ECDSA signatures; later moved to
  Ed25519.
- **Goal.** Fast (~3-5 second) consensus on transaction sets.

## Theory

### Unique Node List (UNL)

Each validator's UNL is its trusted set: the validators whose
votes it accepts. Recommended UNLs are published by Ripple Labs
and a few independent operators, but each validator may
configure their own.

### Round structure

RPCA runs in rounds (~3-5 seconds each):

1. *Open phase.* Validators propose transactions to include.
2. *Iterative voting.* Validators broadcast their proposed
   transaction set; iteratively prune to reach 80% UNL
   agreement.
3. *Validation.* Validators sign the agreed ledger.
4. *Ledger close.* When 80% of UNL has signed the same ledger,
   it is closed.

### 80% quorum

The 4/5 threshold (rather than PBFT's 2/3) is more conservative.
Ripple argues it provides extra safety margin against
adversarial UNL configuration; some critics argue it makes
liveness harder.

### UNL overlap requirement

Ripple's safety theorem requires UNLs of any two correct nodes
to overlap by at least 90%. This is stricter than SCP's quorum-
intersection requirement and effectively means the UNLs are
near-identical.

### Comparison: SCP, RPCA, PBFT

| property              | RPCA       | SCP         | PBFT       |
| --------------------- | ---------- | ----------- | ---------- |
| trust unit            | UNL        | quorum slice | global committee |
| quorum threshold      | 80% UNL    | per-node    | 2f + 1     |
| safety condition      | UNL overlap >= 90% | quorum intersection | f < n/3 |
| open membership       | partial    | yes         | no         |
| convergence speed     | seconds    | seconds-min | seconds    |
| open critique         | tier-1 centralised | tier-1 centralised | n/a |

### Properties

- *Fast convergence* (3-5 seconds in production).
- *Federated trust.*
- *Conservative quorum* (80%).
- *Strict UNL overlap* requirement.

### Limitations

- *UNL homogeneity.* In practice, all major validators run
  near-identical UNLs (recommended by Ripple Labs).
- *Centralisation.* Ripple Labs and a few foundations
  effectively control validator selection.
- *No formal proof.* RPCA's white paper offers an informal
  analysis; later work (e.g., Chase-MacBrough 2018) showed
  the original safety proof was incomplete.

### Cobalt: Ripple's improved variant

Mauro Conti, James Drummond, and others showed RPCA's safety
proof had gaps; Ripple Labs published *Cobalt* (2019), an
improved variant with formal safety. Cobalt is what runs in
production today.

## Practice

- *XRP Ledger.* Production since 2012. Cross-border payments
  and asset issuance.
- ~150 validators with overlapping UNLs.
- Block time ~3-5 seconds.
- Throughput ~1500 tx/s.

### Production-implementation notes

- *UNL recommendation.* Ripple Labs publishes a recommended UNL
  every quarter; most validators copy it.
- *Validator diversity.* Despite the recommendation, no single
  organisation controls more than a few validators in the
  default UNL.
- *Ledger close timing.* Validators have soft deadlines to
  declare a ledger closed; if 80% don't agree by deadline,
  they propose alternative ledgers.

## Verifiability and circuit encoding

**tag: `partial`.**

RPCA circuits are similar to SCP's: federated voting plus
threshold-signature verification. Cost depends on UNL size
(typically ~150) and signature scheme.

A SNARK-friendly variant could use BLS aggregate signatures and
verify the 80% threshold in `O(log n)` constraints.

## Known attacks and limitations

- *Centralised UNL configuration.* Ripple Labs's recommended
  UNL is effectively the de facto trust set.
- *Validator forks.* In practice, multiple "trusted" UNLs can
  diverge if validators don't update simultaneously.
- *Liveness with stale UNLs.* If UNL composition changes faster
  than validators update, consensus can stall.

## References

- Schwartz, Youngs, Britto, "The Ripple Protocol Consensus
  Algorithm", Ripple Labs white paper, 2014.
- Chase, MacBrough, "Analysis of the XRP Ledger Consensus
  Protocol", arXiv 1802.07242, 2018 (introduces Cobalt).
- MacBrough, "Cobalt: BFT Governance in Open Networks",
  arXiv 1802.07240, 2018.

## Implementation notes

The crate provides `RippleNode` with a UNL and a `vote_pass`
function checking whether a proposed ledger has 80% UNL
agreement. Tests verify the 80% threshold computation.

See also [`HISTORY.md`](../HISTORY.md), section "2009 to 2014".
