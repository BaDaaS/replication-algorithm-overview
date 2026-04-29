//! Module 0014: a minimal common-coin Rabin-style ABA.
//!
//! The common coin is supplied externally via a closure; in
//! tests we use a deterministic per-round bit for reproducible
//! checks of the validity case.

#![warn(missing_docs)]

use sim::{Envelope, NodeId, Process, StepCtx};

/// Wire message.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Phase 1 proposal.
    Propose {
        /// Round.
        r: u32,
        /// Process's preference.
        v: bool,
    },
    /// Phase 2 echo.
    Echo {
        /// Round.
        r: u32,
        /// Echoed value (None = abstain).
        v: Option<bool>,
    },
}

/// Common-coin oracle: returns the per-round coin bit.
pub type CoinFn = fn(round: u32) -> bool;

/// Rabin-style ABA process state.
pub struct RabinNode {
    id: NodeId,
    everyone: Vec<NodeId>,
    /// Total processes.
    pub n: usize,
    /// Byzantine bound.
    pub f: usize,
    /// Current round.
    pub round: u32,
    /// Current preference.
    pub preference: bool,
    /// Final decision.
    pub decision: Option<bool>,
    /// Round oracle for the common coin.
    pub coin: CoinFn,
    proposes: std::collections::HashMap<u32, Vec<bool>>,
    echoes: std::collections::HashMap<u32, Vec<Option<bool>>>,
    sent_propose: u32,
    sent_echo: u32,
}

impl RabinNode {
    /// Build a Rabin-style node.
    pub fn new(
        id: NodeId,
        everyone: Vec<NodeId>,
        n: usize,
        f: usize,
        input: bool,
        coin: CoinFn,
    ) -> Self {
        Self {
            id,
            everyone,
            n,
            f,
            round: 1,
            preference: input,
            decision: None,
            coin,
            proposes: std::collections::HashMap::new(),
            echoes: std::collections::HashMap::new(),
            sent_propose: 0,
            sent_echo: 0,
        }
    }

    fn broadcast(&self, msg: &Msg, ctx: &mut StepCtx<'_, Msg>) {
        for &peer in &self.everyone {
            ctx.send(self.id, peer, msg.clone());
        }
    }

    fn maybe_propose(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if self.sent_propose < self.round {
            self.sent_propose = self.round;
            self.broadcast(
                &Msg::Propose {
                    r: self.round,
                    v: self.preference,
                },
                ctx,
            );
        }
    }

    fn maybe_echo(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        let votes = self.proposes.get(&self.round).cloned().unwrap_or_default();
        if votes.len() < self.n - self.f {
            return;
        }
        if self.sent_echo >= self.round {
            return;
        }
        self.sent_echo = self.round;
        let count_true = votes.iter().filter(|&&b| b).count();
        let count_false = votes.len() - count_true;
        let v = if count_true > 2 * self.n / 3 {
            Some(true)
        } else if count_false > 2 * self.n / 3 {
            Some(false)
        } else {
            None
        };
        self.broadcast(&Msg::Echo { r: self.round, v }, ctx);
    }

    fn maybe_decide(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        let votes = self.echoes.get(&self.round).cloned().unwrap_or_default();
        if votes.len() < self.n - self.f {
            return;
        }
        let count_true = votes.iter().filter(|&&v| v == Some(true)).count();
        let count_false = votes.iter().filter(|&&v| v == Some(false)).count();
        if count_true > 2 * self.n / 3 {
            self.decision = Some(true);
            return;
        }
        if count_false > 2 * self.n / 3 {
            self.decision = Some(false);
            return;
        }
        // Update preference using majority hint or coin.
        self.preference = if count_true >= 1 && count_false == 0 {
            true
        } else if count_false >= 1 && count_true == 0 {
            false
        } else {
            (self.coin)(self.round)
        };
        self.round += 1;
        self.maybe_propose(ctx);
    }
}

impl Process for RabinNode {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        self.maybe_propose(ctx);
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::Propose { r, v } => {
                self.proposes.entry(r).or_default().push(v);
                if r == self.round {
                    self.maybe_echo(ctx);
                }
            }
            Msg::Echo { r, v } => {
                self.echoes.entry(r).or_default().push(v);
                if r == self.round {
                    self.maybe_decide(ctx);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Scheduler};

    use super::*;

    fn deterministic_coin(_r: u32) -> bool {
        true
    }

    fn build(inputs: &[bool]) -> Scheduler<RabinNode> {
        let n = inputs.len();
        let f = (n - 1) / 3;
        let ids: Vec<NodeId> =
            (0..n).map(|i| NodeId(u32::try_from(i).unwrap())).collect();
        let mut sched = Scheduler::<RabinNode>::new(0);
        for (i, &id) in ids.iter().enumerate() {
            sched
                .add_node(RabinNode::new(
                    id,
                    ids.clone(),
                    n,
                    f,
                    inputs[i],
                    deterministic_coin,
                ))
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
