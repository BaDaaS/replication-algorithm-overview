# 0063: SBFT -- Scalable Decentralized Trust Infrastructure

## Historical context

Gueta, Abraham, Grossman, Malkhi, Pinkas, Reiter, Seredinschi,
Tamir, Tomescu published "SBFT: a Scalable and Decentralized
Trust Infrastructure" at DSN 2019. SBFT is IBM's enterprise
BFT, structurally a HotStuff variant with two key
optimisations:

- *Threshold signatures.* BLS-aggregated, similar to
  HotStuff's QCs.
- *Linear collectors.* The leader uses *collectors* (a small
  set of designated replicas) to gather and aggregate
  signatures, reducing communication beyond plain HotStuff.
- *Fast path.* When all `3f + 1` replicas are responsive,
  SBFT commits in one round-trip. Falls back to slower
  paths under partial responsiveness.

SBFT is deployed in IBM's Hyperledger Fabric BFT ordering
service (experimental).

## System and threat model

Partial synchrony, `f < n / 3`, BLS aggregate signatures.

## Theory

### How SBFT differs from HotStuff

| property                | HotStuff (PODC 2019) | SBFT (DSN 2019)         |
| ----------------------- | -------------------- | ----------------------- |
| voting topology         | leader-centric       | leader + collectors     |
| fast path               | optimistic respons.  | one-RT when all live    |
| BLS aggregation         | yes                  | yes                     |
| commit chain length     | 3                    | 1 (fast) / 2 (slow)     |
| recovery on partial liveness | view change      | slow-path back-pressure |
| optimistic responsive   | yes                  | yes                     |
| linear authenticator    | yes                  | yes (with collectors)   |

The collector role is SBFT's distinctive feature:
designated replicas serve as aggregation hubs, reducing
end-to-end signature-collection latency. HotStuff's leader
plays this role implicitly; SBFT distributes it.

### How SBFT differs from Zyzzyva

Zyzzyva (module 0045) also has a fast path requiring all
replicas responsive. SBFT differs:

- *Threshold cryptography.* SBFT uses BLS aggregation;
  Zyzzyva uses individual signatures.
- *Designated collectors.* SBFT explicitly assigns
  aggregation responsibility; Zyzzyva relies on the client.
- *Recovery.* SBFT's slow path uses HotStuff-style two-chain
  recovery; Zyzzyva's involves rollback.

## Practice

- *Hyperledger Fabric (BFT mode).* IBM's prototype BFT
  ordering uses SBFT.
- *Concord.* IBM's blockchain platform built on SBFT.
- *Research benchmark.* Reference for collector-style BFT.

## Verifiability

**tag: `friendly`.** Per-block ~10^6 (BLS QC); collectors
add ~constant constraints.

## References

- Gueta et al., "SBFT: a Scalable and Decentralized Trust
  Infrastructure", DSN 2019.

See also [`HISTORY.md`](../HISTORY.md), section "2015 to
2019".
