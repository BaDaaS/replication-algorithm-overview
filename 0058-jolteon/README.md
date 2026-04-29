# 0058: Jolteon -- Two-Chain HotStuff

## Historical context

Gelashvili, Spiegelman, Xiang, Danezis, Li, Malkhi, Xia,
Zhou published "Jolteon and Ditto: Network-Adaptive
Efficient Consensus with Asynchronous Fallback" in 2021.
Jolteon is a *two-chain* HotStuff: collapses HotStuff's four
phases into two, achieving the same safety with half the
latency in the common case.

## System and threat model

HotStuff-style.

## Theory

Jolteon's commit rule: a block commits when it has *two*
consecutive QC-extending children, not three. The safety
proof shows that two-chain commits are still safe under
partial synchrony.

## Practice

Jolteon is the consensus heart of DiemBFTv3 / v4 (and hence
Aptos). Most modern HotStuff descendants adopt it.

## Verifiability

**Tag: `friendly`.** Per-block ~10^6 (BLS QC).

## References

- Gelashvili et al., "Jolteon and Ditto", 2021.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to 2023".
