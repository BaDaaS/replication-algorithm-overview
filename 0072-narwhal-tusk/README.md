# 0072: Narwhal and Tusk

## Historical context

Danezis, Kokoris-Kogias, Sonnino, Spiegelman published
"Narwhal and Tusk: A DAG-based Mempool and Efficient BFT
Consensus" at EuroSys 2022. The decisive insight: completely
decouple *mempool/dissemination* (Narwhal) from
*consensus/ordering* (Tusk). The mempool becomes a fully
parallel layer that can saturate network bandwidth; the
consensus runs over the mempool's DAG, deciding only on
ordering, not on data.

The result: 600k tx/s sustained at n = 50, vs HotStuff's
~50k tx/s. Production: Sui mainnet (Bullshark over Narwhal).

## System and threat model

- **Narwhal.** Asynchronous reliable broadcast of
  transaction batches (certificates of availability).
- **Tusk.** Asynchronous BFT consensus over Narwhal's DAG.
- **Failures.** Byzantine, `f < n / 3`.
- **Crypto.** Threshold-BLS, Ed25519 for batch signatures.

## Theory

### Narwhal: the mempool

Each round, every replica:

1. Builds a local *batch* of transactions.
2. Sends the batch to all peers.
3. Collects `2f + 1` signed acknowledgements (a
   *certificate of availability*).
4. Includes that certificate (plus parents) in next round's
   *Narwhal block*.

Crucially, Narwhal blocks contain only *certificates*, not
transactions. Once a certificate is in the DAG, the batch
is provably stored at `2f + 1` replicas (`f + 1` honest);
the certificate is small (signature + hashes), the batch is
large.

### Tusk: the consensus over Narwhal

Every three Narwhal rounds, a *leader vertex* is chosen
randomly via threshold-BLS coin. If the leader vertex is
referenced by enough later vertices, it commits, and the
total order of all causal ancestors follows.

Tusk is a wave-based async-BFT consensus, structurally
similar to DAG-Rider's anchor commit (module 0071).

### How Narwhal-Tusk differs from prior BFT

| property              | HoneyBadger | DAG-Rider    | Narwhal-Tusk |
| --------------------- | ----------- | ------------ | ------------ |
| mempool / consensus separation | no | partial   | full         |
| transactions in consensus | yes (decrypted) | no | no (certs only) |
| sustained throughput at n=50 | ~10k tx/s | ~50k tx/s | ~600k tx/s |
| latency               | ~few s      | ~few s       | ~3-4 RTT     |
| consensus protocol    | n parallel ABA | wave anchor | wave anchor (Tusk) |
| async                 | yes         | yes          | yes          |

The structural breakthrough: by removing transaction data
from consensus messages, Narwhal-Tusk eliminates the bandwidth
bottleneck that limited prior BFT. Throughput becomes
network-bandwidth bound rather than consensus-bound.

### How Narwhal-Tusk compares with HotStuff family

HotStuff and DiemBFTv4 keep transactions in the consensus
path. Narwhal-Tusk removes them. The throughput gap (10x or
more) is a function of this single architectural choice;
it is the most influential BFT idea since HotStuff's
linearity (2019).

### Bullshark over Narwhal

Bullshark (module 0073) reuses Narwhal's mempool and adds a
*partial-synchrony* commit rule (faster than Tusk's async
commit when network is healthy). Sui adopted Bullshark over
Narwhal in 2022; Aptos's Quorum Store is similar.

## Practice

- *Sui mainnet.* Production Bullshark over Narwhal until
  2024; replaced by Mysticeti (module 0085) for lower
  latency.
- *Aptos Quorum Store.* Adapts Narwhal ideas for the Aptos
  consensus pipeline.
- *Reference implementation.* Mysten Labs' open-source code.

## Verifiability

**tag: `friendly`.** Per-batch certificate ~10^6 (BLS
threshold sig). Tusk anchor commit ~10^6 per wave. zk-bridges
to Sui (Polyhedra) verify Narwhal certs in SNARKs.

## References

- Danezis, Kokoris-Kogias, Sonnino, Spiegelman, "Narwhal
  and Tusk: A DAG-based Mempool and Efficient BFT
  Consensus", EuroSys 2022.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to
2023".
