# 0076: Mysticeti

## Historical context

Babel, Sonnino, Spiegelman, Kokoris-Kogias, Lutsch published
"Mysticeti: Reaching the Limits of Latency with Uncertified
DAGs" at NSDI 2024. Mysticeti is the most aggressive
collapse of DAG-BFT to date: it removes Narwhal's
certificate-of-availability layer and runs consensus
directly on raw signed blocks. This brings commit latency
down to ~390ms in production for Sui's geo-distributed
validator set, the lowest of any production-grade BFT.

The follow-up *Mysticeti-FPC* (Fast Path Commit) adds a
2/3-quorum fast-commit path for non-conflicting transactions,
delivering sub-second finality.

## System and threat model

- **Network.** Partial synchrony with async fallback.
- **Failures.** Byzantine, `f < n / 3`.
- **Crypto.** BLS aggregate signatures.
- **Goal.** Sub-second BFT finality.

## Theory

### Uncertified DAG

Each block is signed (not certified by 2f+1 receivers).
Recipients run consensus over the local DAG; certificate
absence is compensated by the consensus rule, which only
commits blocks observed by 2f+1 honest replicas.

### Three commit paths

Mysticeti has three commit paths:

- *Fast.* 2/3 super-majority observes block in next round:
  commit immediately. ~1 RTT.
- *Steady.* 2f+1 observation in 2 rounds: commit. ~2 RTT.
- *Async.* DAG-Rider-style wave anchor: commit eventually.

The fast path applies under low-conflict workloads.

### How Mysticeti differs from Bullshark

| property              | Bullshark         | Mysticeti              |
| --------------------- | ----------------- | ---------------------- |
| certificate layer     | Narwhal cert     | none (uncertified DAG) |
| typical commit RTT    | ~2                | ~1 (fast) / 2 (steady) |
| latency (production)  | ~2 s             | ~390 ms               |
| async fallback        | Tusk             | DAG-Rider style       |
| production deployment | Sui 22-24        | Sui 24+                |

The structural delta: removing the certificate layer is the
main latency win. Each block reaches consensus a full RTT
faster than under Bullshark.

### How Mysticeti differs from Cordial Miners

Cordial Miners (module 0074) also drops certificates but
keeps DAG-Rider's async commit. Mysticeti adds the
fast/steady commit paths for production-grade latency.

### Mysticeti-FPC

For *non-conflicting* transactions (Sui's UTXO model lets
many transactions touch disjoint state), Mysticeti-FPC
commits in the fast path with 2/3 super-majority. For
conflicting transactions, falls back to steady path.

In Sui's owned-objects fast path: ~140ms commit. For shared
objects: ~390ms.

## Practice

- *Sui mainnet (2024+).* Production Mysticeti. The lowest-
  latency BFT in production.
- *Reference implementation.* Mysten Labs open source.

## Verifiability

**tag: `friendly`.** Per-block ~3k (signed) plus per-anchor
QC ~10^6. Sub-second finality enables real-time light
clients.

## References

- Babel, Sonnino, Spiegelman, Kokoris-Kogias, Lutsch,
  "Mysticeti: Reaching the Limits of Latency with
  Uncertified DAGs", NSDI 2024.

See also [`HISTORY.md`](../HISTORY.md), section "2024 to
2026".
