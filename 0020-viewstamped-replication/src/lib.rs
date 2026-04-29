//! Module 0020: a minimal Viewstamped Replication normal-case
//! operation. View change is sketched in comments.

#![warn(missing_docs)]
#![allow(clippy::cast_possible_truncation)]

use sim::{Envelope, NodeId, Process, StepCtx};

/// Wire message.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Primary -> backup: prepare a new operation.
    Prepare {
        /// View number.
        view: u32,
        /// Operation sequence number.
        op_num: u32,
        /// The operation payload.
        op: u32,
        /// Latest committed `op_num`.
        commit_num: u32,
    },
    /// Backup -> primary: accepted PREPARE.
    PrepareOk {
        /// View number.
        view: u32,
        /// Op sequence number.
        op_num: u32,
        /// Backup's id.
        backup: NodeId,
    },
}

/// VR replica state.
pub struct VrNode {
    id: NodeId,
    everyone: Vec<NodeId>,
    primary_idx: usize,
    /// Current view number.
    pub view_num: u32,
    /// Sequence number of next op (primary only).
    next_op: u32,
    /// Backups' acks per `op_num`.
    acks: std::collections::HashMap<u32, std::collections::BTreeSet<NodeId>>,
    /// Replicated log.
    pub log: Vec<u32>,
    /// Number of committed ops (the latest `op_num` known
    /// committed).
    pub commit_num: u32,
    /// Pending operations (only set on the primary).
    pub pending: Vec<u32>,
    started: bool,
}

impl VrNode {
    /// Build a VR node.
    pub fn new(
        id: NodeId,
        everyone: Vec<NodeId>,
        view_num: u32,
        pending: Vec<u32>,
    ) -> Self {
        let primary_idx = view_num as usize % everyone.len();
        Self {
            id,
            everyone,
            primary_idx,
            view_num,
            next_op: 0,
            acks: std::collections::HashMap::new(),
            log: Vec::new(),
            commit_num: 0,
            pending,
            started: false,
        }
    }

    fn is_primary(&self) -> bool {
        self.everyone[self.primary_idx] == self.id
    }

    fn quorum(&self) -> usize {
        // f + 1 (including the primary).
        self.everyone.len() / 2 + 1
    }
}

impl Process for VrNode {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if !self.is_primary() || self.started {
            return;
        }
        self.started = true;
        // Send PREPARE for each pending op in sequence.
        let pending = std::mem::take(&mut self.pending);
        for op in pending {
            self.next_op += 1;
            let op_num = self.next_op;
            self.log.push(op);
            for &peer in &self.everyone {
                if peer == self.id {
                    continue;
                }
                ctx.send(
                    self.id,
                    peer,
                    Msg::Prepare {
                        view: self.view_num,
                        op_num,
                        op,
                        commit_num: self.commit_num,
                    },
                );
            }
        }
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::Prepare {
                view,
                op_num,
                op,
                commit_num,
            } => {
                if view != self.view_num {
                    return; // wrong view; in real VR, would
                    // trigger view-change check
                }
                while self.log.len() < op_num as usize {
                    self.log.push(0); // gap; should never happen
                    // under FIFO + non-faulty
                }
                if (self.log.len() as u32) < op_num {
                    self.log.push(op);
                } else {
                    self.log[(op_num - 1) as usize] = op;
                }
                self.commit_num = self.commit_num.max(commit_num);
                ctx.send(
                    self.id,
                    self.everyone[self.primary_idx],
                    Msg::PrepareOk {
                        view: self.view_num,
                        op_num,
                        backup: self.id,
                    },
                );
            }
            Msg::PrepareOk {
                view,
                op_num,
                backup,
            } => {
                if view != self.view_num || !self.is_primary() {
                    return;
                }
                let acks = self.acks.entry(op_num).or_default();
                acks.insert(backup);
                // Primary itself counts toward quorum.
                if acks.len() + 1 >= self.quorum() {
                    self.commit_num = self.commit_num.max(op_num);
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
    fn primary_commits_via_quorum() {
        let everyone = vec![NodeId(0), NodeId(1), NodeId(2)];
        let mut sched = Scheduler::<VrNode>::new(0);
        sched
            .add_node(VrNode::new(
                NodeId(0),
                everyone.clone(),
                0,
                vec![10, 20, 30],
            ))
            .unwrap();
        for &id in &everyone[1..] {
            sched
                .add_node(VrNode::new(id, everyone.clone(), 0, vec![]))
                .unwrap();
        }
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 1000).unwrap();
        let primary = sched.node(NodeId(0)).unwrap();
        assert_eq!(primary.commit_num, 3);
        assert_eq!(primary.log, vec![10, 20, 30]);
        for id in [NodeId(1), NodeId(2)] {
            let backup = sched.node(id).unwrap();
            assert_eq!(backup.log, vec![10, 20, 30]);
        }
    }
}
