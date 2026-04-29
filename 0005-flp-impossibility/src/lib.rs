//! Module 0005: a constructive illustration of FLP-style
//! adversarial scheduling.
//!
//! We implement a tiny two-process consensus that exchanges
//! `Vote(b)` messages and decides on the majority. With a benign
//! adversary, both processes decide quickly. With an adversary
//! that *withholds* one of the two votes, the protocol fails to
//! reach termination within any preset bound. This is not a proof
//! of FLP (which lives in the README); it is a controlled
//! demonstration of the adversary-schedule freedom that makes the
//! impossibility tight.

#![warn(missing_docs)]

use sim::{Adversary, Envelope, NodeId, Process, SimRng, StepCtx, Time};

/// Wire message used by the toy consensus.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Vote with a binary value.
    Vote {
        /// The vote.
        b: bool,
    },
}

/// A two-process consensus participant.
pub struct VoterNode {
    id: NodeId,
    peer: NodeId,
    /// This process's input.
    input: bool,
    /// Whether the input has been broadcast.
    sent: bool,
    /// Set of received votes (own input recorded once received
    /// from the loopback).
    received_my: bool,
    received_peer: Option<bool>,
    /// Final decision.
    pub decision: Option<bool>,
}

impl VoterNode {
    /// Build a voter with the given input.
    pub fn new(id: NodeId, peer: NodeId, input: bool) -> Self {
        Self {
            id,
            peer,
            input,
            sent: false,
            received_my: false,
            received_peer: None,
            decision: None,
        }
    }

    fn try_decide(&mut self) {
        // Decision rule: decide once both votes received.
        // Termination requires both deliveries; FLP says an
        // adversary may withhold one indefinitely.
        if self.received_my && self.received_peer.is_some() {
            let peer = self.received_peer.unwrap();
            // Majority of 2 is the OR of the two inputs (a tie
            // breaks toward `false` here, mirroring the
            // conservative LSP default).
            self.decision = Some(self.input || peer);
        }
    }
}

impl Process for VoterNode {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if !self.sent {
            self.sent = true;
            // Loopback to self and one to peer.
            ctx.send(self.id, self.id, Msg::Vote { b: self.input });
            ctx.send(self.id, self.peer, Msg::Vote { b: self.input });
        }
    }

    fn on_receive(&mut self, env: Envelope<Msg>, _ctx: &mut StepCtx<'_, Msg>) {
        let Msg::Vote { b } = env.msg;
        if env.from == self.id {
            self.received_my = true;
        } else {
            self.received_peer = Some(b);
        }
        self.try_decide();
    }
}

/// Adversary that schedules every message in delivery order
/// *except* the one peer-to-peer message destined for `target`.
/// Used to keep one process bivalent indefinitely.
pub struct WithholdToOne {
    /// The recipient whose peer message is withheld.
    pub target: NodeId,
}

impl Adversary<Msg> for WithholdToOne {
    fn intercept(
        &mut self,
        env: Envelope<Msg>,
        now: Time,
        _rng: &mut SimRng,
    ) -> Vec<(Time, Envelope<Msg>)> {
        if env.to == self.target && env.from != self.target {
            return Vec::new();
        }
        vec![(now + 1, env)]
    }
}

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Scheduler};

    use super::*;

    fn pair(b0: bool, b1: bool) -> Scheduler<VoterNode> {
        let mut sched = Scheduler::<VoterNode>::new(0);
        sched
            .add_node(VoterNode::new(NodeId(0), NodeId(1), b0))
            .unwrap();
        sched
            .add_node(VoterNode::new(NodeId(1), NodeId(0), b1))
            .unwrap();
        sched
    }

    #[test]
    fn benign_schedule_terminates() {
        let mut sched = pair(true, false);
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 1000).unwrap();
        for id in [NodeId(0), NodeId(1)] {
            assert_eq!(sched.node(id).unwrap().decision, Some(true));
        }
    }

    #[test]
    fn withhold_breaks_termination_for_target() {
        let mut sched = pair(true, false);
        let mut adv = WithholdToOne { target: NodeId(1) };
        sched.run(&mut adv, 1000).unwrap();
        // node 0 hears its loopback and node 1's vote: decides.
        // node 1 hears only its own loopback (peer-to-peer is
        // dropped): never decides.
        assert_eq!(sched.node(NodeId(0)).unwrap().decision, Some(true));
        assert_eq!(sched.node(NodeId(1)).unwrap().decision, None);
    }
}
