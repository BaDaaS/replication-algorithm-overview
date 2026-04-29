# 0084: Bitcoin Backbone Protocol

## Historical context

Garay, Kiayias, and Leonardos published "The Bitcoin Backbone
Protocol" at Eurocrypt 2015. It is the first rigorous
cryptographic analysis of Nakamoto consensus, abstracting
Bitcoin's core into a simulator-based protocol and proving its
properties under the random-oracle model.

Before GKL 2015, Nakamoto's whitepaper had only an informal
heuristic argument. Practitioners trusted the longest-chain rule
because nothing had broken; theorists could not say what it
guaranteed. GKL changed that by formally specifying:

1. The protocol abstraction (proof-of-work, longest-chain rule).
2. The execution model (synchronous rounds, computational bound on
   adversary).
3. Three properties that any "robust transaction ledger" must
   satisfy, and proofs that the backbone protocol satisfies them.

The framework was extended to partially synchronous and
asynchronous networks by Pass-Seeman-Shelat 2017 (module 0085).

## System and threat model

- **Network.** Synchronous rounds. Honest messages delivered
  within one round.
- **Failures.** A computational bound: the adversary controls a
  fraction `t/n < 1/2` of total hashing power, modelled as queries
  to a random oracle.
- **Cryptography.** Random oracle for SHA-256 (idealised hash).
  Each party may query the oracle `q` times per round.
- **Goal.** Prove three properties: common prefix, chain quality,
  chain growth.

### Adversary model

The adversary in GKL is *Byzantine*: it can fork honest miners,
withhold blocks, and rush messages. Importantly, it cannot
exceed its hashing budget; the random-oracle model precludes
finding low-target hashes faster than honest parties of the same
power.

## Theory

### Three core properties

1. **Common prefix (`CP_k`).** For any two honest parties `P_1`
   and `P_2` and any rounds `r_1 <= r_2`, the chain held by
   `P_1` at round `r_1`, with the last `k` blocks dropped, is a
   prefix of the chain held by `P_2` at round `r_2`.
2. **Chain quality (`CQ_{l, mu}`).** In any window of `l`
   consecutive blocks of an honest party's chain, at least
   `mu * l` blocks are mined by honest parties.
3. **Chain growth (`CG_{tau, s}`).** In any window of `s`
   consecutive rounds, an honest party's chain grows by at least
   `tau * s` blocks.

GKL prove that the backbone protocol satisfies all three under
the assumption `f := alpha * q * n` (the per-round mining
probability) is small (the *honest-majority condition*) and the
fraction of adversarial power is below `1/2`.

### Theorem (informal)

Let `alpha` be honest hashing fraction, `beta = 1 - alpha`. If
`alpha * (1 - 2 * f) >= (1 + delta) * beta` for some
`delta > 0`, then the backbone protocol satisfies:

- *CP_k* with overwhelming probability in `k`.
- *CG_{tau, s}* with `tau >= (1 - delta) * f * alpha`.
- *CQ_{l, mu}* with `mu >= 1 - (1 + delta) * beta / alpha`.

The bound `f << 1` rules out frequent forks; the gap `delta`
ensures honest blocks accumulate work faster than adversarial
ones.

### Three applications proved on top

GKL show that any protocol satisfying CP, CG, CQ implements:

1. **Robust public transaction ledger** (BFT-style SMR).
   Transactions in honest blocks at depth `>= k` are *persistent*
   and the ledger has *liveness*.
2. **Byzantine agreement.** Two-phase BA construction using the
   ledger as a broadcast primitive.
3. **Public-randomness beacon.** Hash of deep blocks as
   unbiasable random source.

Each is reducible to the three core properties of the backbone.

### Comparison with related analyses

| Analysis      | Network         | Adversary      | Bound on `t/n` | Year |
| ------------- | --------------- | -------------- | -------------- | ---- |
| Nakamoto 2008 | informal        | static         | `< 1/2`        | 2008 |
| GKL 2015      | synchronous     | static, Byz    | `< 1/2`        | 2015 |
| Pass et al.   | partial sync    | static, Byz    | `< 1/2`        | 2017 |
| GKL 2017      | sync, adaptive  | adaptive       | `< 1/2`        | 2017 |
| BMTZ 2018     | bounded delay   | adaptive       | `< 1/2`        | 2018 |

The synchronous-only assumption of GKL 2015 is its main
weakness; subsequent papers strengthened the network model.

### Why this paper matters

- *First complete proof.* Before GKL, the longest-chain rule was
  folklore; afterwards, it had a theorem.
- *Clean abstractions.* CP / CG / CQ are now standard primitives;
  every PoW analysis since GKL frames its result in these terms.
- *Reduction approach.* Decoupling the chain protocol from the
  ledger lets later work substitute different chain protocols
  (e.g., Ouroboros) and inherit the application proofs.

## Practice

The Bitcoin backbone protocol is not an implementation; it is
the abstraction layer between Bitcoin and its formal proof. Real
Bitcoin clients (Bitcoin Core, btcd) implement many heuristics
that are not in the backbone (mempool prioritisation, peer
scoring, compact blocks). The backbone strips these away to
focus on the chain rule and the PoW puzzle.

### Implementation notes

- *Round-based execution.* GKL assumes synchronous rounds; real
  Bitcoin runs continuously. The match is via
  rounds = block intervals.
- *Random oracle.* SHA-256 is modelled as a random oracle; this
  is the standard cryptographic assumption for hash-based
  protocols.
- *Difficulty assumed constant.* GKL fix the target; later work
  (e.g., GKL 2017) extends to dynamic difficulty.

## Verifiability and circuit encoding

**tag: `partial`.**

The backbone abstraction is the natural target for SNARK-based
light clients: a verifier checks CP / CG / CQ by checking the
chain header sequence. The dominant cost is SHA-256 evaluation
(see module 0083).

Key insight for circuit design: only the chain header sequence
matters for the backbone properties. Transactions can be
committed via Merkle root and verified separately. This permits
recursive header-only proofs (Mina-style) of the chain prefix.

## Known attacks and limitations

- *Synchrony assumption.* Real networks are partially
  synchronous; messages can be delayed beyond a round.
  Pass-Seeman-Shelat 2017 (module 0085) addresses this.
- *Static adversary.* The adversary's corruption set is fixed at
  the start. Adaptive corruptions are harder to analyse.
- *Honest majority of compute.* If `t/n >= 1/2`, all three
  properties fail.

## References

- Garay, Kiayias, Leonardos, "The Bitcoin Backbone Protocol:
  Analysis and Applications", Eurocrypt 2015.
- Garay, Kiayias, Leonardos, "The Bitcoin Backbone Protocol with
  Chains of Variable Difficulty", Crypto 2017.
- Pass, Seeman, Shelat, "Analysis of the Blockchain Protocol in
  Asynchronous Networks", Eurocrypt 2017.

## Implementation notes

The crate provides simple property checkers for the three
backbone properties (`common_prefix`, `chain_quality`,
`chain_growth`) operating on abstract block sequences. Tests
verify the predicates on simple constructed chains.

See also [`HISTORY.md`](../HISTORY.md), section "2009 to 2014".
