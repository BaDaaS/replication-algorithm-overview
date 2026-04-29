# 0057: LibraBFT and DiemBFT

## Historical context

LibraBFT (2019, Baudet-Cherniak-Danezis-Garillot-Kichidis-
Malkhi-Pinzon-Sonnino-Sun-Tonkikh-Xu) was Facebook's
HotStuff implementation for the Libra blockchain. After
Libra's pivot to Diem in 2020 and its eventual dissolution
in 2022, the protocol lineage continued as DiemBFT v1, v2,
v3, v4, and was inherited by Aptos and (initially) Sui.

Each Diem version is a strict refinement of the previous;
together they constitute the most-iterated production
HotStuff lineage.

## System and threat model

HotStuff-style: partial synchrony, `f < n / 3`, BLS or
Ed25519 aggregate signatures, with a stake-weighted
extension for PoS.

## Theory

### Version-by-version delta

| version | year | major change |
| ------- | ---- | ------------ |
| LibraBFT v1 | 2019 | Direct HotStuff in Rust, three-chain commit, integrated pacemaker. |
| DiemBFT v2  | 2020 | Pacemaker improvements; better view-change throughput. |
| DiemBFT v3  | 2020 | Two-chain commit (Jolteon-style refinement). |
| DiemBFT v4  | 2021 | Quorum Store: decouples mempool from consensus, parallel block construction. |
| Aptos / Shoal++ | 2024 | Multiple QCs in flight per anchor; Shoal-style anchor commits. |

Each version is a *strict refinement*: safety arguments
carry forward, with new-version replicas able to handle
old-version messages.

### How LibraBFT/DiemBFT differs from baseline HotStuff

- *Rust-native implementation.* The original HotStuff was
  pseudocode + TLA+; LibraBFT made it a production stack.
- *Stake-weighted voting.* The `2f + 1` threshold becomes
  `> 2/3 of total stake`. PoS-native.
- *Move VM integration.* Block proposals carry Move
  bytecode and VM state-roots; consensus and execution are
  tightly coupled.
- *Quorum Store.* Decouples transaction batching from
  consensus, similar in spirit to Narwhal-Tusk (module
  0072).
- *Vote extensions.* Validators can attach
  application-specific data to votes; used by Aptos for
  oracle-feed aggregation.

### How DiemBFTv4 differs from Tendermint

| property                | DiemBFTv4              | Tendermint              |
| ----------------------- | ---------------------- | ----------------------- |
| pipeline                | yes (Quorum Store)     | per-slot, no pipeline   |
| commit chain length     | 2 (Jolteon)            | 1 (locked precommit)    |
| optimistic responsive   | yes                    | no                      |
| stake-weighted voting   | yes                    | yes                     |
| signature scheme        | BLS aggregate          | Ed25519 (no aggregate)  |
| typical commit latency  | ~200 ms (Aptos)        | ~5 s (Cosmos defaults)  |

The throughput difference is significant: Aptos's mainnet
achieves ~10^4 TPS sustained, Cosmos chains typically
~10^3 TPS. Most of the gap is BLS aggregation + Quorum-
Store pipelining.

## Practice

- *Aptos mainnet.* Production DiemBFTv4 + Quorum Store +
  Shoal++.
- *Sui mainnet (initial).* Used a HotStuff variant; later
  moved to Bullshark and Mysticeti (modules 0073, 0085).
- *Diem dissolution (2022).* Code base open-sourced; Aptos
  and Sui both forked.

## Verifiability

**tag: `friendly`.** Same as HotStuff: ~10^6 per block (one
BLS QC pairing). Stake-weighted threshold check adds ~`n`
constraints (negligible).

zk-bridges to Aptos and Sui are practical; Polyhedra and
Succinct's SP1 have implementations of the verifier.

## References

- Baudet et al., "State Machine Replication in the Libra
  Blockchain", whitepaper 2019.
- DiemBFT v4 specification, 2021.
- Aptos consensus documentation.

See also [`HISTORY.md`](../HISTORY.md), section "2015 to
2019" and "2020 to 2023".
