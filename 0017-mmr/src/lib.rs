//! Module 0017: a minimal MMR-style signature-free ABA.
//!
//! Implements the BV-broadcast primitive plus the MMR
//! decision loop. The common coin is supplied externally.

#![warn(missing_docs)]

use sim::{Envelope, NodeId, Process, StepCtx};

/// Wire message type.
#[derive(Clone, Debug)]
pub enum Msg {
    /// BV-broadcast at round `r` with value `v`.
    Bv {
        /// Round number.
        r: u32,
        /// Value being BV-broadcast.
        v: bool,
    },
    /// MMR Aux phase: process reports its `bin_values` set.
    Aux {
        /// Round number.
        r: u32,
        /// `Some(v)` if the set is `{v}`, `None` if the set
        /// has both 0 and 1.
        v: Option<bool>,
    },
}

/// Common coin oracle.
pub type CoinFn = fn(round: u32) -> bool;

/// MMR ABA process.
pub struct MmrNode {
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
    /// Per-(round, value) BV receive set: who sent `Bv(r, v)`.
    bv_receivers: std::collections::HashMap<
        (u32, bool),
        std::collections::BTreeSet<NodeId>,
    >,
    /// Have we already echoed `Bv(r, v)`?
    bv_echoed: std::collections::BTreeSet<(u32, bool)>,
    /// Per-round `bin_values` set.
    bin_values:
        std::collections::HashMap<u32, std::collections::BTreeSet<bool>>,
    /// Per-round Aux deliveries.
    aux_received: std::collections::HashMap<u32, Vec<Option<bool>>>,
    sent_initial_bv: u32,
    sent_aux: u32,
}

impl MmrNode {
    /// Build an MMR node.
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
            bv_receivers: std::collections::HashMap::new(),
            bv_echoed: std::collections::BTreeSet::new(),
            bin_values: std::collections::HashMap::new(),
            aux_received: std::collections::HashMap::new(),
            sent_initial_bv: 0,
            sent_aux: 0,
        }
    }

    fn broadcast(&self, msg: &Msg, ctx: &mut StepCtx<'_, Msg>) {
        for &peer in &self.everyone {
            ctx.send(self.id, peer, msg.clone());
        }
    }

    fn maybe_initial_bv(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if self.sent_initial_bv < self.round {
            self.sent_initial_bv = self.round;
            self.broadcast(
                &Msg::Bv {
                    r: self.round,
                    v: self.preference,
                },
                ctx,
            );
        }
    }

    fn handle_bv(
        &mut self,
        from: NodeId,
        r: u32,
        v: bool,
        ctx: &mut StepCtx<'_, Msg>,
    ) {
        let key = (r, v);
        let set = self.bv_receivers.entry(key).or_default();
        set.insert(from);
        let count = set.len();
        // Amplification: if > f distinct senders, echo once.
        if count > self.f && self.bv_echoed.insert(key) {
            self.broadcast(&Msg::Bv { r, v }, ctx);
        }
        // Delivery: > 2f distinct senders.
        if count > 2 * self.f {
            self.bin_values.entry(r).or_default().insert(v);
            if r == self.round {
                self.maybe_aux(ctx);
            }
        }
    }

    fn maybe_aux(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        let bin = self.bin_values.get(&self.round);
        if bin.is_none_or(std::collections::BTreeSet::is_empty) {
            return;
        }
        if self.sent_aux >= self.round {
            return;
        }
        let bin = bin.unwrap();
        self.sent_aux = self.round;
        let to_report = if bin.len() == 1 {
            bin.iter().copied().next()
        } else {
            None
        };
        self.broadcast(
            &Msg::Aux {
                r: self.round,
                v: to_report,
            },
            ctx,
        );
    }

    fn maybe_advance(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        let auxes = self
            .aux_received
            .get(&self.round)
            .cloned()
            .unwrap_or_default();
        if auxes.len() < self.n - self.f {
            return;
        }
        let coin = (self.coin)(self.round);
        let count_true = auxes.iter().filter(|&&v| v == Some(true)).count();
        let count_false = auxes.iter().filter(|&&v| v == Some(false)).count();
        let dominant = if count_true > 2 * self.f {
            Some(true)
        } else if count_false > 2 * self.f {
            Some(false)
        } else {
            None
        };
        if let Some(v) = dominant {
            if v == coin {
                self.decision = Some(v);
                return;
            }
            self.preference = v;
        } else {
            self.preference = coin;
        }
        self.round += 1;
        self.maybe_initial_bv(ctx);
    }
}

impl Process for MmrNode {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        self.maybe_initial_bv(ctx);
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::Bv { r, v } => {
                self.handle_bv(env.from, r, v, ctx);
            }
            Msg::Aux { r, v } => {
                self.aux_received.entry(r).or_default().push(v);
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

    fn build(inputs: &[bool], coin: CoinFn) -> Scheduler<MmrNode> {
        let n = inputs.len();
        let f = (n - 1) / 3;
        let ids: Vec<NodeId> =
            (0..n).map(|i| NodeId(u32::try_from(i).unwrap())).collect();
        let mut sched = Scheduler::<MmrNode>::new(0);
        for (i, &id) in ids.iter().enumerate() {
            sched
                .add_node(MmrNode::new(id, ids.clone(), n, f, inputs[i], coin))
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
