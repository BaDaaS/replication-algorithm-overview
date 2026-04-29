//! Module 0028: a minimal Mencius simulator.
//!
//! Each of `n` replicas owns slots `i mod n`. The replica
//! proposes operations from its pending queue for its own
//! slots; for others' slots it follows the owner's proposal.

#![warn(missing_docs)]
#![allow(clippy::cast_possible_truncation)]

use sim::{Envelope, NodeId, Process, StepCtx};

/// Wire message.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Owner proposes op (or NO-OP) at slot.
    Propose {
        /// Slot.
        slot: u32,
        /// Operation; 0 means NO-OP.
        op: u32,
    },
    /// Acceptor accepts.
    Ack {
        /// Slot.
        slot: u32,
    },
}

/// Mencius node.
pub struct MenciusNode {
    id: NodeId,
    everyone: Vec<NodeId>,
    /// This node's index in the rotation (0..n).
    pub index: u32,
    /// Pending operations to be proposed in this node's slots.
    pub pending: Vec<u32>,
    /// Per-slot committed op.
    pub log: std::collections::BTreeMap<u32, u32>,
    /// Acks per slot (owner only).
    acks: std::collections::HashMap<u32, std::collections::BTreeSet<NodeId>>,
    next_slot_to_propose: u32,
    started: bool,
}

impl MenciusNode {
    /// Build a Mencius node.
    pub fn new(
        id: NodeId,
        everyone: Vec<NodeId>,
        index: u32,
        pending: Vec<u32>,
    ) -> Self {
        Self {
            id,
            everyone,
            index,
            pending,
            log: std::collections::BTreeMap::new(),
            acks: std::collections::HashMap::new(),
            next_slot_to_propose: 0,
            started: false,
        }
    }

    fn n(&self) -> u32 {
        self.everyone.len() as u32
    }

    fn is_owner(&self, slot: u32) -> bool {
        slot % self.n() == self.index
    }

    fn quorum(&self) -> usize {
        self.everyone.len() / 2 + 1
    }
}

impl Process for MenciusNode {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if self.started {
            return;
        }
        self.started = true;
        // Propose all my pending ops in my owned slots.
        let pending = std::mem::take(&mut self.pending);
        for op in pending {
            // Find next owned slot.
            let mut s = self.next_slot_to_propose;
            while !self.is_owner(s) {
                s += 1;
            }
            self.next_slot_to_propose = s + 1;
            self.log.insert(s, op);
            for &peer in &self.everyone {
                ctx.send(self.id, peer, Msg::Propose { slot: s, op });
            }
        }
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::Propose { slot, op } => {
                self.log.insert(slot, op);
                ctx.send(self.id, env.from, Msg::Ack { slot });
            }
            Msg::Ack { slot } => {
                if !self.is_owner(slot) {
                    return;
                }
                let set = self.acks.entry(slot).or_default();
                set.insert(env.from);
                let _ = self.quorum(); // commit threshold reached
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Scheduler};

    use super::*;

    #[test]
    fn three_replicas_each_propose() {
        let everyone = vec![NodeId(0), NodeId(1), NodeId(2)];
        let mut sched = Scheduler::<MenciusNode>::new(0);
        sched
            .add_node(MenciusNode::new(
                NodeId(0),
                everyone.clone(),
                0,
                vec![10, 20],
            ))
            .unwrap();
        sched
            .add_node(MenciusNode::new(
                NodeId(1),
                everyone.clone(),
                1,
                vec![100, 200],
            ))
            .unwrap();
        sched
            .add_node(MenciusNode::new(
                NodeId(2),
                everyone.clone(),
                2,
                vec![1000, 2000],
            ))
            .unwrap();
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 1000).unwrap();
        // Each replica proposes in its own slots; all replicas
        // see all proposals.
        for id in [NodeId(0), NodeId(1), NodeId(2)] {
            let n = sched.node(id).unwrap();
            assert!(n.log.values().any(|&v| v == 10));
            assert!(n.log.values().any(|&v| v == 100));
            assert!(n.log.values().any(|&v| v == 1000));
        }
    }
}
