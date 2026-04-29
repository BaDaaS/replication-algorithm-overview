# 0100: Stellar Consensus Protocol (SCP)

## Historical context

David Mazieres published "The Stellar Consensus Protocol: A
Federated Model for Internet-level Consensus" in 2015 (Stellar
Development Foundation white paper). SCP introduces *Federated
Byzantine Agreement* (FBA): a consensus model in which each
node chooses its own *quorum slice* (a set of nodes whose
agreement it accepts) rather than relying on a globally agreed
quorum size.

Federated quorums break the symmetry of classical BFT: in PBFT
(module 0042), the quorum size `2f + 1` is fixed for the entire
system; in FBA, each node has its own definition of "enough
agreement", and the system-wide consensus emerges from the
intersection structure.

This was a significant philosophical shift. Classical BFT
assumes a *closed* committee with known identities and known
fault threshold; PoW assumes an *open* anonymous network. SCP
sits between: nodes choose whom to trust, and the global
consensus property holds *as long as the chosen trust graph
satisfies certain intersection conditions*.

## System and threat model

- **Network.** Asynchronous (FLP-resilient via federated
  voting).
- **Failures.** Byzantine; fault tolerance depends on the
  individual quorum-slice configurations.
- **Cryptography.** Standard signatures.
- **Goal.** Open membership with self-chosen trust graph.

## Theory

### Quorum slice

Each node `v` chooses a set of *quorum slices* `Q(v)`. A
quorum slice for `v` is a set of nodes whose unanimous
agreement `v` accepts as binding. `Q(v)` may be a list of
multiple slices: `v` accepts agreement if *any* of its slices
agrees.

A *quorum* is a set `U` such that every node `v` in `U` has at
least one slice `S` in `Q(v)` with `S` contained in `U`. Quorums
are *self-justifying*: every member's slice is fully inside the
quorum.

### Quorum intersection

The key property: any two quorums must intersect (modulo faulty
nodes). Without this, the system can reach divergent decisions.
SCP proves: if the quorum-intersection property holds despite
Byzantine nodes, then SCP's federated voting achieves safety.

### Federated voting (nomination + ballot)

SCP runs in two layers:

1. *Nomination.* Nodes propose values; nomination converges to
   a "preferred" value via local voting on quorum slices.
2. *Ballot.* PBFT-style three-phase commit on the preferred
   value, but using federated quorums.

### Comparison: SCP, PBFT, Tendermint

| property               | PBFT (module 0042) | SCP            | Tendermint (0055) |
| ---------------------- | ------------------ | -------------- | ----------------- |
| committee              | fixed              | self-organising | fixed            |
| quorum                 | global `2f + 1`    | per-node       | global `2f + 1`   |
| open membership        | no                 | yes            | no                |
| safety condition       | `f < n/3`          | quorum-intersection | `f < n/3`    |
| network model          | partial sync       | async          | partial sync      |
| primary used by        | banks, internal    | Stellar, Ripple | Cosmos           |

SCP's federated model trades the simplicity of a global quorum
for flexibility in trust topology.

### Properties

- *Open membership.* Anyone can join with their own slice
  configuration.
- *Personalised trust.* Each node chooses whom to trust.
- *Async safety.* No timing assumption needed for safety.
- *Quorum-intersection requirement.* For consensus, the
  participants' slices must overlap appropriately.

### Limitations

- *Configuration overhead.* Picking good quorum slices is
  non-trivial; bad configurations can violate intersection.
- *Centralisation pressure.* Networks tend to converge on a
  small set of widely-trusted nodes (Stellar's "tier-1" nodes,
  Ripple's UNL).
- *No quantitative fault threshold.* Unlike PBFT's `f < n/3`,
  SCP's tolerance depends on the network topology.

### Subsequent influence

- *Ripple consensus protocol* (RippleNet). Similar federated
  trust model.
- *Cobalt* (Stellar 2018). Improved liveness in the SCP model.
- *FBA-Sync* (DiGiandomenico-Schultz 2022). FBA in the
  synchronous model.
- *MIR-BFT.* Federated multi-leader pipelining.

## Practice

- *Stellar Network.* Production since 2015; cross-border
  payments, asset issuance.
- *Ripple Network.* Similar federated model.
- Both rely on a small set of "tier-1" validators that
  effectively form a closed committee, raising debate about
  whether they are truly open.

### Production-implementation notes

- *Tier-1 nodes.* Stellar's quorum-set-recommendation system
  effectively centralises trust on a handful of organisations;
  the official Stellar Foundation lists ~7 tier-1s as of
  recent times.
- *Quorum-set configuration.* Operators often copy the
  recommended Stellar quorum set rather than designing their
  own.
- *Slow convergence.* Federated voting can take longer than
  PBFT in adversarial conditions.

## Verifiability and circuit encoding

**tag: `partial`.**

SCP circuits would encode federated-voting state machines plus
signature aggregation per slice. Cost depends on slice size and
intersection structure. Threshold-signature-based optimisations
(BLS-aggregate per slice) can reduce verification cost
significantly.

## Known attacks and limitations

- *Centralisation pressure.* Tier-1 nodes effectively decide
  consensus.
- *Slice configuration errors.* Bad slices can break safety.
- *No quantitative threshold.* Hard to argue about specific
  attack scenarios.

## References

- Mazieres, "The Stellar Consensus Protocol: A Federated Model
  for Internet-level Consensus", Stellar white paper, 2015.
- Lokhava, Lossi, Tang, et al., "Fast and Secure Global
  Payments with Stellar", SOSP 2019.
- Garcia-Perez, Gotsman, "Federated Byzantine Quorum Systems",
  OPODIS 2018.

## Implementation notes

The crate provides a `QuorumSlice` per node and a
`is_quorum_intersection` predicate that checks whether any two
quorums overlap. Tests verify simple configurations.

See also [`HISTORY.md`](../HISTORY.md), section "2014 to 2017".
