//! Module 0006: a synchronous round-counter Byzantine broadcast
//! illustrating the `f + 1` round bound.
//!
//! With `n = 4, f = 1`, the canonical signed-message broadcast
//! terminates in 2 rounds. We expose a parameter `rounds` that
//! lets the protocol be artificially shortened, and we
//! demonstrate that with `rounds = 1` an adversary can break
//! agreement, while `rounds = 2` (the Dolev-Strong bound)
//! preserves it.

#![warn(missing_docs)]

use sim::{Adversary, Envelope, NodeId, Process, SimRng, StepCtx, Time};

/// Wire message type.
#[derive(Clone, Debug)]
pub enum Msg {
    /// A signed value with a chain of forwarding signers.
    Signed {
        /// The value being asserted.
        v: bool,
        /// Ordered list of signers (commander first).
        chain: Vec<NodeId>,
    },
}

/// A signed-broadcast process.
pub struct SbNode {
    id: NodeId,
    everyone: Vec<NodeId>,
    is_commander: bool,
    /// Commander's value (used only by the commander).
    pub intent: bool,
    /// For Byzantine commanders: alternative intent and target
    /// recipients.
    pub equivocation: Option<(bool, Vec<NodeId>)>,
    /// Allowed chain length: the protocol's *round* bound.
    /// `rounds = 2` is the Dolev-Strong matching upper bound for
    /// `f = 1`; `rounds = 1` shortens it to break the bound.
    rounds: usize,
    /// Set of values seen with a chain of length up to `rounds`.
    seen: std::collections::BTreeSet<bool>,
    /// Final decision (after the synchronous bound).
    pub decision: Option<bool>,
    started: bool,
}

impl SbNode {
    /// Build a commander.
    pub fn commander(
        id: NodeId,
        everyone: Vec<NodeId>,
        intent: bool,
        equivocation: Option<(bool, Vec<NodeId>)>,
        rounds: usize,
    ) -> Self {
        Self {
            id,
            everyone,
            is_commander: true,
            intent,
            equivocation,
            rounds,
            seen: std::collections::BTreeSet::new(),
            decision: None,
            started: false,
        }
    }

    /// Build a follower.
    pub fn follower(id: NodeId, everyone: Vec<NodeId>, rounds: usize) -> Self {
        Self {
            id,
            everyone,
            is_commander: false,
            intent: false,
            equivocation: None,
            rounds,
            seen: std::collections::BTreeSet::new(),
            decision: None,
            started: false,
        }
    }
}

impl Process for SbNode {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if !self.is_commander || self.started {
            return;
        }
        self.started = true;
        // Round 0: commander sends signed value to each follower.
        for &peer in &self.everyone {
            if peer == self.id {
                continue;
            }
            let v = if let Some((alt, ref tgts)) = self.equivocation
                && tgts.contains(&peer)
            {
                alt
            } else {
                self.intent
            };
            ctx.send(
                self.id,
                peer,
                Msg::Signed {
                    v,
                    chain: vec![self.id],
                },
            );
        }
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        let Msg::Signed { v, chain } = env.msg;
        // Commander signature must be the first link.
        let commander = self.everyone[0];
        if chain.first() != Some(&commander) {
            return;
        }
        // Distinct signers.
        let mut distinct = chain.clone();
        distinct.sort();
        distinct.dedup();
        if distinct.len() != chain.len() {
            return;
        }
        // Chain length is bounded by rounds + 1 (commander + r
        // forwards).
        if chain.len() > self.rounds + 1 {
            return;
        }
        let was_new = self.seen.insert(v);

        // Forward iff we are not the commander and we still have
        // budget for one more signature.
        if !self.is_commander
            && was_new
            && chain.len() <= self.rounds
            && !chain.contains(&self.id)
        {
            let mut new_chain = chain;
            new_chain.push(self.id);
            for &peer in &self.everyone {
                if peer == self.id || new_chain.contains(&peer) {
                    continue;
                }
                ctx.send(
                    self.id,
                    peer,
                    Msg::Signed {
                        v,
                        chain: new_chain.clone(),
                    },
                );
            }
        }

        // Decide: with f = 1, after `rounds = f + 1 = 2`, we
        // accept if exactly one value is in `seen`; otherwise
        // default to `false`.
        if !self.is_commander && self.seen.len() == 1 {
            self.decision = Some(*self.seen.iter().next().unwrap());
        } else if !self.is_commander && self.seen.len() >= 2 {
            self.decision = Some(false);
        }
    }
}

/// Adversary that drops every honest forward (any signed message
/// whose chain length is at least 2). Only the commander's direct
/// round-0 messages are delivered. Combined with a Byzantine
/// commander that equivocates, this exhibits the agreement break
/// when the round budget is below the Dolev-Strong lower bound.
pub struct DropAllForwards;

impl Adversary<Msg> for DropAllForwards {
    fn intercept(
        &mut self,
        env: Envelope<Msg>,
        now: Time,
        _rng: &mut SimRng,
    ) -> Vec<(Time, Envelope<Msg>)> {
        let Msg::Signed { ref chain, .. } = env.msg;
        if chain.len() >= 2 {
            return Vec::new();
        }
        vec![(now + 1, env)]
    }
}

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Scheduler};

    use super::*;

    fn build(
        rounds: usize,
        eq: Option<(bool, Vec<NodeId>)>,
    ) -> Scheduler<SbNode> {
        let everyone = vec![NodeId(0), NodeId(1), NodeId(2), NodeId(3)];
        let mut sched = Scheduler::<SbNode>::new(0);
        sched
            .add_node(SbNode::commander(
                NodeId(0),
                everyone.clone(),
                true,
                eq,
                rounds,
            ))
            .unwrap();
        for &id in &everyone[1..] {
            sched
                .add_node(SbNode::follower(id, everyone.clone(), rounds))
                .unwrap();
        }
        sched
    }

    #[test]
    fn rounds_two_succeeds_under_byzantine_commander() {
        // f = 1, rounds = f + 1 = 2. Byzantine commander sends
        // `false` to NodeId(1) only; honest commander to others.
        // After 2 rounds, every honest follower has heard both
        // values from forwards and decides the conservative
        // `false`. Crucially they all *agree*.
        let mut sched = build(2, Some((false, vec![NodeId(1)])));
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 100).unwrap();
        let mut decisions = Vec::new();
        for id in [NodeId(1), NodeId(2), NodeId(3)] {
            decisions.push(sched.node(id).unwrap().decision);
        }
        assert!(decisions.iter().all(|&d| d == decisions[0]));
        assert!(decisions[0].is_some());
    }

    #[test]
    fn rounds_one_can_break_agreement() {
        // f = 1, rounds = 1, but adversary drops every forward
        // so the protocol effectively executes only round 0.
        // Byzantine commander sends `true` to NodeId(1, 2) and
        // `false` to NodeId(3); each honest follower sees only
        // its direct value, and they disagree.
        let mut sched = build(1, Some((false, vec![NodeId(3)])));
        let mut adv = DropAllForwards;
        sched.run(&mut adv, 100).unwrap();
        let d1 = sched.node(NodeId(1)).unwrap().decision;
        let d2 = sched.node(NodeId(2)).unwrap().decision;
        let d3 = sched.node(NodeId(3)).unwrap().decision;
        // Below the bound, agreement does not hold: NodeId(1, 2)
        // see `true`; NodeId(3) sees `false`.
        assert_eq!(d1, Some(true));
        assert_eq!(d2, Some(true));
        assert_eq!(d3, Some(false));
    }
}
