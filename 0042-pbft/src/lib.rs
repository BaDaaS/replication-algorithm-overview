//! Module 0042: minimal PBFT three-phase simulator.

#![warn(missing_docs)]
#![allow(clippy::cast_possible_truncation)]

use std::collections::{BTreeMap, BTreeSet};

use sim::{Envelope, NodeId, Process, StepCtx};

/// Wire message.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Phase 1: primary -> all replicas.
    PrePrepare {
        /// View.
        v: u32,
        /// Sequence number.
        n: u32,
        /// Operation digest.
        op: u32,
    },
    /// Phase 2: replica -> all on accept.
    Prepare {
        /// View.
        v: u32,
        /// Sequence number.
        n: u32,
        /// Operation digest.
        op: u32,
    },
    /// Phase 3: replica -> all on prepare-cert.
    Commit {
        /// View.
        v: u32,
        /// Sequence number.
        n: u32,
        /// Operation digest.
        op: u32,
    },
}

/// PBFT replica.
pub struct PbftNode {
    id: NodeId,
    everyone: Vec<NodeId>,
    /// Total replicas.
    pub n: usize,
    /// Byzantine bound.
    pub f: usize,
    /// Whether this node is the primary.
    pub is_primary: bool,
    /// Current view.
    pub view: u32,
    /// Pending operations (primary only).
    pub pending: Vec<u32>,
    next_seq: u32,
    pre_prepares: BTreeMap<(u32, u32), u32>, // (v,n) -> op
    prepares: BTreeMap<(u32, u32, u32), BTreeSet<NodeId>>,
    commits: BTreeMap<(u32, u32, u32), BTreeSet<NodeId>>,
    /// Committed log of (v, n, op).
    pub log: Vec<(u32, u32, u32)>,
    sent_pre_prepare: bool,
}

impl PbftNode {
    /// Build a PBFT node.
    pub fn new(
        id: NodeId,
        everyone: Vec<NodeId>,
        n: usize,
        f: usize,
        is_primary: bool,
        pending: Vec<u32>,
    ) -> Self {
        Self {
            id,
            everyone,
            n,
            f,
            is_primary,
            view: 0,
            pending,
            next_seq: 0,
            pre_prepares: BTreeMap::new(),
            prepares: BTreeMap::new(),
            commits: BTreeMap::new(),
            log: Vec::new(),
            sent_pre_prepare: false,
        }
    }

    fn quorum(&self) -> usize {
        2 * self.f + 1
    }

    fn broadcast(&self, msg: &Msg, ctx: &mut StepCtx<'_, Msg>) {
        for &peer in &self.everyone {
            ctx.send(self.id, peer, msg.clone());
        }
    }
}

impl Process for PbftNode {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if !self.is_primary || self.sent_pre_prepare {
            return;
        }
        self.sent_pre_prepare = true;
        let pending = std::mem::take(&mut self.pending);
        for op in pending {
            self.next_seq += 1;
            let n = self.next_seq;
            self.broadcast(
                &Msg::PrePrepare {
                    v: self.view,
                    n,
                    op,
                },
                ctx,
            );
        }
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::PrePrepare { v, n, op } => {
                if v != self.view {
                    return;
                }
                self.pre_prepares.insert((v, n), op);
                self.broadcast(&Msg::Prepare { v, n, op }, ctx);
            }
            Msg::Prepare { v, n, op } => {
                if v != self.view {
                    return;
                }
                let set = self.prepares.entry((v, n, op)).or_default();
                set.insert(env.from);
                if set.len() >= self.quorum() {
                    self.broadcast(&Msg::Commit { v, n, op }, ctx);
                }
            }
            Msg::Commit { v, n, op } => {
                if v != self.view {
                    return;
                }
                let set = self.commits.entry((v, n, op)).or_default();
                set.insert(env.from);
                if set.len() >= self.quorum() && !self.log.contains(&(v, n, op))
                {
                    self.log.push((v, n, op));
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
    fn four_replicas_pbft_commit() {
        // n = 4, f = 1.
        let everyone = vec![NodeId(0), NodeId(1), NodeId(2), NodeId(3)];
        let mut sched = Scheduler::<PbftNode>::new(0);
        sched
            .add_node(PbftNode::new(
                NodeId(0),
                everyone.clone(),
                4,
                1,
                true,
                vec![10, 20, 30],
            ))
            .unwrap();
        for &id in &everyone[1..] {
            sched
                .add_node(PbftNode::new(
                    id,
                    everyone.clone(),
                    4,
                    1,
                    false,
                    vec![],
                ))
                .unwrap();
        }
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 5000).unwrap();
        for id in &everyone {
            let r = sched.node(*id).unwrap();
            assert_eq!(r.log.len(), 3, "node {id} log {:?}", r.log);
        }
    }
}
