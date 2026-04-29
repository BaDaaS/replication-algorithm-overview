# Module 0002 Solutions

## Solution 1 [T]: crash-stop refines authenticated Byzantine

Let `Alg` be a protocol tolerating `f` authenticated Byzantine
faults. Suppose `f` processes are crashed (rather than Byzantine).
A crashed process's behaviour is a special case of an authenticated
Byzantine process: it sends nothing after the crash, which is one
of the behaviours an authenticated Byzantine adversary can choose.
By assumption, `Alg` is correct against all authenticated
Byzantine schedules; in particular, against the schedule in which
`f` processes silently halt at their respective crash times.
Therefore `Alg` is correct against `f` crashes. The lower bound on
resilience follows by contrapositive.

## Solution 2 [T]: PSL three-process Byzantine impossibility

Three honest commanders `c_1, c_2, c_3`, of which one (say `c_2`)
is Byzantine. The Byzantine process can present different values
to the other two commanders. Consider two indistinguishable
schedules:

- *S_1*: `c_2` is Byzantine, telling `c_1` the value 0 and `c_3`
  the value 1. The honest `c_1, c_3` must choose between 0 and 1.
- *S_2*: `c_3` is Byzantine, simulating the messages of `S_1` to
  `c_1`. To `c_1`, `S_1` and `S_2` are indistinguishable: it sees
  one report of 0 and one report of 1.

In `S_1`, `c_3` is honest and observes 1; SMR validity requires
`c_3` to decide a value present in some honest input. In `S_2`,
`c_1` cannot distinguish, but `c_3` is now Byzantine so any
decision is acceptable. The contradiction is that `c_1` must
decide based on the same view in both cases, but the constraints
are inconsistent.

The complete proof, including the case analysis on which two
processes are honest, is in PSL 1980 Theorem 2. The argument
generalises: with `n <= 3f`, three groups of processes can be
constructed in which the adversary controls one group entirely
and the other two cannot agree.

## Solution 3 [T]: crash-recovery without stable storage

Two processes `p_1, p_2` running a leader-broadcast SMR with
`p_1` as leader. Sequence:

1. Client submits `op_1` to `p_1`. `p_1` decides `op_1` and tells
   the client "committed".
2. `p_1` crashes before forwarding `op_1` to `p_2`.
3. `p_1` recovers with empty in-memory state; it has no memory of
   committing `op_1`.
4. Client submits `op_2`. `p_1` decides `op_2` as the first
   operation.
5. `p_2` had no record of `op_1` either.

Now `p_1`'s and `p_2`'s logs disagree with the client's view (the
client believes `op_1` was committed first; the replicas believe
`op_2` was committed first). SMR safety fails because the agreed
prefix at the client and at the replicas differ.

In contrast, crash-stop forbids `p_1` from acknowledging `op_1`
without surviving long enough to share it. Stable storage rescues
crash-recovery by persisting the acknowledgement before responding.

## Solution 4 [P]: crash-recovery node sketch

State to persist per replica:

- Current view (or `currentTerm` in Raft).
- Voted-for in this view (or `votedFor` in Raft).
- The committed-prefix log (or its hash).
- Any "promise" the replica has issued in the protocol.

Reference: Raft section 5 lists the fields explicitly. ZooKeeper's
Zab persists the same triple `(epoch, voted_leader, log)`.

## Solution 5 [P]: equivocation detector

Recipient `3` cannot detect equivocation in isolation; it sees one
value and has no contradicting witness. A detector requires
gossip:

```
on receive M from sender S to me:
  store (S, M)
  broadcast (S, M) to all peers
on receive (S, M) from peer P:
  if (S, M') already stored with M != M':
    record (S, M, M', P) as evidence of equivocation
```

After one gossip round, every honest recipient learns of every
contradiction. The cost is `O(n^2)` messages per equivocation. In
production, this is the role of CometBFT's "evidence module" and
Casper FFG's slashing layer.

## Solution 6 [F]: failure-mode refinement

```text
inductive FailureMode where
  | honest
  | crash
  | omission
  | byzantineAuth
  | byzantineUnauth

def refines : FailureMode -> FailureMode -> Prop
  | _, honest                         => True
  | crash,           crash             => True
  | crash,           omission          => True
  | crash,           byzantineAuth     => True
  | crash,           byzantineUnauth   => True
  | omission,        omission          => True
  | omission,        byzantineAuth     => True
  | omission,        byzantineUnauth   => True
  | byzantineAuth,   byzantineAuth     => True
  | byzantineAuth,   byzantineUnauth   => True
  | byzantineUnauth, byzantineUnauth   => True
  | _, _                                => False
```

Honest refines every mode, crash refines all but honest, etc.
Monotonicity holds when the protocol's transition relation is
quantified universally over the adversary's schedules: if `Alg`
is safe against schedule set `Sigma_2` and `refines mode_1 mode_2`,
then `Sigma_1 ⊆ Sigma_2`, hence `Alg` is safe against `Sigma_1`.

## Solution 7 [V]: slashable behaviour predicates

- **Casper FFG double-vote.** `DoubleVote(v, h)(att1, att2) :=
  signed_by(att1, v) /\ signed_by(att2, v) /\ height(att1) =
  height(att2) /\ att1 != att2`. The verifier checks two BLS
  signatures and an equality of heights. Cost: two BLS-12-381
  pairings, on the order of `~10^6` constraints per signature
  verification under aggregation; about `~3 * 10^6` constraints
  total. Used in Ethereum's slashing on the beacon chain.

- **Casper FFG surround-vote.** `SurroundVote(...) :=
  source(att1) < source(att2) /\ target(att2) < target(att1)`,
  plus signature checks. The cost is the same as double-vote.
  Source: Buterin-Griffith 2017, Section 4.4.

- **Tendermint double-prevote.** `DoublePrevote(v, h, r)(s1, s2)
  := same_validator(s1, s2) /\ same_round(s1, s2) /\
  prevote_block(s1) != prevote_block(s2)`. Tendermint uses Ed25519
  signatures, which are friendlier in some SNARK frameworks (Aleo)
  and harder in others (Halo 2 over Pasta needs a wrapping curve).

## Solution 8 [V]: rational deviation and verifiability

Selfish mining is a *strategic* deviation that does not violate
Bitcoin's safety predicate (no double spend, no invalid block):
withheld blocks are not malformed. A SNARK proof of Bitcoin's
longest-chain rule attests to "this prefix is consistent with the
protocol", which a selfishly-mined chain still is. The proof
neither detects nor prevents selfish mining.

Mitigations:

- *Symmetric rewards.* Subsidising losing forks (e.g. Ethereum's
  uncle rewards) lowers the gain from withholding. The chain's
  rule is changed to incorporate uncles, making selfish mining
  less profitable.
- *FruitChains* (Pass-Shi 2017) decouples block production
  (blocks) from transaction inclusion (fruits), giving each fruit
  inclusion a reward independent of block winning.
- *Higher-power lower bounds.* Sapirshtein-Sompolinsky-Zohar
  (2016) tightened the threshold; selfish mining is profitable
  only above a power-dependent fraction. Fractional ASIC pools and
  cooperatives tend to absorb sub-threshold attackers.

A verifiable replication algorithm augments these mitigations only
in the sense that it makes the offence cheaper to *report* (a
light client can produce a succinct witness). It does not change
the strategic calculation.
