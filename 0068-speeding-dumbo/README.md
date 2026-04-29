# 0068: Speeding Dumbo

## Historical context

Guo, Lu, Lu, Tang, Zhang, Zhao published "Speeding Dumbo:
Pushing Asynchronous BFT Closer to Practice" at NDSS 2022.
The follow-up to Dumbo (module 0067) reduces the
asynchronous-coin overhead by two further factor-of-N
reductions, bringing async-BFT throughput within striking
distance of partial-synchrony designs.

## System and threat model

Asynchronous, `f < n / 3`, threshold-BLS.

## Theory

### Key optimisations over Dumbo

- *Pipelined ABA.* Multiple ABA instances proceed
  concurrently, amortising the threshold-coin cost.
- *Reduced common-coin invocations.* Each round needs
  fewer fresh coin queries.
- *Streaming dispersal.* AVID is overlapped with ABA
  voting.

### How Speeding Dumbo differs

| property                | Dumbo2  | Speeding Dumbo |
| ----------------------- | ------- | -------------- |
| ABAs per epoch          | 1       | 1 (pipelined)  |
| coin queries per epoch  | ~5      | ~2             |
| throughput (n=100, paper) | ~10000 tx/s | ~30000 tx/s |
| latency under good network | ~1.5 s | ~700 ms      |
| async                   | yes     | yes            |

The 3x throughput improvement is mainly from pipelining;
the latency reduction comes from fewer coin queries.

### How Speeding Dumbo compares to partial-synchrony BFT

Speeding Dumbo's throughput at n = 100 is ~30000 tx/s. Aptos
(DiemBFTv4 + Quorum Store, partial sync) achieves ~10000
tx/s. The async protocol now exceeds partial-sync designs
on raw throughput, while preserving the stronger fault
tolerance.

The trade-off: Speeding Dumbo's threshold-DKG is heavy
operationally; deploying it across geo-distributed
validators adds significant setup latency.

## Practice

- Reference implementation in Go.
- Production interest: some private chains; no major
  mainnet uses Speeding Dumbo as of 2026.

## Verifiability

**tag: `friendly`.** Per-epoch ~10^7 constraints; pipelining
reduces wall-clock proof time but not constraint count.

## References

- Guo et al., "Speeding Dumbo: Pushing Asynchronous BFT
  Closer to Practice", NDSS 2022.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to
2023".
