//! Module 0003: a Two-Generals attempt under the course simulator.
//!
//! This crate exhibits, by simulation, the well-known impossibility:
//! a deterministic two-party agreement protocol cannot guarantee
//! both decisions match under a lossy channel that may drop any
//! message. We implement a finite-bounded "attack-time" protocol
//! and show that with a benign network the parties agree, but a
//! single dropped final acknowledgement breaks agreement.

#![warn(missing_docs)]

use sim::{Envelope, NodeId, Process, StepCtx};

/// Wire message used by the protocol.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Proposal of an attack time.
    Propose {
        /// Attack time.
        t: u64,
    },
    /// Acknowledgement of receipt at sequence number `seq`.
    Ack {
        /// Mirrored proposal time.
        t: u64,
        /// Sequence number; protocol decides after `seq >=
        /// rounds`.
        seq: u32,
    },
}

/// A two-generals process. Index 0 is the proposer; index 1 the
/// receiver.
pub struct General {
    id: NodeId,
    peer: NodeId,
    is_proposer: bool,
    /// Number of acknowledgements before the proposer decides.
    rounds: u32,
    /// Last sequence number observed.
    last_seq: u32,
    /// Final attack-time decision (None = retreat).
    pub decision: Option<u64>,
    proposed_at: u64,
    proposed: bool,
}

impl General {
    /// Build a general.
    pub fn new(
        id: NodeId,
        peer: NodeId,
        is_proposer: bool,
        rounds: u32,
        proposed_at: u64,
    ) -> Self {
        Self {
            id,
            peer,
            is_proposer,
            rounds,
            last_seq: 0,
            decision: None,
            proposed_at,
            proposed: false,
        }
    }
}

impl Process for General {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if self.is_proposer && !self.proposed {
            ctx.send(
                self.id,
                self.peer,
                Msg::Propose {
                    t: self.proposed_at,
                },
            );
            self.proposed = true;
        }
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::Propose { t } => {
                // Receiver (non-proposer) decides on the proposed
                // time and acks back. After acking once, the
                // receiver commits.
                if !self.is_proposer {
                    self.decision = Some(t);
                    ctx.send(self.id, self.peer, Msg::Ack { t, seq: 1 });
                }
            }
            Msg::Ack { t, seq } => {
                self.last_seq = seq;
                if self.is_proposer {
                    if seq >= self.rounds {
                        self.decision = Some(t);
                    } else {
                        ctx.send(
                            self.id,
                            self.peer,
                            Msg::Ack { t, seq: seq + 1 },
                        );
                    }
                } else {
                    // Receiver also pings the seq up while the
                    // proposer wants more rounds.
                    if seq < self.rounds {
                        ctx.send(
                            self.id,
                            self.peer,
                            Msg::Ack { t, seq: seq + 1 },
                        );
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sim::{Adversary, NoOpAdversary, Scheduler, SimRng, Time};

    use super::*;

    fn pair(rounds: u32) -> Scheduler<General> {
        let mut sched = Scheduler::<General>::new(0);
        sched
            .add_node(General::new(NodeId(0), NodeId(1), true, rounds, 42))
            .unwrap();
        sched
            .add_node(General::new(NodeId(1), NodeId(0), false, rounds, 0))
            .unwrap();
        sched
    }

    #[test]
    fn reliable_network_agrees() {
        let mut sched = pair(3);
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 1000).unwrap();
        let g0 = sched.node(NodeId(0)).unwrap();
        let g1 = sched.node(NodeId(1)).unwrap();
        assert_eq!(g0.decision, Some(42));
        assert_eq!(g1.decision, Some(42));
    }

    /// Adversary that drops every Ack with `seq == final_seq`
    /// going to NodeId(0) (the proposer). The receiver therefore
    /// decides Attack on the Propose round, but the proposer
    /// never sees enough Acks and never decides.
    struct DropFinalAck {
        final_seq: u32,
    }
    impl Adversary<Msg> for DropFinalAck {
        fn intercept(
            &mut self,
            env: Envelope<Msg>,
            now: Time,
            _rng: &mut SimRng,
        ) -> Vec<(Time, Envelope<Msg>)> {
            if env.to == NodeId(0)
                && let Msg::Ack { seq, .. } = env.msg
                && seq == self.final_seq
            {
                return Vec::new();
            }
            vec![(now + 1, env)]
        }
    }

    #[test]
    fn dropping_final_ack_breaks_agreement() {
        let mut sched = pair(3);
        let mut adv = DropFinalAck { final_seq: 3 };
        sched.run(&mut adv, 1000).unwrap();
        let g0 = sched.node(NodeId(0)).unwrap();
        let g1 = sched.node(NodeId(1)).unwrap();
        // Agreement fails: the receiver decided, the proposer
        // did not.
        assert_eq!(g1.decision, Some(42));
        assert_eq!(g0.decision, None);
    }
}
