# Module 0004 Solutions

## Solution 1 [T]: PSL three-process case

See module 0002 Solution 2 for the same construction in the
failure-model context. The argument is the same: with `n = 3, f
= 1`, two indistinguishable schedules `S_1` (the Byzantine is
process 2) and `S_2` (the Byzantine is process 3) generate the
same view at process 1, but the protocol must produce different
outputs to satisfy validity in each. Determinism forces a
contradiction.

## Solution 2 [T]: induction on m

Induction hypothesis at `m - 1`: `OM(m-1)` is correct for any
system with at most `m - 1` faults and `n' > 3(m - 1)`.

Inductive step: consider `OM(m)` on a system with `n > 3m`
processes and `f <= m` faults.

- *Honest commander.* Every honest lieutenant `L_i` receives the
  commander's value `v` directly in step 1. In step 2, `L_i`
  acts as the commander of an `OM(m-1)` instance among the other
  `n - 1 > 3m - 1 >= 3(m-1) + 2` lieutenants, of which at most
  `m - 1` are faulty (the at most `m` faulty processes minus the
  commander, who is honest). By IH, every honest lieutenant
  records the same value `v_i = v` for `L_i`. Step 3's majority
  is unanimous on `v`.
- *Byzantine commander.* The commander may send different
  values to different lieutenants. Let `L_i, L_j` be honest. In
  step 2, both act as commanders of `OM(m-1)` among `n - 1 > 3m
  - 1 >= 3(m - 1) + 2` processes with at most `m - 1` faulty
  (the commander is faulty but absent from these subprotocols).
  By IH, each honest lieutenant records the same value for `L_i`
  (call it `r_i`) and the same value for `L_j` (call it `r_j`).
  Hence every honest lieutenant computes the same majority over
  `(r_i)_{i in [n-1]}`.

In both cases all honest lieutenants decide identically. QED.

## Solution 3 [T]: tightness of the round bound

Dolev-Strong 1983 prove that any deterministic synchronous
Byzantine agreement protocol with `f` faulty processes requires
at least `f + 1` rounds, even with authenticated messages
(signatures). The intuition: a Byzantine process can withhold its
contribution until round `f`, and the bound `f + 1` is the
smallest that lets honest processes either include or exclude the
withheld message safely. `OM(f)` and `SM(f)` both run in `f + 1`
rounds, matching the lower bound.

The Dolev-Strong proof is by adversary-strategy construction: in
each round the adversary keeps one specific process silent
towards a specific honest target; after `r < f + 1` rounds the
adversary has constructed indistinguishable executions with
different outputs.

## Solution 4 [P]: SM(1) sketch

A reference Rust extension (intentionally pedagogical, not
crypto-grade):

```rust
type Signer = NodeId;
type Sig = (Signer, u64); // (signer, hmac(key, value))

#[derive(Clone, Debug)]
enum SignedMsg {
    SignedOrder { v: Decision, chain: Vec<Sig> },
}

fn append_sig(
    chain: &mut Vec<Sig>, signer: NodeId, value: Decision, key: u64,
) {
    let h = hash_with_key(key, value);
    chain.push((signer, h));
}

fn verify_chain(chain: &[Sig], v: Decision, keys: &[u64]) -> bool {
    chain.iter().enumerate().all(|(i, (signer, h))| {
        keys.get(signer.0 as usize).copied()
            .map(|k| hash_with_key(k, v) == *h)
            .unwrap_or(false)
    })
}
```

Even with the commander Byzantine, the chain of `f + 1`
signatures bounds the maximum length such that *some* honest
lieutenant must have signed. By forwarding, every honest
lieutenant learns of every value with a length-`f+1` chain, and
the decision rule "decide the unique value, else default"
guarantees agreement.

## Solution 5 [P]: PBFT vs OM(1)

| OM(1) (synchronous) | PBFT (partial synchrony) |
| ------------------- | ------------------------ |
| Round 0: leader sends order | `pre-prepare`: leader sends order |
| Round 1: lieutenants forward | `prepare`: replicas echo |
| (decide on majority)         | `commit`: replicas commit on `2f + 1` `prepare`s |

The extra `commit` phase is required by partial synchrony for
*liveness*: in synchrony, a missing message is final (the round
ends), but in partial synchrony a missing message might just be
delayed. Without the commit phase, a leader change could be
ambiguous (some replicas have prepared with the old leader's
value, others with the new). The third phase ensures the value
is "locked in" before the leader can change.

## Solution 6 [F]: pseudo-Lean `SignedChain`

```text
class SignedChain (V : Type) (Sig : Type) where
  sign     : NodeId -> V -> Sig
  verify   : NodeId -> V -> Sig -> Bool
  -- distinct signers do not collide on values
  collisionFree :
    forall (i j : NodeId) (v : V) (s : Sig),
      verify i v s = true /\ verify j v s = true -> i = j

structure Chain (V : Type) (Sig : Type) where
  value   : V
  sigs    : List (NodeId × Sig)

def Chain.valid
    {V Sig : Type} [SignedChain V Sig] (c : Chain V Sig)
    : Bool :=
  -- pairwise distinct signers
  c.sigs.toFinset.card = c.sigs.length
  -- each signature verifies on the value
  /\ c.sigs.all (fun (i, s) => SignedChain.verify i c.value s)
```

Reuse: cslib's `HasFresh` for distinct names; Mathlib's
`Finset.card`, `List.toFinset`, `List.all`. The cryptographic
hypothesis `collisionFree` corresponds to the EUF-CMA security of
the underlying signature scheme; in a full formalisation it is
parameterised by a security game with negligible adversary
advantage, à la `IsCryptographicallyUnforgeable`.

## Solution 7 [V]: BLS-aggregated QC

A QC over a value `v` with `2f + 1` signers is a tuple
`(v, agg_sig, signer_set)` with `agg_sig` the BLS aggregate. The
SNARK encoding is a single pairing check:

```
e(agg_sig, G2) = product_{i in signer_set} e(H(v), pk_i)
```

In BLS12-381, this is one `Miller loop + final exponentiation`
inside the circuit. Halo 2 over Pasta or Plonk over BN254 admits
this efficiently if curve cycles are available; on bn254
(Mathlib has partial coverage), the cost per QC verification is
on the order of `~10^7` constraints per pairing in
straightforward encoding, dropped to `~10^6` with custom gates
(see "Vampire" 2023, "EVM zkBridge" 2022).

Why preferable to `SM(f)` chains in production:

- A QC is `O(1)` size regardless of `f`. A signed chain is `O(f)`
  size.
- Aggregation reduces verifier work from `f + 1` signature
  verifications to a single pairing check.
- The signer set is committed to once (an `n`-bit bitmap of `2f
  + 1` ones), which is also `O(1)` succinct.

Formalisation trade-off:

- BLS12-381 pairing-friendly group infrastructure is non-trivial
  to verify in Lean. Mathlib's `EllipticCurve` exists but
  pairing-specific machinery is ongoing work.
- Schnorr over Pasta needs a curve cycle (Pallas / Vesta) that
  Mathlib does not yet have in its library.
- Both encodings are realistic but require non-trivial library
  development. CSLib could host the abstract `SignedChain`
  typeclass and let downstream projects supply the concrete
  signature scheme.

## Solution 8 [V]: OM(1) in a circuit

Constraint inventory for OM(1), `n = 4`, `f = 1`, all signed:

- Round 0: 3 signature checks (commander's signed order to each
  lieutenant). At ~3k constraints each Schnorr-over-Pasta, ~9k
  constraints.
- Round 1: 6 signature checks (each lieutenant signs and forwards
  to 2 peers). ~18k constraints.
- Decision: 3 majority computations over (1 + 2) values each.
  Negligible (~100 constraints total).
- Chain-consistency: each lieutenant's recorded chain must verify
  the commander's signature on the original value, then the
  lieutenant's own signature. ~2 verifications per lieutenant per
  forward = 12 verifications. ~36k constraints.

Total: roughly 60-70k constraints for the full OM(1) execution
proof. With recursion (Halo 2 / Pickles), unbounded chains of
such instances admit a constant-size verifier proof.
