# Module 0019 Solutions

## Solution 1 [T]: 2PC blocking

A participant `p` in PREPARED state has voted YES and is
awaiting the coordinator's COMMIT or ABORT. The coordinator
crashes after collecting all votes but before broadcasting the
decision. `p`'s local state cannot determine the global
decision because:

- *If all voted YES.* The coordinator may have committed (and
  notified some participants before crashing) or may have
  crashed before deciding.
- *If at least one voted NO.* The coordinator may have aborted
  (and notified some) or may not yet have decided.

Without communicating with another participant who has heard
the decision, `p` cannot proceed without risking inconsistency
with the (possibly committed) decision. The only safe action
is to wait for the coordinator's recovery.

## Solution 2 [T]: 3PC recovery

3PC's PRE_COMMIT round implements a "lock-in" before the final
COMMIT. Recovery on coordinator failure:

- Surviving participants exchange their states.
- If any participant is in COMMIT state, all surviving commit.
- If any is in PRE_COMMIT (and none is in ABORT), all
  surviving commit.
- If no participant is in PRE_COMMIT, all surviving abort.

The PRE_COMMIT state is the "decision is irrevocably commit"
phase. By the synchrony assumption, all participants either
reached PRE_COMMIT or did not; no inconsistency arises.

## Solution 3 [T]: 3PC unsafe under async

Construct: coordinator sends PRE_COMMIT to participant 1 only;
the message to participant 2 is delayed; participant 2 times
out and assumes the coordinator crashed; participant 2 starts
recovery and decides ABORT (no PRE_COMMIT seen from anyone);
participant 1 has PRE_COMMIT and decides COMMIT. Agreement
violated.

The pathology requires asynchrony to make participant 2's
timeout fire while the message was actually in flight. Under
synchrony, this cannot happen.

## Solution 4 [P]: timeout-based recovery

```rust
fn on_timer(&mut self, _: u64, ctx: &mut StepCtx<'_, Msg>) {
    if matches!(self.state, State::Prepared) {
        // Query other participants.
        for &p in &self.participants {
            if p != self.id {
                ctx.send(self.id, p, Msg::QueryDecision);
            }
        }
    }
}
```

This is essentially the 3PC structure: surviving participants
form a recovery quorum. The difference with 3PC is the
synchrony assumption (3PC's timeouts are part of the protocol
proof; the timeout-based 2PC recovery here is a heuristic).

## Solution 5 [P]: 3PC extension

3PC adds one phase. State machine:

```
Initial -> Voted -> PreCommitted -> Committed
                                 -> Aborted
```

Wire messages: CAN_COMMIT, VOTE, PRE_COMMIT, ACK_PRE,
COMMIT, ABORT. The recovery test: crash coordinator after
PRE_COMMIT, verify participants exchange states and decide
COMMIT.

## Solution 6 [F]: pseudo-Lean atomic-commit

```text
class AtomicCommit (n : Nat) where
  vote      : NodeId -> Bool
  decision  : NodeId -> Option Bool

  agreement :
    forall i j : Fin n,
    decision i = some d_i -> decision j = some d_j -> d_i = d_j

  validity :
    (forall i, vote i = true) -> (decision i = some true)
    (exists i, vote i = false) -> (decision i = some false)

  termination :
    forall (alive : Fin n -> Bool),
    (NoCrash alive) -> Eventually (forall i, alive i -> decision i ≠ none)

theorem two_pc_implements_ac
    (h_no_crash : NoCoordinatorCrash) :
    AtomicCommit n := ...
```

## Solution 7 [V]: verifiable 2PC

Per commit:

- Coordinator signs PREPARE: 1 signature, ~3k constraints.
- Each participant signs VOTE: `n` signatures, `~n * 3k`
  constraints. With BLS aggregation: 1 pairing, `~10^6`.
- Coordinator signs DECIDE: 1 signature, ~3k constraints.

Total per commit: `~10^6` constraints (dominated by the BLS
verification). Recursion across many commits gives constant
final-proof size.

Production use case: cross-shard transactions in Aptos and
Sui use a similar pattern (Quorum Store + per-shard certs).

## Solution 8 [V]: cross-chain atomic commit

Cross-chain atomic commit between chains A and B:

- Phase 1 (vote): each chain commits to a "prepare" state-root
  showing the transaction is locked.
- Phase 2 (decide): each chain observes the other's prepare
  via a zk-bridge proof; once both observed, both commit.

The "coordinator" is replaced by the bridge protocol. The
challenge: the prepare state must be irrevocable (no rollback)
on each chain before the bridge confirms. Production: IBC
packet-flow approximates this, with timeouts for safety.
