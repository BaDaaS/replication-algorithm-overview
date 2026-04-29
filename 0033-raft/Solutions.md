# Module 0033 Solutions

## Solution 1 [T]: Election Safety

Suppose two distinct leaders `L_1, L_2` exist in term `t`.
Each was elected with majority votes. By majority
intersection, some node `v` voted for both. But Raft's
voting rule: each node votes at most once per term (tracked
by `voted_for` in stable storage). Contradiction.

The persistence of `voted_for` across crashes is essential:
without it, a recovering node could accidentally vote twice
in the same term.

## Solution 2 [T]: Log Matching

Logs grow append-only at the leader. Each entry's `(index,
term)` is fixed at creation. AppendEntries includes
`prev_index, prev_term`; if the follower's log doesn't match,
it rejects, and the leader retries with an earlier index.

So if two logs have an entry at `(i, t)`:
- Both come from leader of term `t`.
- The leader of term `t` had a unique log at index `i`.
- All replicas that accepted entry `(i, t)` did so via
  AppendEntries with consistent `prev_index/prev_term`,
  recursively to index `0`.
- So all logs are identical from `0` to `i`.

QED (induction on `i`).

## Solution 3 [P]: leader election sketch

```rust
fn on_election_timeout(&mut self, ctx: ...) {
    self.state = ServerState::Candidate;
    self.current_term += 1;
    self.voted_for = Some(self.id);
    let votes_needed = self.quorum();
    self.votes_received = 1;
    for peer in self.everyone {
        ctx.send(self.id, peer, RequestVote {
            term: self.current_term,
            last_log_index: self.log.len() as u32,
            last_log_term: self.log.last().map(|(t, _)| *t).unwrap_or(0),
        });
    }
}

fn on_request_vote(&mut self, env, ctx) {
    if env.term < self.current_term { reject }
    else if self.voted_for.is_none() {
        self.voted_for = Some(env.from);
        ctx.send(self.id, env.from, Vote { term: env.term, granted: true });
    }
}
```

Test: kill leader at time T; followers time out at random,
race for votes. After ~election_timeout * 2, one becomes
leader.

## Solution 4 [P]: joint consensus

```rust
fn reconfigure(&mut self, new_set: Vec<NodeId>) {
    // Phase 1: commit "joint" config
    self.append_log(Op::ConfigChange(JointConfig {
        old: self.config.clone(),
        new: new_set.clone(),
    }));
    // wait for commit using OLD majority AND NEW majority
    // ...

    // Phase 2: commit "new alone"
    self.append_log(Op::ConfigChange(SingleConfig(new_set)));
}
```

Safety: during Phase 1, decisions need majorities in *both*
old and new sets, so any decision propagates to both. After
Phase 2 commits, only new applies.

## Solution 5 [F]: pseudo-Lean

Verdi's Coq formalisation models RaftState with:

- `current_term, voted_for, log, commit_index, last_applied`,
- per-server state machine.

The 5 invariants are stated as Coq theorems and proved
mutually. Verdi's release contains ~50000 lines of Coq.

A Lean port would reuse cslib's LTS framework with Raft
states; Mathlib's `Finset` for majorities; `List` for the
log. Full port is open work.

## Solution 6 [V]: verifiable Raft

Per commit:

- BLS-aggregated AppendEntries success cert from `f + 1`
  followers: ~10^6 constraints.
- Log-Merkle proof: the new entry plus a Merkle path to the
  previous commit root: `~log(n) * 200 = ~3000` constraints.
- Term consistency: ~constraints.

Total per commit: ~10^6 (BLS-dominated). With chain
recursion, constant final-proof size.

Production: zk-rollups using Raft-style ordering (e.g.
private/permissioned chains) follow this template.
