# 0074: Cordial Miners

## Historical context

Keidar, Naor, Spiegelman, Spiegelman published "Cordial
Miners: Fast and Efficient Consensus for Every Eventuality"
at DISC 2023. Cordial Miners is a DAG-BFT that drops
explicit reliable broadcast: replicas just signed-broadcast
their blocks, with no acknowledgement layer. The "cordiality"
assumption: most replicas behave well, even if a minority
is Byzantine.

Trade-off: lower per-block overhead vs Narwhal's reliable-
broadcast layer; but degraded throughput under high
Byzantine activity.

## System and threat model

Asynchronous, `f < n / 3`.

## Theory

Each replica signs and broadcasts blocks. Recipients accept
blocks if signed by a known validator. Consensus over the
"cordial DAG" runs as in Bullshark.

Without the certificate-of-availability machinery, the
mempool is `O(n)` rather than `O(n^2)` per round.

### How Cordial Miners differs from Narwhal

| property              | Narwhal      | Cordial Miners     |
| --------------------- | ------------ | ------------------ |
| reliable broadcast    | yes (cert)   | no (just signed)   |
| per-block overhead    | O(n^2)       | O(n)               |
| Byzantine tolerance   | full f < n/3 | full f < n/3       |
| throughput peak       | high         | higher             |
| throughput under attack | stable     | degraded           |

The cordiality assumption removes the bandwidth multiplier
of Narwhal's certificate exchange. Production deployments
that monitor for adversarial activity can use Cordial as a
"fast mode".

## Practice

- Reference design; not yet in major mainnet.
- Influence on Mysticeti (module 0076) and Mahi-Mahi
  (module 0077) which adopt similar streamlined ideas.

## Verifiability

**tag: `friendly`.** Per-block ~3k constraints (one
signature). Per anchor commit ~10^6 (BLS QC).

## References

- Keidar et al., "Cordial Miners", DISC 2023.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to
2023".
