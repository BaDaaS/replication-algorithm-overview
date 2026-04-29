//! Module 0015: a minimal Bracha-style ABA.
//!
//! We abstract Bracha RB with the simulator's reliable broadcast
//! primitive (every-to-every send under `NoOpAdversary`). The
//! common coin is supplied externally.

#![warn(missing_docs)]

use sim::{Envelope, NodeId, Process, StepCtx};

/// Wire message.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Vote for round `r`.
    Vote {
        /// Round number.
        r: u32,
        /// Process's preference.
        v: bool,
    },
    /// Auxiliary set message.
    Aux {
        /// Round number.
        r: u32,
        /// Reported majority value (None = no majority).
        v: Option<bool>,
    },
}

/// Common-coin oracle.
pub type CoinFn = fn(round: u32) -> bool;

/// Bracha-ABA process.
pub struct BrachaAbaNode {
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
    /// Coin oracle.
    pub coin: CoinFn,
    votes: std::collections::HashMap<u32, Vec<bool>>,
    auxes: std::collections::HashMap<u32, Vec<Option<bool>>>,
    sent_vote: u32,
    sent_aux: u32,
}

impl BrachaAbaNode {
    /// Build a node.
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
            votes: std::collections::HashMap::new(),
            auxes: std::collections::HashMap::new(),
            sent_vote: 0,
            sent_aux: 0,
        }
    }

    fn broadcast(&self, msg: &Msg, ctx: &mut StepCtx<'_, Msg>) {
        for &peer in &self.everyone {
            ctx.send(self.id, peer, msg.clone());
        }
    }

    fn maybe_vote(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if self.sent_vote < self.round {
            self.sent_vote = self.round;
            self.broadcast(
                &Msg::Vote {
                    r: self.round,
                    v: self.preference,
                },
                ctx,
            );
        }
    }

    fn maybe_aux(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        let votes = self.votes.get(&self.round).cloned().unwrap_or_default();
        if votes.len() < self.n - self.f {
            return;
        }
        if self.sent_aux >= self.round {
            return;
        }
        self.sent_aux = self.round;
        let count_true = votes.iter().filter(|&&b| b).count();
        let count_false = votes.len() - count_true;
        let majority = if count_true > 2 * self.n / 3 {
            Some(true)
        } else if count_false > 2 * self.n / 3 {
            Some(false)
        } else {
            None
        };
        self.broadcast(
            &Msg::Aux {
                r: self.round,
                v: majority,
            },
            ctx,
        );
    }

    fn maybe_advance(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        let auxes = self.auxes.get(&self.round).cloned().unwrap_or_default();
        if auxes.len() < self.n - self.f {
            return;
        }
        let mut s = std::collections::BTreeSet::new();
        for b in auxes.iter().flatten() {
            s.insert(*b);
        }
        let coin_bit = (self.coin)(self.round);
        if s.len() == 1 {
            let v = *s.iter().next().unwrap();
            self.preference = v;
            if v == coin_bit {
                self.decision = Some(v);
                return;
            }
        } else {
            self.preference = coin_bit;
        }
        self.round += 1;
        self.maybe_vote(ctx);
    }
}

impl Process for BrachaAbaNode {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        self.maybe_vote(ctx);
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::Vote { r, v } => {
                self.votes.entry(r).or_default().push(v);
                if r == self.round {
                    self.maybe_aux(ctx);
                }
            }
            Msg::Aux { r, v } => {
                self.auxes.entry(r).or_default().push(v);
                if r == self.round {
                    self.maybe_advance(ctx);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Scheduler};

    use super::*;

    fn always_true(_r: u32) -> bool {
        true
    }

    fn build(inputs: &[bool], coin: CoinFn) -> Scheduler<BrachaAbaNode> {
        let n = inputs.len();
        let f = (n - 1) / 3;
        let ids: Vec<NodeId> =
            (0..n).map(|i| NodeId(u32::try_from(i).unwrap())).collect();
        let mut sched = Scheduler::<BrachaAbaNode>::new(0);
        for (i, &id) in ids.iter().enumerate() {
            sched
                .add_node(BrachaAbaNode::new(
                    id,
                    ids.clone(),
                    n,
                    f,
                    inputs[i],
                    coin,
                ))
                .unwrap();
        }
        sched
    }

    #[test]
    fn validity_all_true_with_matching_coin() {
        let mut sched = build(&[true, true, true, true], always_true);
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 1000).unwrap();
        for id in [NodeId(0), NodeId(1), NodeId(2), NodeId(3)] {
            assert_eq!(sched.node(id).unwrap().decision, Some(true));
        }
    }
}
