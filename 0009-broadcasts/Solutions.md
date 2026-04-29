# Module 0009 Solutions

## Solution 1 [T]: Bracha RB safety

Suppose, for contradiction, two distinct contents `m, m'` from
sender `s` are both delivered. Each delivery requires `2f + 1`
ready messages. Of these `2f + 1`, at least `f + 1` are honest
(at most `f` Byzantine). Honest processes ready at most one
content per sender (by the readied set), so the same `f + 1`
honest processes cannot have readied both `m` and `m'`. Hence
the two ready sets share at most `f` honest contributors. The
remaining `f + 1` contributors per delivery would have to be
Byzantine, but there are only `f` Byzantine processes. Pigeon
contradiction. QED.

## Solution 2 [T]: HT equivalence

(See README's proof.) The construction is: AB-broadcast each
process's input, decide the first delivered value (gives a
consensus oracle); run consensus per slot, AB-broadcast the
decided value (gives an AB oracle).

## Solution 3 [T]: causal broadcast from RB

Add a vector clock `VC[i]` per process. On RB-broadcasting,
increment `VC[me]` and tag the message with the new vector. On
RB-deliver, buffer until all causal predecessors (entries with
smaller VC) are delivered. The CB property follows from the
buffering rule plus VC's monotonicity.

## Solution 4 [P]: Bracha under omission

Bracha tolerates arbitrary omissions as long as `2f + 1`
honest readies still get through. For `n = 4, f = 1`, that
means `3 of 3` honest readies (all of them) must survive. With
30% drop, the probability of all surviving is `0.7^k` where `k`
is the number of paths; in practice the redundancy across
processes saves it (any one of the readies coming from any
honest process suffices for that recipient).

## Solution 5 [P]: threshold-signature replacement

Replace `Echo` and the per-content echo collection with: each
process produces a *partial signature* on the content and sends
to the sender; the sender aggregates `n - f` partial sigs into
a threshold sig and broadcasts it as a `Ready`. Recipients
verify the threshold sig and apply the same `2f + 1` ready
delivery rule.

Bandwidth: from `O(n^2)` echoes to `O(n)` partial sigs plus
`O(n)` ready broadcasts. SNARK circuit: the `Echo` collection
collapses to a single threshold-sig verification (one pairing).

## Solution 6 [F]: typeclasses

```text
class ReliableBroadcast (M : Type) where
  broadcast : NodeId -> M -> Effect
  deliver   : NodeId -> M -> Effect
  validity  : ...
  integrity : ...
  agreement : ...

class AtomicBroadcast (M : Type) extends ReliableBroadcast M where
  total_order : ...
```

cslib's `InferenceSystem` framework expresses these as
deduction rules; safety properties become derived theorems over
the inference system.

## Solution 7 [V]: TOB SNARK layout

Public input:

- *Commitment to the prefix.* Poseidon hash of the ordered
  sequence's Merkle root.
- *Signature aggregate.* BLS aggregate signature of `2f + 1`
  validators on the commitment.
- *Validator-set bitmap.* `n`-bit bitmap of signers.
- *Slot witness.* Current slot number with timestamp witness.
- *Previous proof's commitment.* For recursion.

Verifier checks: pairing equation on the BLS aggregate, signer
bitmap card >= 2f + 1, slot witness fresher than `delta_max`,
recursive proof verification. This is the structure of Mina's
Pickles per-slot proofs.
