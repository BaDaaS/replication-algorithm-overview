# 0058: Jolteon -- Two-Chain HotStuff

## Historical context

Gelashvili, Spiegelman, Xiang, Danezis, Li, Malkhi, Xia, Zhou
published "Jolteon and Ditto: Network-Adaptive Efficient
Consensus with Asynchronous Fallback" in 2021. Jolteon
collapses HotStuff's four-phase chain (prepare, pre-commit,
commit, decide) into two phases (vote-1, vote-2). The
safety proof requires only a *two-chain* of QCs to commit a
block, replacing HotStuff's three-chain rule.

The companion protocol Ditto (module 0064) adds an
asynchronous fallback: when network conditions degrade,
Ditto switches from Jolteon's optimistic two-chain commit
to a HoneyBadger-style asynchronous protocol, then back.
Jolteon-Ditto is the design currently shipping in DiemBFTv4
and Aptos.

## System and threat model

Same as HotStuff: partial synchrony, `f < n / 3`,
BLS-aggregate signatures.

## Theory

### Two-chain commit rule

In HotStuff, a block `B` is committed when its grandchild
`B''` exists and contains a QC over the child `B'` of `B`.
The chain `B -> B' -> B''` plus the linkage QCs forms a
"three-chain". Jolteon shortens this:

- A block `B` is committed when its child `B'` is locked
  (i.e., `B'` has a QC and the network has voted on a
  proposal extending `B'`).

This requires only a two-chain `B -> B'`, saving one round-
trip in the common case. The safety proof (Gelashvili et al.
2021, Theorem 1) shows that the two-chain rule is preserved
under partial synchrony, leveraging an extra invariant on
the proposer's locked QC.

### How Jolteon differs from HotStuff

| property                  | HotStuff (PODC 2019) | Jolteon (2021)        |
| ------------------------- | -------------------- | --------------------- |
| commit rule               | three-chain          | two-chain             |
| common-case rounds        | 4                    | 2                     |
| common-case latency       | 4 RTT                | 2 RTT                 |
| optimistic responsiveness | yes                  | yes                   |
| linear authenticator      | yes (BLS)            | yes (BLS)             |
| asynchronous fallback     | no                   | with Ditto (yes)      |
| view-change cost          | `O(n)` per phase     | `O(n)` per phase      |

The two-chain rule is the load-bearing improvement. Jolteon
has the same authenticator complexity as HotStuff, but
roughly half the steady-state latency.

### How Jolteon differs from Streamlet

Streamlet (module 0060) also uses a chain-of-3 finality, but
its commit latency is bounded by the synchrony parameter
`Delta`, not by the actual round-trip time. Jolteon adds
HotStuff's optimistic responsiveness: it commits in two
RTTs of *actual* network latency, regardless of `Delta`.

### How Jolteon differs from Tendermint

Tendermint (module 0055) commits per slot with three voting
steps and explicit locking. Jolteon's locking is implicit in
the QC chain. Jolteon's pipelining gives one block per RT in
steady state; Tendermint's locked-precommit forces sequential
slots.

## Practice

- *DiemBFTv3 / v4.* Direct adoption of Jolteon's two-chain
  rule.
- *Aptos.* Production Jolteon + Quorum Store + Shoal++.
- *Subsequent research.* Jolteon is the modern baseline;
  HotStuff-2 (module 0059) further reduces phase count to two
  with a different proof strategy.

## Verifiability

**tag: `friendly`.** Per-block ~10^6 constraints (one BLS QC
pairing). Two-chain encoding requires one fewer step than
HotStuff's three-chain when verified recursively, marginally
reducing prover work.

## References

- Gelashvili, Spiegelman, Xiang, Danezis, Li, Malkhi, Xia,
  Zhou, "Jolteon and Ditto: Network-Adaptive Efficient
  Consensus with Asynchronous Fallback", 2021.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to
2023".
