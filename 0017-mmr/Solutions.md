# Module 0017 Solutions

## Solution 1 [T]: BV-broadcast properties

*Validity.* If `> f` honest BV-broadcast `v`, then each
honest `q` receives `v` from those `> f` distinct senders.
Once `q` has `> f`, it broadcasts (echoes) `v`. With `n` honest
echoes amplifying, eventually each honest sees `> 2f` distinct
senders of `v`, triggering BV-deliver.

*Justification.* If `q` BV-delivers `v`, then `q` has received
`> 2f` `Bv(v)` messages. Of these, `> f` are honest. Hence
some honest process broadcast `v`.

*Uniformity.* If `q` BV-delivers `v`, then `> 2f` distinct
senders sent `v`, of which `> f` are honest. The `> f` honest
senders' `Bv(v)` messages eventually reach every other honest,
who echoes; eventually each honest sees `> 2f` and delivers.

QED.

## Solution 2 [T]: MMR termination

The argument mirrors Bracha 1987 (module 0015):

- BV uniformity ensures all honest agree on `bin_values_p`.
- The Aux phase's `> n - f` quorum gives `> 2f` honest
  contributions, so the `Aux` set converges.
- With probability `1/2`, the coin matches the dominant value,
  triggering decision in the next round.

Expected rounds `<= 4`.

## Solution 3 [T]: security comparison

| Compromise            | MMR           | CKS         |
| --------------------- | ------------- | ----------- |
| Hash function broken  | safety fails  | partial (auxiliary) |
| Threshold key broken  | not used      | safety fails |
| Coin source biased    | termination weakened | termination weakened |

MMR is more robust to PKI compromise (no signatures), more
fragile to hash-collision attacks (no signature backstop). CKS
is the inverse. In a deployment with strong hash functions and
PKI, the choice is engineering preference; in a PQ-anxious
deployment, MMR's signature-free design is preferable.

## Solution 4 [P]: two-phase BV

A two-phase BV (pre-vote, vote) replaces the amplification
rule:

```
phase 1 (pre-vote):
  broadcast (PreVote, v)
  wait for n - f PreVotes

phase 2 (vote):
  let v_seen := the value present in > 2f / 3 of PreVotes,
                or NULL
  broadcast (Vote, v_seen)
  wait for n - f Votes
  bin_values := { v : v != NULL and present in > 2f / 3 Votes }
```

Cost: `O(n^2)` per phase, `O(n^2)` total, same as MMR's
amplification. The structural difference: two phases instead of
the dynamic amplification, simplifying the formal analysis at
the cost of a fixed quadratic per-phase exchange.

## Solution 5 [P]: post-quantum coin

Post-quantum common-coin sources:

- *Hash-chain VDFs.* Compute a chain of hashes and wait `T`
  steps; the hash output is the coin. Hash-based, PQ-safe, but
  high latency.
- *PQ-threshold signatures.* Lattice-based threshold-Schnorr
  variants exist but are not yet practical.
- *PQ-VRF.* Lattice-based VRFs (module 0011 references) exist
  but are large.
- *Drand-PQ.* drand has explored PQ variants; not yet
  production.

For an MMR-PQ deployment, a hash-chain VDF is the simplest
choice. The trade-off is latency: the VDF takes `T` steps to
evaluate, slowing each round.

## Solution 6 [F]: pseudo-Lean BV

```text
class BinaryValueBroadcast (n f : Nat) (M : Type) where
  ...
  validity :
    forall (v : M),
    (count {p : Fin n, IsHonest p /\ Broadcast p v}) > f ->
    forall (q : Fin n), IsHonest q -> Eventually (Deliver q v)

theorem validity_from_amplification :
    ∀ v : M, ∀ q : Fin n, IsHonest q ->
    (count {p, IsHonest p /\ Broadcast p v}) > f ->
    Eventually (Deliver q v) := by
  intro v q hq h_count
  -- 1. q receives v from > f distinct honest senders.
  -- 2. q's amplification rule: count > f triggers echo.
  -- 3. After q's echo, every other honest sees one more.
  -- 4. By induction on number of honest, all honest reach
  --    > 2f and deliver.
  sorry
```

## Solution 7 [V]: zk-MMR

For `n = 100, f = 33`, per round:

- BV phase: ~`n * 2f` per-message hash checks (no signatures),
  `~33 * 200` constraints in Poseidon = `~6.6k` constraints
  per BV-broadcast, `~600k` total per round.
- Aux phase: `~200k` constraints (similar to BV).
- Coin: depends on source; if drand-BLS, `~10^6`.

Total per round: `~1.8 * 10^6` constraints. Slightly more than
CKS because of the extra BV-amplification overhead. Hash-based
attestations are cheaper per check than signature checks but
multiply due to the amplification rule.

## Solution 8 [V]: hash vs signature trade-off

| Attestation         | Per-check cost        | Witness size           |
| ------------------- | --------------------- | ---------------------- |
| Hash-based          | ~200 constraints      | one hash per echo (n bytes) |
| Signature (Schnorr) | ~3,000 constraints    | one signature (32 bytes) |
| Aggregate sig (BLS) | ~10^6 (one pairing)   | one aggregate (96 bytes) |

For a single attestation, hash-based wins. For `k`
attestations of the same value, signature aggregation wins
(linear vs sublinear). The crossover depends on the protocol:
MMR's BV amplification has many distinct attestations
(one per echo step), so hash-based is competitive; CKS's
threshold signature aggregates them all, so the pairing-check
amortises.

In practice, modern designs (Mina Pickles, Aleo snarkVM) use
both: hashes for per-message integrity, BLS aggregation for
quorum certificates.
