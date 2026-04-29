# 0050: Steward -- Hierarchical BFT

## Historical context

Amir, Danilov, Dolev, Kirsch, Lane, Nita-Rotaru, Olsen,
Zage published "Steward: Scaling Byzantine Fault-Tolerant
Replication to Wide Area Networks" at TDSC 2010. Steward
addresses the latency cost of running BFT across data
centres by structuring replicas hierarchically:

- *Local-site BFT.* Each data centre runs an internal BFT
  (PBFT-equivalent) over its replicas.
- *Inter-site Paxos.* Sites then run a crash-fault Paxos to
  coordinate globally.

The hierarchy isolates Byzantine faults to within sites and
treats sites as crash-fault peers, dramatically reducing
WAN-level message complexity.

## System and threat model

- Each site has `n_local` replicas, tolerating `f_local`
  Byzantine.
- `S` sites globally, tolerating `f_global` site-level crash
  (entire-site failures).
- `n_local >= 3 * f_local + 1` per site; `S >= 2 * f_global +
  1` globally.

## Theory

Two-level state machine:

1. *Local BFT.* PBFT inside each site.
2. *Global Paxos.* Sites broadcast their committed local
   results; global Paxos commits.

Steward's key insight: Byzantine corruption inside a site
doesn't propagate outside (the site's external behaviour is
constrained by its local BFT).

## Practice

- Hyperledger Fabric's per-channel ordering services use a
  similar hierarchy.
- Modern multi-region deployments adopt the structural idea.

## Verifiability

**Tag: `friendly`.** Per-commit ~10^6 (BLS) per layer.

## References

- Amir et al., "Steward", TDSC 2010.

See also [`HISTORY.md`](../HISTORY.md), section "2009 to 2014".
