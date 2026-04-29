# 0057: LibraBFT and DiemBFT

## Historical context

LibraBFT (2019, Baudet-Cherniak-Danezis-Garillot-Kichidis-
Malkhi-Pinzon-Sonnino-Sun-Tonkikh-Xu) was Facebook's
HotStuff implementation for the Libra blockchain. After
Libra's pivot to Diem in 2020 and its eventual dissolution
in 2022, the protocol lineage continued as DiemBFT v1, v2,
v3, v4. The Aptos and Sui blockchains adopted DiemBFT
descendants.

DiemBFTv4 incorporates Jolteon's two-chain refinement
(module 0058), making it the production basis for Aptos.

## System and threat model

HotStuff-style: partial synchrony, `f < n / 3`, BLS or
Ed25519 aggregate signatures.

## Theory

Each Diem version refined HotStuff:

- *v1 (2019).* Direct HotStuff implementation in Rust.
- *v2 (2020).* Pacemaker improvements; pipelined block
  proposals.
- *v3 (2020).* Jolteon-style two-chain commit (module 0058).
- *v4 (2021).* Quorum Store separation; further pipelining.

## Practice

- *Aptos.* Production deployment of DiemBFTv4 with Quorum
  Store and Shoal pipelining.
- *Sui (initial).* Used a HotStuff variant; later moved to
  Bullshark and Mysticeti.

## Verifiability

**Tag: `friendly`.** Same as HotStuff: ~10^6 per block.

## References

- Baudet et al., "State Machine Replication in the Libra
  Blockchain", 2019 whitepaper.
- DiemBFT v4 specification, 2021.

See also [`HISTORY.md`](../HISTORY.md), section "2015 to 2019".
