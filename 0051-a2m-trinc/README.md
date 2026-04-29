# 0051: A2M and TrInc -- Trusted Hardware for BFT

## Historical context

Two influential papers used a small trusted-hardware
primitive to reduce BFT's replica count:

- *A2M (Attested Append-only Memory)* (Chun, Maniatis, Shenker,
  Kubiatowicz, SOSP 2007). A trusted log that cannot be
  rewritten; even a Byzantine process can be made to appear
  to commit-once-and-only-once.
- *TrInc (Trusted Increment)* (Levin, Douceur, Lorch, Moscibroda,
  NSDI 2009). A trusted monotonically-increasing counter:
  signs `(counter, value)` tuples for accountability.

Both reduce BFT's replica count from `n > 3f` to `n > 2f` by
preventing Byzantine equivocation: a Byzantine process can
still send arbitrary messages but can only sign each
sequence number once.

## System and threat model

- Each replica has a small *trusted module* (TPM or trusted
  log) that signs monotonically-increasing counter values.
- Software outside the trusted module may be Byzantine.
- Goal: BFT with `f < n / 2` instead of `f < n / 3`.

## Theory

Equivocation is the primary Byzantine attack: a Byzantine
process tries to sign two contradicting messages with the
same sequence number. The trusted counter prevents this:
each `(counter, sig)` pair is unique.

With equivocation prevented, the BFT quorum reduces from
`2f + 1` (in `n = 3f + 1`) to `f + 1` (in `n = 2f + 1`).

## Practice

- *Hyperledger Fabric.* Optional TEE-based ordering modes.
- *Modern TEE-based BFT.* Intel SGX-enabled BFT systems
  (Hybster, MinBFT) descend from A2M/TrInc.

## Verifiability

**Tag: `partial`.** The trusted-counter assumption is
hardware, not cryptographic. Verifying TPM attestations in
SNARKs is open.

## References

- Chun et al., "Attested Append-only Memory: Making Adversaries
  Stick to Their Word", SOSP 2007.
- Levin et al., "TrInc: Small Trusted Hardware for Large
  Distributed Systems", NSDI 2009.

See also [`HISTORY.md`](../HISTORY.md), section "2000 to 2008".
