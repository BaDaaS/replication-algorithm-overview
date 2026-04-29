# 0059: HotStuff-2

## Historical context

Malkhi and Nayak's 2023 paper "HotStuff-2: Optimal Two-Phase
Responsive BFT" gives the optimal phase count: just two
phases per block while preserving HotStuff's optimistic
responsiveness and linear authenticator complexity.

HotStuff-2 was the inspiration for Aptos's optimisation work
and the modern wave of two-phase BFT designs.

## System and threat model

HotStuff-style.

## Theory

HotStuff-2's two phases (vote-1, vote-2) replace HotStuff's
four. The proof shows that the safety properties of HotStuff
are preserved, the optimistic responsiveness is unchanged,
and the throughput improves by 2x in the steady state.

## Practice

- *Aptos.* Adopted HotStuff-2 ideas in DiemBFTv4 and
  subsequent versions.
- *Reference for 2024+ BFT designs.* Research benchmark.

## Verifiability

**Tag: `friendly`.** Per-block ~10^6 (BLS QC).

## References

- Malkhi, Nayak, "HotStuff-2: Optimal Two-Phase Responsive
  BFT", 2023.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to 2023".
