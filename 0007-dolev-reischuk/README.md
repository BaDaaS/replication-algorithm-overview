# 0007: The Dolev-Reischuk Message-Complexity Lower Bound

## Historical context

Dolev and Reischuk's "Bounds on Information Exchange for
Byzantine Agreement" (JACM 1985, [`dolev1985reischuk`]) gave the
seminal message-complexity lower bound for Byzantine agreement:
`Omega(n * f)` messages are required by any deterministic
protocol tolerating `f` Byzantine faults.

This complements the round-complexity bound of Dolev-Strong 1983
(module 0006). Together, they fix the asymptotic resource
requirements of synchronous Byzantine agreement: at least
`f + 1` rounds and at least `Omega(n * f)` messages.

The bound has shaped two decades of BFT engineering. PBFT 1999
runs in `O(n^2)` messages per view (matching `n * (n - 1)`
roughly). HotStuff 2019 improved this to `O(n)` per view by
*amortising* the cost over the QC; the Dolev-Reischuk bound
applies *per agreement instance*, but a streamlined-BFT
pipeline reuses messages across many instances. Modern
DAG-based BFT (Bullshark, Mysticeti) further amortises by
having every honest validator broadcast at most one block per
round, with all decisions inferred from the DAG structure.

## System and threat model

Same as module 0006: synchronous, deterministic, `f` Byzantine
out of `n`, with or without authentication.

## Theory

### Theorem (Dolev-Reischuk lower bound)

Any deterministic Byzantine agreement protocol that tolerates
`f` faults exchanges at least `(n - f) * f / 2` messages in the
worst case. In particular, the message complexity is
`Omega(n * f)`.

*Proof sketch.* The adversary's strategy: in any execution,
the `f` Byzantine processes do not initiate any messages. Honest
processes therefore must rely on messages exclusively from their
honest peers. Consider the number of *distinct messages* an
honest process must receive before it can decide.

By a counting argument: there are `f` "missing" senders (the
Byzantine processes who could but do not communicate), and each
honest process must somehow learn the values of the others. If
the protocol exchanges fewer than `(n - f) * f / 2` honest-
honest messages, then at least one honest process has fewer
than `f / 2` reports about other honest processes, and the
adversary's silent-Byzantine strategy creates an
indistinguishable scenario in which the protocol decides
incorrectly.

The full argument is a careful pigeonhole over the
honest-to-honest message graph. Dolev-Reischuk 1985 Theorem 3.1
gives it in detail.

QED (sketch).

### Lower bound holds with or without authentication

Dolev-Reischuk's argument does not use the structure of the
messages, only the *count*. Authentication does not lower the
asymptotic message count; it only changes the per-message
content (an authenticated chain).

### Tight upper bounds

- *PBFT* runs in `O(n^2)` messages per view; with `n = 3f + 1`,
  this is `O(n * f)`. Matches the lower bound.
- *HotStuff (chained)* runs in `O(n)` per view but each view
  produces only one decision, so amortised across `f`
  decisions to recover from `f` view changes the cost is
  `O(n * f)`.
- *Bull-shark / Narwhal* runs in `O(n)` per round of DAG
  growth; rounds amortise into commits.

### Communication complexity (bytes)

The Dolev-Reischuk bound is on *message count*, not bytes.
Aggregated signatures (BLS) shrink each message but do not
reduce count. Modern *communication-complexity* lower bounds are
`Omega(n * f)` in the standard model and `Omega(n)` in the
authenticated model with adaptive-corruption-resistant
threshold signatures (Spiegelman-Aleo 2020; the
"signature-aggregation lower bound").

## Practice

### Why production protocols are not under the bound

A naive read of "PBFT uses `O(n^2)` messages" might suggest
optimisation. But Dolev-Reischuk says no asymptotic improvement
is possible without changing the model. The wins come from:

- *Constant factors.* All-to-all gossip with batching and
  pipelining reduces effective per-message cost.
- *Amortisation.* HotStuff's chained QC reuses the prepare
  exchange across multiple commits.
- *Decoupling.* Narwhal-Tusk separates dissemination (`O(n)` per
  round) from agreement, so agreement only "decides" on already-
  disseminated blocks.

In throughput terms, modern DAG-BFT systems achieve `100k+ TPS`
not by beating Dolev-Reischuk but by amortising it over many
transactions per round.

### Network engineering

The bound has direct operational consequences. A 100-validator
PBFT network with `f = 33` exchanges `~6600` messages per
view; at 1 KB per message, that is `~6.6 MB` per view. Production
deployments engineer the network for this baseline and use
batching to amortise.

## Formalisation aspects

### Pseudo-Lean theorem statement

```text
def MessageCount (P : SyncProtocol) (sched : Schedule) : Nat :=
  -- count messages exchanged in the run

theorem dolev_reischuk_lower_bound
    (n f : Nat) (h_nf : n > 3 * f) :
    forall (P : SyncProtocol) (h_correct : IsByzAgreement P f),
      exists (sched : Schedule),
        MessageCount P sched >= (n - f) * f / 2 := by
  sorry  -- pigeonhole on honest-to-honest message graph
```

### Pigeonhole as a Lean lemma

```text
lemma pigeonhole_witness :
    forall (G : Finset Process) (E : Finset (Process × Process)),
    G.card = n - f ->
    (forall (g : Process) (h : g ∈ G), G.degree_in E g >= f / 2) ->
    E.card >= (n - f) * f / 2 := by
  sorry
```

The proof is straightforward graph-theoretic counting. cslib's
graph module (in development) or Mathlib's `SimpleGraph` would
hold it.

## Verifiability and circuit encoding

**Tag: `na`** for the lower bound; `partial` to `friendly` for
upper-bound protocols.

The Dolev-Reischuk message-count lower bound has a direct
verifiability consequence: a SNARK proof of correct execution
must, in the worst case, encode `Omega(n * f)` messages. With
`n = 100, f = 33`, that is roughly `3300` messages to encode. At
`~3k` constraints per signature verification (Schnorr over
Pasta), the per-instance proving cost is `~10^7` constraints
without aggregation. With BLS aggregation:

- Per-QC proving cost: one pairing (`~10^6` constraints) plus
  the bitmap of signers (`~n` constraints).
- Per-instance: a few QCs (typically 2-3 in PBFT-style
  protocols), so `~3 * 10^6` constraints.

The takeaway is that the lower bound determines the prover's
*minimum* work. The verifier's work is independent of `f` (one
or two pairings), so verification is succinct regardless. This
asymmetry, prover work scales with the protocol's message
complexity, verifier work is constant, is the structural
benefit of SNARK-based verifiable replication.

## Known attacks and limitations

- *Lower bound on the messages exchanged, not on bytes.*
  Aggregation reduces bytes, not messages.
- *Synchronous model.* The bound is for synchronous protocols.
  Partial-synchrony and asynchronous variants have different
  (often higher) bounds; see Bracha-Toueg 1985 and
  Mostefaoui-Moumen-Raynal 2014.
- *Randomised protocols.* Randomisation can reduce expected
  message count but not worst-case (Aspnes 2003).

## Implementation notes

The crate provides a *message counter* for the simulator: a
wrapper adversary that records the total number of envelopes
intercepted. Tests instantiate the leader-broadcast SMR from
module 0000 with `n = 4, 8, 16` and verify the message count
scales as expected (`O(n^2)` for the all-to-all leader fanout).

This is empirical, not a formal lower-bound demonstration. The
counter is a useful instrument for the rest of the course: every
later module can use it to sanity-check its protocol's per-view
message count.

## References

- Dolev and Reischuk, "Bounds on Information Exchange for
  Byzantine Agreement", JACM 1985. [`dolev1985reischuk`].
- Dolev and Strong, "Authenticated Algorithms for Byzantine
  Agreement", SICOMP 1983. [`dolev1983`].
- Spiegelman, Aleo, "Communication-Efficient and Crash-
  Quiescent Atomic Broadcast", DISC 2020.
- Aspnes, "Randomized protocols for asynchronous consensus",
  Distributed Computing 2003.

See also [`HISTORY.md`](../HISTORY.md), section "1980 to 1985".
