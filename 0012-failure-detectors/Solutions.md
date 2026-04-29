# Module 0012 Solutions

## Solution 1 [T]: weak-to-strong completeness

Gossip protocol: each process `i` periodically broadcasts its
suspect list `S_i`. On receipt of `S_j` from peer `j`, process
`i` adds `S_j` to its own suspect list (`S_i := S_i ∪ S_j`).

Correctness:

- *Strong completeness.* If a faulty `f` is permanently
  suspected by some honest `i` (weak completeness), then `i`'s
  broadcast carries `f` to every honest peer; eventually all
  honest peers suspect `f`.
- *Accuracy preservation.* If the original detector has
  eventual weak accuracy, the gossip preserves it: an honest
  process never permanently suspected by some peer remains so.

Messaging cost: `O(n)` per process per round, `O(n^2)` overall
per gossip round.

## Solution 2 [T]: `Omega` is the weakest

CHT 1996: any detector that solves consensus implements
`Omega`. The reduction: from a consensus-solving protocol `P`
with detector `D`, define `leader_i := f(state_of_P_at_i)` for
some function `f` extracting a "would-be coordinator" from the
local state. Show that, eventually, all honest processes agree
on the same `leader_i` and the leader is honest.

The argument uses the *bivalence-style* reduction of FLP: any
non-terminating execution of `P` would correspond to an
infinite bivalent execution, contradicting `P`'s correctness.
The eventual agreement on a leader is exactly `Omega`.

## Solution 3 [T]: Byzantine generalisations

Under Byzantine faults, a malicious `j` can broadcast a *false*
suspect set, causing honest processes to suspect honest peers.
The gossip-based weak-to-strong reduction relies on the
sender's honesty about its suspect set, which Byzantine breaks.

Doudou-Schiper 1998's *muteness* detector replaces the suspect
list with a "mute" predicate: `j` is mute iff `j` has not sent
any *protocol-level* message in the last `T` ticks. Muteness is
robust to Byzantine adversaries because it is observable from
the receiver's local state, not communicated by the suspect.

Production: PBFT and HotStuff do not name a detector; their
view-change machinery is structurally an `Omega`-equivalent
that uses muteness on the leader.

## Solution 4 [P]: SWIM

SWIM (Das-Gupta-Motivala 2002) adds:

1. *Indirect probes.* If `i` doesn't get a direct ack from `j`,
   it asks `k` peers to probe `j` on its behalf. If any of them
   gets an ack, `j` is alive.
2. *Piggybacked dissemination.* Membership updates are
   piggybacked on heartbeats; a separate gossip layer is not
   required.
3. *Randomised peer selection.* Each round, `i` probes a random
   peer rather than all peers. This bounds per-tick load to
   `O(1)` while still achieving `O(log n)`-time dissemination.

SWIM provides eventual completeness and accuracy under
adaptive failure rates; it does not guarantee real-time
detection. Used in: Hashicorp Consul, Cassandra (legacy).

## Solution 5 [P]: tune timeouts

Empirical observations (typical):

- *Detection latency.* With `interval = 1, timeout = 5`,
  latency is exactly `timeout = 5` ticks after the peer's last
  beat.
- *False suspicion under partial synchrony.* Random delays
  `<= timeout / 2` cause no false suspicions; delays of order
  `timeout` cause occasional false suspicions; delays exceeding
  `timeout` cause sustained false suspicions.

The production rule of thumb: set `timeout = 5 * RTT_p99 + buffer`,
where `RTT_p99` is the 99th-percentile round-trip time.

## Solution 6 [F]: pseudo-Lean detector classes

```text
class EventuallyStronglyAccurate (D : FailureDetector n) where
  prop :
    Eventually (
      forall i j, IsHonest i -> IsHonest j -> j ∉ D.output i
    )

class StronglyComplete (D : FailureDetector n) where
  prop :
    Eventually (
      forall i j, IsHonest i -> IsCrashed j -> j ∈ D.output i
    )

abbrev EventuallyStrong (D : FailureDetector n) :=
  StronglyComplete D /\ EventuallyStronglyAccurate D
```

LTL form: `<>S` is `F G (forall j ∈ Faulty, F (suspected j))`.

## Solution 7 [V]: verifiable detector outputs

A signed-heartbeat detector produces verifiable evidence: each
heartbeat is `(sender, timestamp, sigma)` with `sigma =
sign(sk, sender || timestamp)`. The freshness predicate
"`heartbeat` was emitted within the last `T` ticks of `now`" is
a circuit-encodable check.

Circuit:

- Verify `sigma` against `sender`'s public key (1 sig
  verification, ~3k constraints in Schnorr-over-Pasta).
- Compare `timestamp + T >= now` (constant constraints).

Application: a zk-bridge that needs to attest "chain `B` is
making progress" can include a fresh signed heartbeat as
witness. Polyhedra and Succinct's Telepathy include progress
predicates of this form.

## Solution 8 [V]: detector accuracy and FFG slashing

Casper FFG (module 0049) defines two slashable conditions:

- *Double-vote.* Same validator votes for two distinct values
  at the same height.
- *Surround-vote.* Validator's vote span surrounds another's.

Both are *accuracy* failures: they imply the validator made
inconsistent claims about the chain's state, which is the
verifiable analogue of "wrongly suspecting an honest peer".

The mapping: `<>S` says eventually no honest validator is
suspected. FFG slashing says any provable equivocation is
penalised, regardless of whether it was a false accusation or
a legitimate one. The slashing mechanism is the FFG verifiable
counterpart of `<>S`-accuracy.

Module 0049 shows that the FFG slashing predicates are
SNARK-friendly under BLS aggregation; light clients can
produce slashing witnesses succinctly.
