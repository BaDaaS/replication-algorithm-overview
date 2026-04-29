# Module 0000 Solutions

Reference solutions to the exercises in [`Exercises.md`](Exercises.md).
Solutions are intentionally written in the style expected of a
graduate take-home: complete, but not over-explained.

## Solution 1 [T]: determinism and output equivalence

Let `s_0^{(1)} = s_0^{(2)} = s_0` and `o_1, ..., o_k` be the shared
operation sequence. Define
`s_i^{(j)} = delta(s_{i-1}^{(j)}, o_i)` and
`y_i^{(j)} = observe(s_{i-1}^{(j)}, o_i)` for `j in {1, 2}`.

Claim: for all `i`, `s_i^{(1)} = s_i^{(2)}` and
`y_i^{(1)} = y_i^{(2)}`.

By induction. Base case `i = 0`: `s_0^{(1)} = s_0^{(2)} = s_0`
and the empty output sequence is trivially equal. Inductive step:
assume the claim holds at `i - 1`. Then

```
s_i^{(1)} = delta(s_{i-1}^{(1)}, o_i)
          = delta(s_{i-1}^{(2)}, o_i)        [by IH]
          = s_i^{(2)}                        [by determinism of delta]

y_i^{(1)} = observe(s_{i-1}^{(1)}, o_i)
          = observe(s_{i-1}^{(2)}, o_i)      [by IH]
          = y_i^{(2)}                        [by determinism of observe]
```

The determinism axiom is used once per step, on the equation
`delta(s, o) = delta(s, o)` and similarly for `observe`. Without
it, the inductive step would fail because two replicas could
legitimately compute different successors for the same `(s, o)`.

## Solution 2 [T]: SMR / atomic broadcast equivalence

**TOB to SMR.** Given `tob-broadcast` with the standard
properties (validity, integrity, agreement, total order, eventual
delivery), define the SMR oracle as: a client submits an operation
`op` by calling `tob-broadcast(op)`; the SMR oracle delivers `op`
to a replica `r` whenever `tob-deliver` fires at `r`. Total order
of the SMR oracle follows from total order of TOB; validity and
eventual delivery transfer directly. The inverse mapping is from
op-deliveries to a single sequence by composing the per-replica
delivery sequences (which agree by total order).

**SMR to TOB.** Use the *log state machine*
`L = (List Op, [], Op, Op, lambda l. lambda op. l ++ [op],
lambda l. lambda op. op)`. Submitting an operation through this
SMR causes every honest replica to extend its local log with the
same operation in the same position; the output of the `apply`
function at index `i` is the `i`-th delivered operation.
Calling this output the `tob-deliver` event recovers TOB.

In both directions the constructions preserve the failure model
under which the source primitive is built; this is why SMR and TOB
are interchangeable in algorithm design.

## Solution 3 [P]: detect determinism failures

Defining `AddRandom` that reads a non-replicated source breaks the
hypothesis that `apply` is a function of `(s, op)`. Two replicas
applying `AddRandom` arrive at distinct states, so the conclusion of
SMR safety fails.

Production mitigations:

- **(a) Compile-time prohibitions.** The Aptos Move VM, Sui Move,
  and Solana SBF restrict the execution environment so that
  programs cannot read non-replicated sources (no system clock
  except a consensus-provided one, no PRNG except a
  consensus-provided one, no environment variables, deterministic
  arithmetic, deterministic map iteration). The JVM does not
  guarantee any of these by itself, which is why Hyperledger Fabric
  ships its own deterministic execution environment for chaincode.
- **(b) Runtime detection.** Cosmos chains exchange the SHA-256 of
  the post-block state in every commit. A divergence triggers a
  consensus failure (an "AppHash mismatch"). The detection cost is
  one hash per block; the response is operational (out-of-band
  recovery, validator restart, or chain halt).

## Solution 4 [P]: snapshotting and log compaction

Add a `Snapshot { seq, state }` message. A follower applies a
snapshot by setting its replica's local state to `state` and its
implicit log length to `seq + 1`. Subsequent `Commit { seq', op }`
messages are applied in order if `seq' = current_log_length` and
buffered otherwise.

Invariant required for SMR safety:

```
Snapshot { seq, state } is honest only if the replica that
emitted it had committed exactly the operations o_0, ..., o_seq
yielding state when applied to s_0.
```

In production, this invariant is enforced by signing snapshots and
requiring the snapshot's signed hash to match the deterministic
state-hash chain. Etcd, ZooKeeper, and CometBFT all do this.

The trade-off is between snapshot frequency (more frequent = faster
recovery, less storage; less frequent = lower bandwidth, more
catch-up). Etcd's default is one snapshot per 100k entries.

## Solution 5 [F]: pseudo-Lean SMR signature

```text
-- Pseudo-Lean (ASCII only).

structure StateMachine where
  S       : Type
  s0      : S
  Op      : Type
  Out     : Type
  delta   : S -> Op -> S
  observe : S -> Op -> Out

structure Replica (M : StateMachine) where
  state   : M.S
  log     : List M.Op
  outputs : List M.Out

def apply_one
    (M : StateMachine) (r : Replica M) (op : M.Op)
    : Replica M :=
  { state   := M.delta r.state op,
    log     := r.log ++ [op],
    outputs := r.outputs ++ [M.observe r.state op] }

-- Reuse cslib's LTS:
--   states : Replica M
--   labels : M.Op
--   step   : Replica M -> M.Op -> Replica M -> Prop
--   step r op r' := r' = apply_one M r op

-- Safety predicates:
def PrefixAgreement
    (M : StateMachine) (r1 r2 : Replica M) : Prop :=
  r1.log <+: r2.log \/ r2.log <+: r1.log

def ValueAgreement
    (M : StateMachine) (r1 r2 : Replica M) (i : Nat) : Prop :=
  i < r1.log.length /\ i < r2.log.length ->
    r1.log[i]? = r2.log[i]?

-- The proof obligation discharged externally is the existence of
-- a total-order oracle:
class TotalOrderOracle (M : StateMachine) (R : Type) where
  feed       : R -> Nat -> Option M.Op
  consistent : forall (r1 r2 : R) (i : Nat),
    feed r1 i = none \/ feed r2 i = none \/
    feed r1 i = feed r2 i
```

The cslib `Cslib.Foundations.LTS.Basic` namespace gives `LTS`,
`SmallStep`, and reachability. The natural reuse is to express
`Replica M` as the state of an LTS whose labels are `M.Op` and
whose transition relation is the singleton step above.

## Solution 6 [F]: equivalent characterisations of "honest replica"

Let `r` be a replica of `M`.

- **(1) Local execution.** `r` is *trace-honest* if there exist
  `s_0, s_1, ..., s_k = r.state` and `o_1, ..., o_k = r.log` with
  `s_i = delta(s_{i-1}, o_i)` for all `1 <= i <= k`.
- **(2) Network interface.** `r` is *interface-honest* if every
  message `r` emits to a peer `r'` is computable from `r.state`
  and the protocol's transition table; `r` does not emit messages
  whose witness is a state `r` has not actually reached.
- **(3) Cryptographic interface.** `r` is *signature-honest* if,
  for every signed message `m` with signature `sigma_r(m)`, there
  is a state `s` in `r`'s execution prefix and an inference rule of
  the protocol that justifies signing `m` from `s`.

Equivalence (sketch): under the modelling assumption that signatures
are unforgeable and the transition relation is fully captured by
the protocol, signature-honesty implies interface-honesty (a signed
message is a network message); interface-honesty implies trace-
honesty (every message witnesses a transition, and a chain of
witnesses fixes a unique trace from `s_0`); trace-honesty implies
signature-honesty (a trace from `s_0` lets the replica sign every
message that the protocol licenses from any visited state).

This decomposition is exactly the one used in Velisarios's Coq
formalisation of PBFT and in Bythos's proof framework.

## Solution 7 [V]: a verifiable counter SMR

Statement language: a triple `(s, s', op-list)` where `s, s' : F_p`
(field of prime order `p` larger than the largest counter value)
and `op-list : List Op` with `Op = Add(F_p) | Mul(F_p) | Read`.
The relation `R(s, s', op-list)` holds iff applying the operations
to `s` (with field arithmetic) yields `s'`.

Constraint system. Choose a SNARK with a Plonkish or R1CS
arithmetisation. Each operation contributes:

- `Add(d)`: a single linear constraint `s_i = s_{i-1} + d`.
- `Mul(f)`: a single multiplication gate `s_i = s_{i-1} * f`.
- `Read`: no constraint on the state, but a copy constraint
  `out_i = s_{i-1}`.

The whole circuit has `O(|op-list|)` constraints. Recursive
composition (Halo 2 / Pickles) gives a constant-size proof for an
unbounded chain of such transitions.

Missing oracle component. The above proves that the state machine
was applied correctly to `op-list`, but says nothing about whether
`op-list` is the output of an honest ordering oracle. To reach the
`deployed` tag, one must additionally prove that `op-list` was
agreed upon by a quorum of validators of the consensus protocol;
this is why no module-0000 construction can be `deployed`.
Mina's Pickles closes the gap by also proving a Samasika
fork-choice transition; zk-rollups close the gap externally by
posting validity proofs to a separate L1 consensus.

Tag: `friendly` (the state machine itself is SNARK-friendly), but
the *protocol composed with the consensus* is `partial` until
the oracle component is supplied.

## Solution 8 [V]: non-SNARK-friendly determinism

- **Keccak-256.** Substitutes used in production: Poseidon (in
  Polygon zkEVM, Scroll, Aleo, Mina), Rescue-Prime (in older
  StarkWare designs), Anemoi (in some Plonk-based protocols). Cost
  per Keccak-256 invocation: roughly 150-400 R1CS constraints under
  Poseidon-2 versus tens of thousands for a direct Keccak circuit
  (Polygon zkEVM whitepaper, 2022; Scroll zkEVM book, 2023).

- **ECDSA over secp256k1.** Substitutes: BLS over BLS12-381 (Aleo,
  Aztec), Schnorr over Pasta (Mina), EdDSA over Edwards curves
  (Aztec). Cost per signature verification: secp256k1 ECDSA in a
  zkSNARK costs `~1.5M` constraints (Polygon zkEVM, "verify
  precompile" documentation); Schnorr over Pasta in Mina is
  approximately `~3k` constraints (Pickles documentation). The two
  orders of magnitude come from the curve choice and from the lack
  of a SNARK-friendly modular inversion routine in secp256k1.

- **Mapping iteration order.** Production zkEVMs handle this by
  serialising the state into a Merkle Patricia trie (Ethereum's
  native commitment) or a Verkle trie, and proving inclusion / non-
  inclusion of accessed keys, side-stepping the iteration question.
  Polygon zkEVM and Scroll both use this approach. Solana's
  validity proofs (planned via the SVM zk-extensions) instead
  enforce a deterministic iteration order at the language level.
