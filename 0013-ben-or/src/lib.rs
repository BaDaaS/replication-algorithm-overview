//! Module 0013: a minimal Ben-Or implementation focused on the
//! validity case (all-equal input).
//!
//! The full Ben-Or with mixed inputs and probabilistic
//! termination is best exercised with property tests; the
//! simulator's deterministic schedule reduces it to a single
//! sample.

#![warn(missing_docs)]

use rand::RngExt;
use sim::{Envelope, NodeId, Process, StepCtx};

/// Wire message type.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Phase-1 proposal in round `r`.
    Phase1 {
        /// Round number.
        r: u32,
        /// Process's preference.
        v: bool,
    },
    /// Phase-2 vote in round `r`. `Some(v)` if a Phase-1
    /// majority was observed; `None` otherwise.
    Phase2 {
        /// Round number.
        r: u32,
        /// Optional value.
        v: Option<bool>,
    },
}

/// Ben-Or process state.
pub struct BenOrNode {
    id: NodeId,
    everyone: Vec<NodeId>,
    /// Total processes.
    pub n: usize,
    /// Crash bound.
    pub f: usize,
    /// Current round.
    pub round: u32,
    /// Current preference.
    pub preference: bool,
    /// Final decision.
    pub decision: Option<bool>,
    phase1_received: std::collections::HashMap<u32, Vec<bool>>,
    phase2_received: std::collections::HashMap<u32, Vec<Option<bool>>>,
    sent_phase1: bool,
    sent_phase2: bool,
}

impl BenOrNode {
    /// Build a Ben-Or node.
    pub fn new(
        id: NodeId,
        everyone: Vec<NodeId>,
        n: usize,
        f: usize,
        input: bool,
    ) -> Self {
        Self {
            id,
            everyone,
            n,
            f,
            round: 1,
            preference: input,
            decision: None,
            phase1_received: std::collections::HashMap::new(),
            phase2_received: std::collections::HashMap::new(),
            sent_phase1: false,
            sent_phase2: false,
        }
    }

    fn broadcast(&self, msg: &Msg, ctx: &mut StepCtx<'_, Msg>) {
        for &peer in &self.everyone {
            ctx.send(self.id, peer, msg.clone());
        }
    }

    fn try_phase1(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if !self.sent_phase1 {
            self.sent_phase1 = true;
            self.broadcast(
                &Msg::Phase1 {
                    r: self.round,
                    v: self.preference,
                },
                ctx,
            );
        }
    }

    fn try_phase2(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        let votes = self
            .phase1_received
            .get(&self.round)
            .cloned()
            .unwrap_or_default();
        if votes.len() < self.n - self.f {
            return;
        }
        if self.sent_phase2 {
            return;
        }
        self.sent_phase2 = true;
        let count_true = votes.iter().filter(|&&b| b).count();
        let count_false = votes.len() - count_true;
        let bias = if count_true > self.n / 2 {
            Some(true)
        } else if count_false > self.n / 2 {
            Some(false)
        } else {
            None
        };
        self.broadcast(
            &Msg::Phase2 {
                r: self.round,
                v: bias,
            },
            ctx,
        );
    }

    fn try_decide(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        let votes = self
            .phase2_received
            .get(&self.round)
            .cloned()
            .unwrap_or_default();
        if votes.len() < self.n - self.f {
            return;
        }
        let count_true = votes.iter().filter(|&&v| v == Some(true)).count();
        let count_false = votes.iter().filter(|&&v| v == Some(false)).count();
        if count_true > self.f {
            self.decision = Some(true);
        } else if count_false > self.f {
            self.decision = Some(false);
        } else if count_true >= 1 {
            self.preference = true;
        } else if count_false >= 1 {
            self.preference = false;
        } else {
            // No Phase2 vote with a value: flip a coin.
            self.preference = ctx.rng.inner().random();
        }
        self.round += 1;
        self.sent_phase1 = false;
        self.sent_phase2 = false;
        if self.decision.is_none() {
            self.try_phase1(ctx);
        }
    }
}

impl Process for BenOrNode {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        self.try_phase1(ctx);
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::Phase1 { r, v } => {
                self.phase1_received.entry(r).or_default().push(v);
                if r == self.round {
                    self.try_phase2(ctx);
                }
            }
            Msg::Phase2 { r, v } => {
                self.phase2_received.entry(r).or_default().push(v);
                if r == self.round {
                    self.try_decide(ctx);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Scheduler};

    use super::*;

    fn build(inputs: &[bool]) -> Scheduler<BenOrNode> {
        let n = inputs.len();
        let f = (n - 1) / 2;
        let ids: Vec<NodeId> =
            (0..n).map(|i| NodeId(u32::try_from(i).unwrap())).collect();
        let mut sched = Scheduler::<BenOrNode>::new(0);
        for (i, &id) in ids.iter().enumerate() {
            sched
                .add_node(BenOrNode::new(id, ids.clone(), n, f, inputs[i]))
                .unwrap();
        }
        sched
    }

    #[test]
    fn validity_all_true() {
        let mut sched = build(&[true, true, true, true]);
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 1000).unwrap();
        for id in [NodeId(0), NodeId(1), NodeId(2), NodeId(3)] {
            assert_eq!(sched.node(id).unwrap().decision, Some(true));
        }
    }

    #[test]
    fn validity_all_false() {
        let mut sched = build(&[false, false, false, false]);
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 1000).unwrap();
        for id in [NodeId(0), NodeId(1), NodeId(2), NodeId(3)] {
            assert_eq!(sched.node(id).unwrap().decision, Some(false));
        }
    }
}
