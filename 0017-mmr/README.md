# 0017: Mostefaoui-Moumen-Raynal Signature-Free Async ABA

## Historical context

Mostefaoui, Moumen, and Raynal published "Signature-Free
Asynchronous Byzantine Consensus with `t < n / 3` and `O(n^2)`
Messages" at PODC 2014 (with extension in JACM 2015). The
result is a binary asynchronous Byzantine agreement protocol
that achieves the optimal `f < n / 3` resilience and `O(n^2)`
messages per agreement, *without* using digital signatures or
threshold cryptography (apart from a common coin).

The "signature-free" qualifier matters: the protocol does not
need a public-key infrastructure or a complex DKG; only a
common coin is required. In some deployments this is significant
because:

- *Simpler trusted setup.* No DKG; the coin can be supplied by
  an external beacon (drand, VDF).
- *Robustness against signature-scheme breaks.* If the
  signature scheme is broken (post-quantum scenarios), the
  protocol's safety degrades gracefully.
- *Code simplicity.* No signature verification per message.

## System and threat model

- **Network.** Asynchronous, reliable channels.
- **Failures.** Byzantine, `f < n / 3`.
- **Common coin.** External oracle.
- **Cryptography.** Hash functions only (no signatures).
- **Goal.** Async binary Byzantine agreement, `O(n^2)`
  messages per instance.

## Theory

### Algorithm: binary value broadcast (BV)

MMR's key primitive is *binary value broadcast*. BV solves a
weaker problem than RB: it ensures that every honest process's
delivered set contains at most the values that *some* honest
process broadcast, but allows the set to be larger than 1.

```
algorithm BV-broadcast(v):
  state: bin_values_p := empty subset of {0, 1}
  upon BV-broadcast(v):
    broadcast(v)
  upon receive (v) from q:
    if (v) received from > f distinct senders and v not yet echoed:
      broadcast(v)
    if (v) received from > 2f distinct senders:
      bin_values_p := bin_values_p union {v}
      BV-deliver(v)
```

BV satisfies:

- *Validity.* If at least `f + 1` honest processes BV-broadcast
  the same `v`, every honest process eventually BV-delivers
  `v`.
- *Justification.* If `v` is BV-delivered, then some honest
  process BV-broadcast `v`.
- *Uniformity.* If `v` is BV-delivered at one honest, it is
  eventually BV-delivered at all honest.

### MMR ABA protocol

```
state: estimate := input bit; round := 1

loop:
  est := estimate
  BV-broadcast(est)
  wait until bin_values_p is non-empty
  // by uniformity, all honest see the same eventually

  broadcast (Aux, round, bin_values_p)
  wait until aux_set is delivered with values from
    n - f distinct senders
  let values := union of aux_set messages
  // a non-empty subset of bin_values_p

  coin := common-coin(round)
  if values = {b}:
    if b = coin: decide b
    else: estimate := b
  else (values = {0, 1}):
    estimate := coin
  round := round + 1
```

### Theorem (MMR 2014/2015 correctness)

For `f < n / 3` Byzantine and a fair common coin, MMR ABA
satisfies validity, agreement, and termination with
probability 1 in expected `O(1)` rounds.

*Proof.* The proof structure is essentially identical to
Bracha 1987, with BV-broadcast replacing Bracha RB. The
load-bearing properties are:

- *BV uniformity.* All honest see the same `bin_values` set.
- *Coin unbiasability.* As in Rabin/CKS.
- *Quorum intersection.* `n - f` Aux messages from any honest
  process intersect with any other honest's collection in at
  least one common honest sender.

QED (sketch). MMR 2015 JACM gives the full proof.

### Communication complexity

`O(n^2)` messages per BV-broadcast, plus `O(n^2)` for the Aux
phase. Total `O(n^2)` per round. With expected `O(1)` rounds,
total `O(n^2)` per agreement instance.

This matches the Dolev-Reischuk lower bound (module 0007) up
to constant factors.

## Practice

### Where MMR is used

- *HoneyBadger BFT.* Uses an MMR-style ABA in some
  implementation variants.
- *Dumbo and Dumbo2.* Build on MMR ABA for the binary
  agreement layer.
- *PaaS-style consensus.* Some consortium-blockchain
  deployments choose MMR for its signature-free property.

### Comparison with CKS

| Property             | CKS 2000      | MMR 2014/2015     |
| -------------------- | ------------- | ----------------- |
| Signatures required  | yes (threshold-RSA/Schnorr) | no |
| Common coin required | yes           | yes               |
| Trusted setup        | DKG           | coin beacon only  |
| Communication        | `O(n^2)`      | `O(n^2)`          |
| Round complexity     | `O(1)`        | `O(1)`            |
| Post-quantum ready   | depends on scheme | yes (with PQ coin) |

The MMR signature-free design is preferred when the deployment
needs simpler key management or post-quantum considerations
matter.

## Formalisation aspects

```text
class BinaryValueBroadcast (n f : Nat) where
  bv_broadcast : NodeId -> Bool -> Effect
  bv_deliver   : NodeId -> Bool -> Effect

  validity : forall v, (count {p : IsHonest p, BVBroadcast p v} > f) ->
             forall q, IsHonest q -> Eventually (BVDeliver q v)
  justification : forall p v, IsHonest p -> BVDeliver p v ->
                   exists q, IsHonest q /\ BVBroadcast q v
  uniformity : forall v, (exists p, IsHonest p /\ BVDeliver p v) ->
                forall q, IsHonest q -> Eventually (BVDeliver q v)

class MmrAba (n f : Nat) [BV : BinaryValueBroadcast n f] [CC : CommonCoin] where
  ...
```

The reduction "MMR ABA from BV + coin" is a clean Lean
formalisation target. The hash-only assumption simplifies the
crypto layer compared to CKS.

## Verifiability and circuit encoding

**Tag: `friendly`** for hash-based primitives; `partial` for the
in-circuit BV.

MMR's signature-free design is *less* SNARK-friendly than CKS
in one sense: the Bracha-style "echo on reception" structure
of BV requires per-message hash checks which, in a circuit,
add up. Per round in circuit:

- BV phase: verify `2f + 1` echo certificates per BV message.
  Without signatures, the echoes are trust-less, but the
  circuit must witness each echo as a public input.
  ~`200k` constraints in Schnorr-over-Pasta.
- Aux phase: same.
- Coin: depends on common-coin source.

In total, similar order to CKS (`~10^6` constraints per
round). The trade-off: simpler crypto (no signatures) but more
public-input witnesses. For zkBridges where signatures are
expensive, MMR's design is sometimes preferred.

## Known attacks and limitations

- *Coin biasability.* MMR delegates the coin to an external
  source; the bias resistance is inherited from that source.
- *Hash collision attacks.* Without signatures, Byzantine
  processes can sometimes equivocate using hash collisions if
  the hash is weak. Production deploys with collision-resistant
  hashes (SHA-256, BLAKE2).
- *Throughput limits.* `O(n^2)` per agreement matches
  Dolev-Reischuk; no improvement over CKS.

## Implementation notes

The crate provides an MMR-style ABA over the simulator. The BV
phase is implemented by tracking per-(round, value) echo and
deliver counts. The Aux phase and decision rule mirror CKS
(module 0016).

Tests: validity case for `n = 4, f = 1` decides in round 1.

## References

- Mostefaoui, Moumen, Raynal, "Signature-Free Asynchronous
  Byzantine Consensus with `t < n / 3` and `O(n^2)` Messages",
  PODC 2014.
- Mostefaoui, Moumen, Raynal, "Signature-Free Asynchronous
  Binary Byzantine Consensus with `t < n / 3, O(n^2)`
  Messages, and `O(1)` Expected Time", JACM 2015.
- Crain, "A Simple and Efficient Asynchronous Randomised
  Binary Byzantine Consensus Algorithm", arXiv 2020.

See also [`HISTORY.md`](../HISTORY.md), section "2009 to 2014".
