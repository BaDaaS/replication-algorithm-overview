# 0060: Streamlet

## Historical context

Chan, Pass, Shi published "Streamlet: Textbook Streamlined
Blockchains" at AFT 2020. Streamlet is the *teaching*
streamlined BFT: a one-vote, three-chain protocol designed
to be simple to teach and prove correct. The whole protocol
specification fits on a single page; the safety proof fits
on the next.

## System and threat model

Partial synchrony, `f < n / 3`.

## Theory

Each *epoch* is one slot, with a deterministic round-robin
leader. The protocol has three rules:

```
each epoch:
  if you are the leader: propose a block extending the
    longest notarised chain you have seen
  on receiving a proposal: vote for it iff it extends the
    longest notarised chain you have seen
  on collecting 2n/3 votes for a block: notarise it

finality: a block is finalised iff there are three
consecutive notarised epochs in its chain (i.e., epochs
e, e+1, e+2 all notarised, all in the same chain).
```

That is the entire protocol: one vote-message type, one
notarisation rule, one finality rule.

### How Streamlet differs from PBFT, Tendermint, HotStuff

| property              | Streamlet | Tendermint | HotStuff |
| --------------------- | --------- | ---------- | -------- |
| phases per slot       | 1 vote    | 3 (propose, prevote, precommit) | 4 (prepare, pre-commit, commit, decide) |
| finality rule         | three consecutive epochs | locked precommit | three-chain QC |
| view change           | implicit (epoch advances each slot) | explicit (round timeouts) | explicit (pacemaker) |
| optimistic responsive | no        | no         | yes      |
| linear authenticator  | optional via aggregate sig | no | yes (BLS) |
| pedagogical clarity   | high      | medium     | low      |

The crucial structural difference: Streamlet's commit
latency is determined by the synchrony bound `Delta` (the
timeout for deciding "this epoch's leader is faulty"), not
by actual network latency. HotStuff's optimistic
responsiveness gives it a lower latency under stable
conditions. The trade-off is exposition: Streamlet's proof
and implementation are far simpler.

### Why three-chain finality

The same intuition as HotStuff's three-chain rule: any
notarised block has at least `f + 1` honest votes; a chain
of three consecutive notarisations transitively shows that
no conflicting block can be notarised in those epochs (by
quorum intersection). HotStuff arrives at the same
conclusion via QC linkage; Streamlet via direct chain
inspection.

## Practice

Streamlet is pedagogical only; not deployed in production.
Pala / Pili (module 0061) descends from similar ideas with
production-grade engineering.

## Verifiability

**tag: `friendly`.** Each notarisation is a BLS aggregate
(~10^6 constraints, one pairing). Three-chain finality is
provable recursively with constant-size SNARKs.

## References

- Chan, Pass, Shi, "Streamlet: Textbook Streamlined
  Blockchains", AFT 2020.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to 2023".
