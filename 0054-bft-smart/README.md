# 0054: BFT-SMaRt

## Historical context

Bessani, Sousa, Alchieri published "State Machine Replication
for the Masses with BFT-SMART" at DSN 2014. BFT-SMaRt is a
modular Java BFT library implementing a PBFT-equivalent
protocol with extensive engineering: pluggable cryptography,
network gossip, state-transfer, dynamic reconfiguration.

It became the most-deployed open-source BFT library of the
2010s, used in academic research, the Hyperledger ordering
service prototypes, and several private blockchains.

## System and threat model

PBFT-style: partial synchrony, `f < n / 3`.

## Theory

The protocol is structurally PBFT with the engineering
refinements of the previous decade (Aardvark robustness,
Spinning rotation, etc.). Modular design lets operators swap
authentication schemes, network layers, and recovery
strategies.

## Practice

- *Hyperledger.* Initial BFT ordering prototypes built on
  BFT-SMaRt.
- *Reference for academic research.* Most BFT comparison
  studies in 2014-2020 used BFT-SMaRt as the baseline.
- *Production private blockchains.* Several enterprise BFT
  deployments use BFT-SMaRt.

## Verifiability

**Tag: `friendly`.** Per-commit ~10^6 (BLS).

## References

- Bessani, Sousa, Alchieri, "State Machine Replication for
  the Masses with BFT-SMART", DSN 2014.

See also [`HISTORY.md`](../HISTORY.md), section "2009 to 2014".
