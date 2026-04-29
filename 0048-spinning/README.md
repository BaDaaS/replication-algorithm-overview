# 0048: Spinning

## Historical context

Veronese, Correia, Bessani, Lung published "Spin One's
Wheels? Byzantine Fault Tolerance with a Spinning Primary"
at SRDS 2009. Spinning rotates the primary every consensus
round (not just on suspected failure). This bounds the
worst-case latency under a faulty primary because no primary
holds the role long enough to slow the protocol much.

## System and threat model

PBFT-style: partial synchrony, `f < n / 3`.

## Theory

Each round has a different primary, in round-robin order.
The protocol is otherwise PBFT-like.

Trade-off: per-round overhead (each primary "warms up") vs
bounded worst-case under attack.

## Practice

Influence: HotStuff's leader rotation, Aptos's per-block
leader election.

## Verifiability

**Tag: `friendly`.** Standard PBFT cost.

## References

- Veronese et al., "Spin One's Wheels?", SRDS 2009.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
