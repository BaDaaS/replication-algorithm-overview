# 0066: BEAT

## Historical context

Duan, Reiter, Zhang published "BEAT: Asynchronous BFT
Made Practical" at CCS 2018. BEAT is a family of five
asynchronous BFT protocols (BEAT0 through BEAT4),
each tuned for a different deployment profile, all sharing
a HoneyBadger-style architecture but with engineering
refinements.

## System and threat model

Same as HoneyBadger: asynchronous, `f < n / 3`,
threshold cryptography.

## Theory

The five BEAT variants:

- *BEAT0.* Direct HoneyBadger refinement; reduces
  redundant computations.
- *BEAT1.* Trades latency for bandwidth: smaller
  AVID dispersal.
- *BEAT2.* Trades bandwidth for latency: more
  parallel transmission.
- *BEAT3.* Asynchronous KZG-commitment-based dispersal.
- *BEAT4.* Combines BEAT3's KZG with BEAT2's parallelism.

Each variant achieves 1.5-3x throughput over HoneyBadger
on commodity hardware in the paper's evaluation.

### How BEAT differs from HoneyBadger

| property                | HoneyBadger | BEAT family |
| ----------------------- | ----------- | ----------- |
| AVID variant            | erasure-coded | KZG (BEAT3+) |
| ABA primitive           | MMR-style   | optimised   |
| throughput at n=64      | ~1500 tx/s  | ~3000-7000 tx/s |
| design space            | one point   | five tuned  |
| threshold-encryption    | yes         | yes         |

BEAT's contribution is engineering exploration of the
HoneyBadger design space, not a fundamentally new protocol.

## Practice

- Reference implementation in Go.
- Influence on later async BFT designs (Dumbo, Speeding
  Dumbo).

## Verifiability

**tag: `friendly`.** Same as HoneyBadger; KZG-based variants
are slightly more SNARK-friendly per dispersal.

## References

- Duan, Reiter, Zhang, "BEAT: Asynchronous BFT Made
  Practical", CCS 2018.

See also [`HISTORY.md`](../HISTORY.md), section "2015 to
2019".
