//! Module 0023: a minimal Multi-Paxos with static leadership
//! and amortised Phase 1.

#![warn(missing_docs)]
#![allow(clippy::cast_possible_truncation)]

use sim::{Envelope, NodeId, Process, StepCtx};

/// Wire message.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Leader -> acceptors: prepare for ballot `b` (Phase 1).
    Prepare {
        /// Ballot.
        b: u32,
    },
    /// Acceptor -> leader: promise (with the highest accepted
    /// per slot, for log catch-up; abbreviated here to a single
    /// per-acceptor accepted log).
    Promise {
        /// Ballot.
        b: u32,
        /// Acceptor's log up to now.
        log: Vec<Option<(u32, u32)>>,
    },
    /// Leader -> acceptors: accept op `op` at slot `i` (Phase 2).
    Accept {
        /// Ballot.
        b: u32,
        /// Slot index.
        i: u32,
        /// Operation.
        op: u32,
    },
    /// Acceptor -> leader: accepted at slot `i`.
    Accepted {
        /// Ballot.
        b: u32,
        /// Slot.
        i: u32,
    },
}

/// Multi-Paxos node combining proposer and acceptor.
pub struct MpNode {
    id: NodeId,
    everyone: Vec<NodeId>,
    /// Whether this node is the static leader.
    pub is_leader: bool,
    /// Pending operations (leader only).
    pub pending: Vec<u32>,
    /// Initial ballot (leader only).
    pub initial_ballot: u32,
    /// Acceptor's promised ballot.
    promised: Option<u32>,
    /// Per-slot accepted (ballot, op).
    pub log: Vec<Option<(u32, u32)>>,
    /// Leader's collected promises by ballot.
    promises: std::collections::BTreeMap<NodeId, Vec<Option<(u32, u32)>>>,
    /// Leader's per-slot accepteds by ballot.
    accepteds:
        std::collections::HashMap<u32, std::collections::BTreeSet<NodeId>>,
    /// Leader's current ballot.
    current_ballot: u32,
    /// Number of committed slots.
    pub commit_index: u32,
    /// Whether the leader has performed Phase 1.
    leader_phase1_done: bool,
    started: bool,
}

impl MpNode {
    /// Build a Multi-Paxos node.
    pub fn new(
        id: NodeId,
        everyone: Vec<NodeId>,
        is_leader: bool,
        pending: Vec<u32>,
        initial_ballot: u32,
    ) -> Self {
        Self {
            id,
            everyone,
            is_leader,
            pending,
            initial_ballot,
            promised: None,
            log: Vec::new(),
            promises: std::collections::BTreeMap::new(),
            accepteds: std::collections::HashMap::new(),
            current_ballot: 0,
            commit_index: 0,
            leader_phase1_done: false,
            started: false,
        }
    }

    fn quorum(&self) -> usize {
        self.everyone.len() / 2 + 1
    }
}

impl Process for MpNode {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if !self.is_leader || self.started {
            return;
        }
        self.started = true;
        self.current_ballot = self.initial_ballot;
        for &peer in &self.everyone {
            ctx.send(
                self.id,
                peer,
                Msg::Prepare {
                    b: self.current_ballot,
                },
            );
        }
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::Prepare { b } => {
                if self.promised.is_none_or(|p| b > p) {
                    self.promised = Some(b);
                    ctx.send(
                        self.id,
                        env.from,
                        Msg::Promise {
                            b,
                            log: self.log.clone(),
                        },
                    );
                }
            }
            Msg::Promise { b, log } => {
                if !self.is_leader || b != self.current_ballot {
                    return;
                }
                self.promises.insert(env.from, log);
                if self.promises.len() >= self.quorum()
                    && !self.leader_phase1_done
                {
                    self.leader_phase1_done = true;
                    // Drain pending ops as Phase-2 Accept
                    // messages, one per slot.
                    let pending = std::mem::take(&mut self.pending);
                    for op in pending {
                        let i = self.log.len() as u32;
                        self.log.push(Some((self.current_ballot, op)));
                        for &peer in &self.everyone {
                            ctx.send(
                                self.id,
                                peer,
                                Msg::Accept {
                                    b: self.current_ballot,
                                    i,
                                    op,
                                },
                            );
                        }
                    }
                }
            }
            Msg::Accept { b, i, op } => {
                if self.promised.is_none_or(|p| b >= p) {
                    self.promised = Some(b);
                    while self.log.len() <= i as usize {
                        self.log.push(None);
                    }
                    self.log[i as usize] = Some((b, op));
                    ctx.send(self.id, env.from, Msg::Accepted { b, i });
                }
            }
            Msg::Accepted { b, i } => {
                if !self.is_leader || b != self.current_ballot {
                    return;
                }
                let set = self.accepteds.entry(i).or_default();
                set.insert(env.from);
                if set.len() >= self.quorum() {
                    self.commit_index = self.commit_index.max(i + 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Scheduler};

    use super::*;

    #[test]
    fn three_node_log_replicates() {
        let everyone = vec![NodeId(0), NodeId(1), NodeId(2)];
        let mut sched = Scheduler::<MpNode>::new(0);
        sched
            .add_node(MpNode::new(
                NodeId(0),
                everyone.clone(),
                true,
                vec![10, 20, 30],
                1,
            ))
            .unwrap();
        for &id in &everyone[1..] {
            sched
                .add_node(MpNode::new(id, everyone.clone(), false, vec![], 0))
                .unwrap();
        }
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 1000).unwrap();
        let leader = sched.node(NodeId(0)).unwrap();
        assert_eq!(leader.commit_index, 3);
        let leader_ops: Vec<u32> = leader
            .log
            .iter()
            .filter_map(|x| x.map(|(_, op)| op))
            .collect();
        assert_eq!(leader_ops, vec![10, 20, 30]);
        for id in [NodeId(1), NodeId(2)] {
            let r = sched.node(id).unwrap();
            let ops: Vec<u32> =
                r.log.iter().filter_map(|x| x.map(|(_, op)| op)).collect();
            assert_eq!(ops, vec![10, 20, 30]);
        }
    }
}
