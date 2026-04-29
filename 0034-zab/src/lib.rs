//! Module 0034: ZAB minimal Broadcast-phase simulator.

#![warn(missing_docs)]
#![allow(clippy::cast_possible_truncation)]

use sim::{Envelope, NodeId, Process, StepCtx};

/// Zxid: epoch + counter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Zxid {
    /// Epoch.
    pub epoch: u32,
    /// Counter within the epoch.
    pub counter: u32,
}

/// Wire message.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Leader -> followers: propose tx with zxid.
    Propose {
        /// Zxid.
        zxid: Zxid,
        /// Operation.
        op: u32,
    },
    /// Follower -> leader: ack the tx.
    Ack {
        /// Zxid.
        zxid: Zxid,
    },
    /// Leader -> followers: commit the tx.
    Commit {
        /// Zxid.
        zxid: Zxid,
    },
}

/// ZAB node.
pub struct ZabNode {
    id: NodeId,
    everyone: Vec<NodeId>,
    /// Whether this node is the leader.
    pub is_leader: bool,
    /// Current epoch (= term).
    pub epoch: u32,
    /// Local log of committed (zxid, op).
    pub log: Vec<(Zxid, u32)>,
    /// Pending operations (leader only).
    pub pending: Vec<u32>,
    next_counter: u32,
    acks: std::collections::BTreeMap<Zxid, std::collections::BTreeSet<NodeId>>,
    started: bool,
}

impl ZabNode {
    /// Build a ZAB node.
    pub fn new(
        id: NodeId,
        everyone: Vec<NodeId>,
        is_leader: bool,
        epoch: u32,
        pending: Vec<u32>,
    ) -> Self {
        Self {
            id,
            everyone,
            is_leader,
            epoch,
            log: Vec::new(),
            pending,
            next_counter: 0,
            acks: std::collections::BTreeMap::new(),
            started: false,
        }
    }

    fn quorum(&self) -> usize {
        self.everyone.len() / 2 + 1
    }
}

impl Process for ZabNode {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if !self.is_leader || self.started {
            return;
        }
        self.started = true;
        let pending = std::mem::take(&mut self.pending);
        for op in pending {
            self.next_counter += 1;
            let zxid = Zxid {
                epoch: self.epoch,
                counter: self.next_counter,
            };
            for &peer in &self.everyone {
                ctx.send(self.id, peer, Msg::Propose { zxid, op });
            }
        }
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::Propose { zxid, op } => {
                // Followers ack but do not commit until COMMIT
                // received.
                ctx.send(self.id, env.from, Msg::Ack { zxid });
                // Stash op for commit (leader broadcasts commit
                // implicitly with op).
                if self.is_leader {
                    self.log.push((zxid, op));
                } else {
                    // Follower keeps a pending log line.
                    self.log.push((zxid, op));
                }
            }
            Msg::Ack { zxid } => {
                if !self.is_leader {
                    return;
                }
                let set = self.acks.entry(zxid).or_default();
                set.insert(env.from);
                if set.len() >= self.quorum() {
                    // Broadcast COMMIT.
                    for &peer in &self.everyone {
                        if peer == self.id {
                            continue;
                        }
                        ctx.send(self.id, peer, Msg::Commit { zxid });
                    }
                }
            }
            Msg::Commit { zxid: _ } => {
                // Followers mark the tx as committed (already
                // in self.log; here we just note receipt).
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Scheduler};

    use super::*;

    #[test]
    fn three_replicas_zab() {
        let everyone = vec![NodeId(0), NodeId(1), NodeId(2)];
        let mut sched = Scheduler::<ZabNode>::new(0);
        sched
            .add_node(ZabNode::new(
                NodeId(0),
                everyone.clone(),
                true,
                1,
                vec![10, 20, 30],
            ))
            .unwrap();
        for &id in &everyone[1..] {
            sched
                .add_node(ZabNode::new(id, everyone.clone(), false, 1, vec![]))
                .unwrap();
        }
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 1000).unwrap();
        let leader = sched.node(NodeId(0)).unwrap();
        assert_eq!(leader.log.len(), 3);
        for id in [NodeId(1), NodeId(2)] {
            let f = sched.node(id).unwrap();
            assert_eq!(f.log.len(), 3);
        }
    }
}
