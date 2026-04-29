# 0052: MinBFT and MinZyzzyva

## Historical context

Veronese, Correia, Bessani, Lung, Verissimo published
"Efficient Byzantine Fault-Tolerance" at IEEE TC 2013.
MinBFT and MinZyzzyva use a TPM-based trusted counter (in
the spirit of TrInc, module 0051) to reduce PBFT and
Zyzzyva's replica counts to `n = 2f + 1` (from `3f + 1`)
and reduce phase counts.

## System and threat model

PBFT-style with a TPM trusted counter per replica. `n = 2f
+ 1`.

## Theory

The TPM signs a monotone counter; equivocation prevented.
PBFT collapses to two phases (Prepare-Commit) instead of
three. Quorum is `f + 1`.

## Practice

- Production hyperscale TEE-based BFT (Intel SGX-enabled).
- Hyperledger Sawtooth's PoET (proof of elapsed time)
  shares the trusted-hardware spirit.

## Verifiability

**Tag: `partial`.** TPM attestations not naturally
SNARK-friendly.

## References

- Veronese et al., "Efficient Byzantine Fault-Tolerance",
  IEEE TC 2013.

See also [`HISTORY.md`](../HISTORY.md), section "2009 to 2014".
