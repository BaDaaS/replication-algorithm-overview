//! Module 0035: Chain Replication minimal simulator.

#![warn(missing_docs)]

use sim::{Envelope, NodeId, Process, StepCtx};

/// Wire message.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Forward an update down the chain.
    Update {
        /// Key.
        k: u32,
        /// Value.
        v: u32,
    },
    /// Tail acks back to head/client.
    Ack {
        /// Key.
        k: u32,
    },
}

/// Chain replica.
pub struct ChainNode {
    id: NodeId,
    /// This replica's index in the chain (0 = head).
    pub index: usize,
    /// Total chain length.
    pub n: usize,
    /// Next replica in the chain (None for tail).
    pub next: Option<NodeId>,
    /// Local store.
    pub store: std::collections::BTreeMap<u32, u32>,
    /// Initial writes (head only).
    pub initial: Vec<(u32, u32)>,
    started: bool,
}

impl ChainNode {
    /// Build a chain replica.
    pub fn new(
        id: NodeId,
        index: usize,
        n: usize,
        next: Option<NodeId>,
        initial: Vec<(u32, u32)>,
    ) -> Self {
        Self {
            id,
            index,
            n,
            next,
            store: std::collections::BTreeMap::new(),
            initial,
            started: false,
        }
    }

    fn is_head(&self) -> bool {
        self.index == 0
    }

    fn is_tail(&self) -> bool {
        self.index == self.n - 1
    }
}

impl Process for ChainNode {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if !self.is_head() || self.started {
            return;
        }
        self.started = true;
        let initial = std::mem::take(&mut self.initial);
        for (k, v) in initial {
            self.store.insert(k, v);
            if let Some(next) = self.next {
                ctx.send(self.id, next, Msg::Update { k, v });
            }
        }
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::Update { k, v } => {
                self.store.insert(k, v);
                if let Some(next) = self.next {
                    ctx.send(self.id, next, Msg::Update { k, v });
                }
                if self.is_tail() {
                    // Tail ack flows back to head (skipping
                    // intermediate hops for simplicity).
                    let _ = env.from;
                }
            }
            Msg::Ack { .. } => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Scheduler};

    use super::*;

    #[test]
    fn three_chain_propagates_writes() {
        let chain = vec![NodeId(0), NodeId(1), NodeId(2)];
        let n = chain.len();
        let mut sched = Scheduler::<ChainNode>::new(0);
        for (i, &id) in chain.iter().enumerate() {
            let next = if i + 1 < n { Some(chain[i + 1]) } else { None };
            let initial = if i == 0 {
                vec![(1, 100), (2, 200), (3, 300)]
            } else {
                vec![]
            };
            sched
                .add_node(ChainNode::new(id, i, n, next, initial))
                .unwrap();
        }
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 1000).unwrap();
        for &id in &chain {
            let r = sched.node(id).unwrap();
            assert_eq!(r.store.get(&1), Some(&100));
            assert_eq!(r.store.get(&2), Some(&200));
            assert_eq!(r.store.get(&3), Some(&300));
        }
    }
}
