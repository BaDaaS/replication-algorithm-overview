//! Module 0004: a synchronous round-based OM(1) Byzantine Generals
//! simulation with `n = 4, f = 1`.
//!
//! Round 0: commander `c` sends its value `v` to each lieutenant.
//! Round 1: each lieutenant relays the value it received to every
//! other lieutenant. After round 1, each lieutenant decides the
//! majority of (its directly-received value and the values
//! relayed by other lieutenants).
//!
//! With one Byzantine commander among `n = 4`, three honest
//! lieutenants reach agreement on the majority of the three values
//! the commander chose to send (no matter the equivocation
//! pattern); with one Byzantine lieutenant and an honest
//! commander, the commander's value still wins.

#![warn(missing_docs)]

use sim::{Envelope, NodeId, Process, StepCtx};

/// The decision domain. We use a binary value (`Attack` /
/// `Retreat`) because the OM(1) majority rule is the standard
/// single-bit Byzantine agreement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decision {
    /// Attack the city.
    Attack,
    /// Retreat from the city.
    Retreat,
}

/// Wire message used by the protocol.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Round 0: the commander telegraphs the order.
    Order {
        /// The order.
        v: Decision,
    },
    /// Round 1: a lieutenant forwards the order it received from
    /// the commander.
    Forward {
        /// The forwarded order.
        v: Decision,
    },
}

/// A general (commander or lieutenant).
pub struct General {
    id: NodeId,
    /// Indices of all generals (commander = 0).
    everyone: Vec<NodeId>,
    is_commander: bool,
    /// Commander's intended value (used only if `is_commander`).
    pub commander_intent: Decision,
    /// For Byzantine commanders, an alternative value to send to a
    /// designated subset of lieutenants.
    pub equivocation: Option<(Decision, Vec<NodeId>)>,
    /// What the lieutenant heard directly from the commander.
    direct: Option<Decision>,
    /// Forwarded values heard from other lieutenants.
    forwards: Vec<Decision>,
    /// Final decision.
    pub decision: Option<Decision>,
    started: bool,
}

impl General {
    /// Build a commander.
    pub fn commander(
        id: NodeId,
        everyone: Vec<NodeId>,
        intent: Decision,
        equivocation: Option<(Decision, Vec<NodeId>)>,
    ) -> Self {
        Self {
            id,
            everyone,
            is_commander: true,
            commander_intent: intent,
            equivocation,
            direct: None,
            forwards: Vec::new(),
            decision: None,
            started: false,
        }
    }

    /// Build a lieutenant.
    pub fn lieutenant(id: NodeId, everyone: Vec<NodeId>) -> Self {
        Self {
            id,
            everyone,
            is_commander: false,
            commander_intent: Decision::Retreat,
            equivocation: None,
            direct: None,
            forwards: Vec::new(),
            decision: None,
            started: false,
        }
    }

    fn lieutenants(&self) -> impl Iterator<Item = NodeId> + '_ {
        let commander = self.everyone[0];
        self.everyone
            .iter()
            .copied()
            .filter(move |&p| p != commander)
    }

    fn finalise(&mut self) {
        // Majority vote over (direct, forwards). We're in OM(1),
        // n = 4, f = 1, so each lieutenant has up to 1 direct
        // value and up to 2 forwards.
        let mut votes = Vec::new();
        if let Some(v) = self.direct {
            votes.push(v);
        }
        votes.extend(self.forwards.iter().copied());
        let attack = votes.iter().filter(|&&v| v == Decision::Attack).count();
        let retreat = votes.len() - attack;
        // tie-break: Retreat (the conservative default in
        // LSP 1982 Section 4).
        self.decision = Some(match attack.cmp(&retreat) {
            std::cmp::Ordering::Greater => Decision::Attack,
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                Decision::Retreat
            }
        });
    }
}

impl Process for General {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if !self.is_commander || self.started {
            return;
        }
        self.started = true;
        // Round 0: commander sends to each lieutenant.
        for lieut in self.lieutenants() {
            let v = if let Some((alt, ref targets)) = self.equivocation
                && targets.contains(&lieut)
            {
                alt
            } else {
                self.commander_intent
            };
            ctx.send(self.id, lieut, Msg::Order { v });
        }
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::Order { v } => {
                if !self.is_commander {
                    self.direct = Some(v);
                    // Round 1: forward to other lieutenants.
                    let me = self.id;
                    let commander = self.everyone[0];
                    let peers: Vec<NodeId> = self
                        .everyone
                        .iter()
                        .copied()
                        .filter(|&p| p != commander && p != me)
                        .collect();
                    for peer in peers {
                        ctx.send(me, peer, Msg::Forward { v });
                    }
                }
            }
            Msg::Forward { v } => {
                if !self.is_commander {
                    self.forwards.push(v);
                    // In OM(1) with n = 4, we expect 2 forwards
                    // (from the other 2 lieutenants). Decide once
                    // we have collected them.
                    let expected_forwards = self.everyone.len() - 2;
                    if self.forwards.len() >= expected_forwards
                        && self.direct.is_some()
                    {
                        self.finalise();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Scheduler};

    use super::*;

    fn build(
        intent: Decision,
        eq: Option<(Decision, Vec<NodeId>)>,
    ) -> Scheduler<General> {
        let everyone = vec![NodeId(0), NodeId(1), NodeId(2), NodeId(3)];
        let mut sched = Scheduler::<General>::new(0);
        sched
            .add_node(General::commander(
                NodeId(0),
                everyone.clone(),
                intent,
                eq,
            ))
            .unwrap();
        for &id in &everyone[1..] {
            sched
                .add_node(General::lieutenant(id, everyone.clone()))
                .unwrap();
        }
        sched
    }

    #[test]
    fn honest_commander_attack_propagates() {
        let mut sched = build(Decision::Attack, None);
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 100).unwrap();
        for id in [NodeId(1), NodeId(2), NodeId(3)] {
            let g = sched.node(id).unwrap();
            assert_eq!(g.decision, Some(Decision::Attack));
        }
    }

    #[test]
    fn byzantine_commander_equivocation_majority_wins() {
        // Commander sends Attack to L1, L2 and Retreat only to L3.
        // Each lieutenant collects:
        //   L1: direct = Attack, forwards from L2 = Attack,
        //       L3 = Retreat. votes = [A, A, R] -> Attack.
        //   L2: direct = Attack, forwards from L1 = Attack,
        //       L3 = Retreat. votes = [A, A, R] -> Attack.
        //   L3: direct = Retreat, forwards from L1 = Attack,
        //       L2 = Attack. votes = [R, A, A] -> Attack.
        // All three honest lieutenants agree on Attack.
        let mut sched =
            build(Decision::Attack, Some((Decision::Retreat, vec![NodeId(3)])));
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 100).unwrap();
        for id in [NodeId(1), NodeId(2), NodeId(3)] {
            let g = sched.node(id).unwrap();
            assert_eq!(
                g.decision,
                Some(Decision::Attack),
                "lieutenant {id} disagreed"
            );
        }
    }
}
