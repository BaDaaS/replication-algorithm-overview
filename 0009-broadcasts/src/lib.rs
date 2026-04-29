//! Module 0009: Bracha's asynchronous reliable broadcast.
//!
//! Generic over a content type `M` that can be hashed for
//! equality. Each node tracks per-content echo and ready counts
//! and delivers once it has accumulated `2f + 1` Ready messages.

#![warn(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::hash::Hash;

use sim::{Envelope, NodeId, Process, StepCtx};

/// Wire message used by Bracha RB.
#[derive(Clone, Debug)]
pub enum Msg<M: Clone + Debug> {
    /// Initial broadcast from the sender.
    Init {
        /// The broadcaster's identifier.
        sender: NodeId,
        /// The content.
        content: M,
    },
    /// Echo confirming receipt of an `Init`.
    Echo {
        /// Original sender.
        sender: NodeId,
        /// Echoed content.
        content: M,
    },
    /// Ready to deliver this content.
    Ready {
        /// Original sender.
        sender: NodeId,
        /// Content to deliver.
        content: M,
    },
}

/// A Bracha reliable-broadcast node.
pub struct BrachaNode<M: Clone + Debug + Eq + Hash + Ord> {
    id: NodeId,
    everyone: Vec<NodeId>,
    /// `n - f`, the threshold for converting `Echo` to `Ready`.
    pub echo_threshold: usize,
    /// `f + 1`, the threshold for the `Ready` amplification rule.
    pub ready_amp_threshold: usize,
    /// `2f + 1`, the delivery threshold.
    pub deliver_threshold: usize,
    /// Initial value to broadcast (only for the broadcaster).
    pub initial: Option<M>,
    initial_done: bool,
    /// Per (sender, content) tally of received Echoes.
    echo_set: BTreeMap<(NodeId, M), BTreeSet<NodeId>>,
    /// Per (sender, content) tally of received Readys.
    ready_set: BTreeMap<(NodeId, M), BTreeSet<NodeId>>,
    /// Have we sent our own Echo for (sender, content)?
    echoed: BTreeSet<(NodeId, M)>,
    /// Have we sent our own Ready for (sender, content)?
    readied: BTreeSet<(NodeId, M)>,
    /// Delivered (sender, content) pairs.
    pub delivered: BTreeSet<(NodeId, M)>,
}

impl<M: Clone + Debug + Eq + Hash + Ord> BrachaNode<M> {
    /// Build a Bracha RB node.
    pub fn new(
        id: NodeId,
        everyone: Vec<NodeId>,
        n: usize,
        f: usize,
        initial: Option<M>,
    ) -> Self {
        Self {
            id,
            everyone,
            echo_threshold: n - f,
            ready_amp_threshold: f + 1,
            deliver_threshold: 2 * f + 1,
            initial,
            initial_done: false,
            echo_set: BTreeMap::new(),
            ready_set: BTreeMap::new(),
            echoed: BTreeSet::new(),
            readied: BTreeSet::new(),
            delivered: BTreeSet::new(),
        }
    }

    fn broadcast<F: FnMut(&NodeId)>(&self, mut f: F) {
        for peer in &self.everyone {
            f(peer);
        }
    }

    fn handle_init(
        &mut self,
        sender: NodeId,
        content: &M,
        ctx: &mut StepCtx<'_, Msg<M>>,
    ) {
        let key = (sender, content.clone());
        if self.echoed.insert(key.clone()) {
            self.broadcast(|peer| {
                ctx.send(
                    self.id,
                    *peer,
                    Msg::Echo {
                        sender,
                        content: content.clone(),
                    },
                );
            });
        }
    }

    fn handle_echo(
        &mut self,
        from: NodeId,
        sender: NodeId,
        content: &M,
        ctx: &mut StepCtx<'_, Msg<M>>,
    ) {
        let key = (sender, content.clone());
        let count = {
            let set = self.echo_set.entry(key.clone()).or_default();
            set.insert(from);
            set.len()
        };
        if count >= self.echo_threshold && self.readied.insert(key.clone()) {
            self.broadcast(|peer| {
                ctx.send(
                    self.id,
                    *peer,
                    Msg::Ready {
                        sender,
                        content: content.clone(),
                    },
                );
            });
        }
    }

    fn handle_ready(
        &mut self,
        from: NodeId,
        sender: NodeId,
        content: &M,
        ctx: &mut StepCtx<'_, Msg<M>>,
    ) {
        let key = (sender, content.clone());
        let count = {
            let set = self.ready_set.entry(key.clone()).or_default();
            set.insert(from);
            set.len()
        };
        // Amplification rule: f + 1 readys imply we ready too,
        // even without seeing n - f echoes.
        if count >= self.ready_amp_threshold && self.readied.insert(key.clone())
        {
            self.broadcast(|peer| {
                ctx.send(
                    self.id,
                    *peer,
                    Msg::Ready {
                        sender,
                        content: content.clone(),
                    },
                );
            });
        }
        if count >= self.deliver_threshold {
            self.delivered.insert(key);
        }
    }
}

impl<M: Clone + Debug + Eq + Hash + Ord + 'static> Process for BrachaNode<M> {
    type Message = Msg<M>;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg<M>>) {
        if self.initial_done {
            return;
        }
        self.initial_done = true;
        if let Some(initial) = self.initial.clone() {
            // Broadcast Init to all peers including self.
            for peer in self.everyone.clone() {
                ctx.send(
                    self.id,
                    peer,
                    Msg::Init {
                        sender: self.id,
                        content: initial.clone(),
                    },
                );
            }
        }
    }

    fn on_receive(
        &mut self,
        env: Envelope<Msg<M>>,
        ctx: &mut StepCtx<'_, Msg<M>>,
    ) {
        match env.msg {
            Msg::Init { sender, content } => {
                self.handle_init(sender, &content, ctx);
            }
            Msg::Echo { sender, content } => {
                self.handle_echo(env.from, sender, &content, ctx);
            }
            Msg::Ready { sender, content } => {
                self.handle_ready(env.from, sender, &content, ctx);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Scheduler};

    use super::*;

    fn build(
        n: u32,
        f: usize,
        initial: Option<u32>,
    ) -> Scheduler<BrachaNode<u32>> {
        let ids: Vec<NodeId> = (0..n).map(NodeId).collect();
        let mut sched = Scheduler::<BrachaNode<u32>>::new(0);
        for (i, &id) in ids.iter().enumerate() {
            let init = if i == 0 { initial } else { None };
            sched
                .add_node(BrachaNode::new(id, ids.clone(), n as usize, f, init))
                .unwrap();
        }
        sched
    }

    #[test]
    fn bracha_delivers_under_no_op() {
        // n = 4, f = 1. NodeId(0) broadcasts content 7. All
        // honest nodes deliver.
        let mut sched = build(4, 1, Some(7));
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 1000).unwrap();
        for id in [NodeId(0), NodeId(1), NodeId(2), NodeId(3)] {
            let n = sched.node(id).unwrap();
            assert!(n.delivered.contains(&(NodeId(0), 7)));
        }
    }
}
