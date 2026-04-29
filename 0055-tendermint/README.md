# 0055: Tendermint

## Historical context

Jae Kwon's 2014 master's thesis "Tendermint: Consensus
without Mining" introduced Tendermint; Buchman's 2016
master's thesis formalised the production protocol;
Buchman-Kwon-Milosevic 2018 (technical report) gave the
canonical formal description. Tendermint is a streamlined-
BFT precursor to HotStuff: a per-height three-step protocol
(propose, prevote, precommit) with leader rotation per round
and a permanent commit on `2f + 1` precommits.

Tendermint became the consensus engine of Cosmos. Production
deployments (Cosmos Hub, Osmosis, Celestia, Sei, dYdX v4,
Berachain) collectively secure billions of dollars of
value as of 2025. The CometBFT fork (post-Tendermint Inc
dissolution) is the active maintenance line.

## System and threat model

- **Network.** Partial synchrony.
- **Failures.** Byzantine, `f < n / 3` (or weighted-stake
  variant in PoS).
- **Cryptography.** Ed25519 signatures.
- **Goal.** SMR over a finite, dynamic validator set.

## Theory

### Three-step round

Per height `h`, per round `r`:

```
1. propose:   leader (= round-robin r mod n) broadcasts
              proposal block.
2. prevote:   each validator broadcasts prevote on the
              proposal (or nil if invalid / late).
3. precommit: on collecting 2f + 1 prevotes for a value,
              broadcast precommit.

on collecting 2f + 1 precommits for a value: commit.
on timeout: increment round, restart.
```

The two voting phases (prevote + precommit) are similar to
PBFT's prepare + commit. The leader rotation per round is
similar to Spinning (module 0048).

### Locking

Tendermint has explicit *locking*: once a validator votes
precommit on a value, it stays "locked" on that value until
it sees `2f + 1` prevotes for a different value at a higher
round. The locking rule is the load-bearing safety
invariant.

### Theorem (Tendermint correctness)

Under partial synchrony with `f < n / 3`, Tendermint
satisfies linearisable SMR safety and liveness.

Buchman 2016 thesis provides the full proof.

### Connection to HotStuff

HotStuff (module 0056) refines Tendermint:

- Adds a *prepare* phase (4 phases instead of 3).
- Achieves *optimistic responsiveness*: bounded latency
  under stable network.
- Compatible with chained QC pipelining for `O(n)` per
  decision (vs Tendermint's `O(n^2)`).

Tendermint's structural advantage: simpler, easier to
implement and reason about. HotStuff's: better throughput
under non-adversarial conditions.

## Practice

### Cosmos / CometBFT deployment

- *Cosmos Hub.* Reference deployment.
- *Hundreds of chains.* Connected via IBC.
- *Stake-weighted voting.* PoS-native: validator's vote
  weight proportional to stake.
- *Vote extensions.* CometBFT v0.38+ allows applications to
  extend votes with custom data.
- *ABCI++.* Application-blockchain-interface v2: gives the
  app layer control over proposal preparation, vote
  extension, and finality.

### Slashing

CometBFT explicitly handles equivocation:

- *Double-prevote.* Same validator prevoting two different
  values at the same height-round. Slashable.
- *Double-precommit.* Same. Slashable.

The `evidence` module collects and verifies slashing
evidence; the application layer applies penalties.

## Formalisation aspects

Tendermint has been formalised in:

- *Galois TLA+ spec* (2018).
- *Coq via Velisarios* (Rahli 2018).
- *Apalache TLA+* (Konnov-Tran-Widder 2020).

```text
structure TendermintState where
  height       : Nat
  round        : Nat
  step         : Step  -- Propose | Prevote | Precommit
  locked_value : Option Value
  locked_round : Option Nat
  valid_value  : Option Value
  valid_round  : Option Nat
```

## Verifiability and circuit encoding

**Tag: `friendly`.**

Per-block: BLS-aggregated 2f + 1 precommit certificates =
~10^6 constraints (one pairing).

Production: Cosmos chains' light-clients (used by IBC,
zk-bridges) verify precommit aggregates. zk-bridges to/from
Cosmos chains (Polyhedra, Succinct's Telepathy) verify
Tendermint signatures in SNARKs at constraint cost ~10^6
per block.

## Known attacks and limitations

- *Quadratic message complexity.* `O(n^2)` per height; at
  large validator counts (>200), bandwidth becomes a
  constraint. CometBFT typically caps validator count at
  ~150.
- *Liveness under attack.* A faulty leader can stall the
  round; round-robin moves on. Worst-case `f + 1` rounds.

## Implementation notes

The crate provides type-level definitions for Tendermint's
state machine (Step enum and TendermintState).

## References

- Kwon, "Tendermint: Consensus without Mining", 2014.
- Buchman, "Tendermint: Byzantine Fault Tolerance in the
  Age of Blockchains", master's thesis 2016.
- Buchman, Kwon, Milosevic, "The latest gossip on BFT
  consensus", 2018.

See also [`HISTORY.md`](../HISTORY.md), section "2015 to 2019".
