//! Module 0033: a minimal Raft -- `AppendEntries` flow with a
//! static leader.
//!
//! Election and joint-consensus reconfiguration are out of
//! scope here; see the README for pointers.

#![warn(missing_docs)]
#![allow(clippy::cast_possible_truncation)]

use sim::{Envelope, NodeId, Process, StepCtx};

/// Wire message.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Leader -> follower: append log entries.
    AppendEntries {
        /// Term.
        term: u32,
        /// Index of log entry preceding new ones.
        prev_index: u32,
        /// Term of `prev_index` entry.
        prev_term: u32,
        /// New entries (term, op).
        entries: Vec<(u32, u32)>,
        /// Latest commit index known to leader.
        leader_commit: u32,
    },
    /// Follower -> leader: success/failure response.
    Response {
        /// Term.
        term: u32,
        /// Match-index after this `AppendEntries`.
        match_index: u32,
        /// Did the follower accept the entries?
        success: bool,
    },
}

/// Raft node (minimalised: static-leader `AppendEntries` flow).
pub struct RaftNode {
    id: NodeId,
    everyone: Vec<NodeId>,
    /// Whether this node is the static leader.
    pub is_leader: bool,
    /// Current term.
    pub current_term: u32,
    /// Log of (term, op).
    pub log: Vec<(u32, u32)>,
    /// Commit index.
    pub commit_index: u32,
    /// Pending operations (leader only).
    pub pending: Vec<u32>,
    /// Per-follower match indices (leader's view).
    match_index: std::collections::BTreeMap<NodeId, u32>,
    started: bool,
}

impl RaftNode {
    /// Build a Raft node.
    pub fn new(
        id: NodeId,
        everyone: Vec<NodeId>,
        is_leader: bool,
        pending: Vec<u32>,
    ) -> Self {
        Self {
            id,
            everyone,
            is_leader,
            current_term: 1,
            log: Vec::new(),
            commit_index: 0,
            pending,
            match_index: std::collections::BTreeMap::new(),
            started: false,
        }
    }

    fn quorum(&self) -> usize {
        self.everyone.len() / 2 + 1
    }
}

impl Process for RaftNode {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if !self.is_leader || self.started {
            return;
        }
        self.started = true;
        // Append all pending ops to the local log; broadcast.
        let pending = std::mem::take(&mut self.pending);
        for op in pending {
            self.log.push((self.current_term, op));
        }
        let entries = self.log.clone();
        for &peer in &self.everyone {
            if peer == self.id {
                continue;
            }
            ctx.send(
                self.id,
                peer,
                Msg::AppendEntries {
                    term: self.current_term,
                    prev_index: 0,
                    prev_term: 0,
                    entries: entries.clone(),
                    leader_commit: self.commit_index,
                },
            );
        }
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::AppendEntries {
                term,
                prev_index: _,
                prev_term: _,
                entries,
                leader_commit,
            } => {
                if term < self.current_term {
                    ctx.send(
                        self.id,
                        env.from,
                        Msg::Response {
                            term: self.current_term,
                            match_index: 0,
                            success: false,
                        },
                    );
                    return;
                }
                self.current_term = term;
                self.log.clone_from(&entries);
                let new_commit = leader_commit.min(self.log.len() as u32);
                self.commit_index = self.commit_index.max(new_commit);
                ctx.send(
                    self.id,
                    env.from,
                    Msg::Response {
                        term,
                        match_index: self.log.len() as u32,
                        success: true,
                    },
                );
            }
            Msg::Response {
                term,
                match_index,
                success,
            } => {
                if !self.is_leader {
                    return;
                }
                if term > self.current_term {
                    return;
                }
                if !success {
                    return;
                }
                self.match_index.insert(env.from, match_index);
                // Advance commit index if a majority reaches a
                // higher index.
                let mut indices: Vec<u32> =
                    self.match_index.values().copied().collect();
                indices.push(self.log.len() as u32); // leader counts
                indices.sort_unstable();
                let median_index = indices.len() - self.quorum();
                let new_commit = indices[median_index];
                self.commit_index = self.commit_index.max(new_commit);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Scheduler};

    use super::*;

    #[test]
    fn three_replicas_replicate_log() {
        let everyone = vec![NodeId(0), NodeId(1), NodeId(2)];
        let mut sched = Scheduler::<RaftNode>::new(0);
        sched
            .add_node(RaftNode::new(
                NodeId(0),
                everyone.clone(),
                true,
                vec![10, 20, 30],
            ))
            .unwrap();
        for &id in &everyone[1..] {
            sched
                .add_node(RaftNode::new(id, everyone.clone(), false, vec![]))
                .unwrap();
        }
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 1000).unwrap();
        let leader = sched.node(NodeId(0)).unwrap();
        assert_eq!(leader.log.len(), 3);
        assert_eq!(leader.commit_index, 3);
        for id in [NodeId(1), NodeId(2)] {
            let f = sched.node(id).unwrap();
            assert_eq!(f.log.len(), 3);
        }
    }
}
