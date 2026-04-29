//! Module 0019: a minimal 2PC simulator.

#![warn(missing_docs)]

use sim::{Envelope, NodeId, Process, StepCtx};

/// 2PC wire message.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Coordinator -> participants: prepare for commit.
    Prepare,
    /// Participant -> coordinator: vote yes/no.
    Vote {
        /// Yes (true) or no (false).
        yes: bool,
    },
    /// Coordinator -> participants: final decision.
    Decide {
        /// Commit (true) or abort (false).
        commit: bool,
    },
}

/// 2PC participant state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// Initial.
    Initial,
    /// Vote-yes already sent; awaiting decision.
    Prepared,
    /// Committed.
    Committed,
    /// Aborted.
    Aborted,
}

/// 2PC node (coordinator or participant).
#[allow(clippy::struct_excessive_bools)]
pub struct TpcNode {
    id: NodeId,
    coord: NodeId,
    participants: Vec<NodeId>,
    is_coord: bool,
    /// Whether this participant will vote YES.
    pub will_vote_yes: bool,
    /// Coordinator: votes received so far.
    votes_received: Vec<bool>,
    /// Coordinator: whether to crash after PREPARE phase (test).
    pub crash_after_prepare: bool,
    crashed: bool,
    /// Local state.
    pub state: State,
    sent_prepare: bool,
}

impl TpcNode {
    /// Build a coordinator.
    pub fn coordinator(
        id: NodeId,
        participants: Vec<NodeId>,
        crash_after_prepare: bool,
    ) -> Self {
        Self {
            id,
            coord: id,
            participants,
            is_coord: true,
            will_vote_yes: true,
            votes_received: Vec::new(),
            crash_after_prepare,
            crashed: false,
            state: State::Initial,
            sent_prepare: false,
        }
    }

    /// Build a participant.
    pub fn participant(id: NodeId, coord: NodeId, will_vote_yes: bool) -> Self {
        Self {
            id,
            coord,
            participants: Vec::new(),
            is_coord: false,
            will_vote_yes,
            votes_received: Vec::new(),
            crash_after_prepare: false,
            crashed: false,
            state: State::Initial,
            sent_prepare: false,
        }
    }
}

impl Process for TpcNode {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if self.is_coord && !self.sent_prepare && !self.crashed {
            self.sent_prepare = true;
            for &p in &self.participants {
                ctx.send(self.id, p, Msg::Prepare);
            }
        }
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        if self.crashed {
            return;
        }
        match env.msg {
            Msg::Prepare => {
                debug_assert!(!self.is_coord);
                self.state = State::Prepared;
                ctx.send(
                    self.id,
                    self.coord,
                    Msg::Vote {
                        yes: self.will_vote_yes,
                    },
                );
            }
            Msg::Vote { yes } => {
                debug_assert!(self.is_coord);
                self.votes_received.push(yes);
                if self.votes_received.len() == self.participants.len() {
                    if self.crash_after_prepare {
                        // Simulate the coordinator crashing
                        // before sending the decision: stop
                        // doing anything.
                        self.crashed = true;
                        return;
                    }
                    let commit = self.votes_received.iter().all(|&v| v);
                    self.state = if commit {
                        State::Committed
                    } else {
                        State::Aborted
                    };
                    for &p in &self.participants {
                        ctx.send(self.id, p, Msg::Decide { commit });
                    }
                }
            }
            Msg::Decide { commit } => {
                debug_assert!(!self.is_coord);
                self.state = if commit {
                    State::Committed
                } else {
                    State::Aborted
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Scheduler};

    use super::*;

    fn build(votes: &[bool], crash_coord: bool) -> Scheduler<TpcNode> {
        let n = votes.len();
        let coord = NodeId(0);
        let participants: Vec<NodeId> =
            (1..=n).map(|i| NodeId(u32::try_from(i).unwrap())).collect();
        let mut sched = Scheduler::<TpcNode>::new(0);
        sched
            .add_node(TpcNode::coordinator(
                coord,
                participants.clone(),
                crash_coord,
            ))
            .unwrap();
        for (i, &p) in participants.iter().enumerate() {
            sched
                .add_node(TpcNode::participant(p, coord, votes[i]))
                .unwrap();
        }
        sched
    }

    #[test]
    fn happy_path_commits() {
        let mut sched = build(&[true, true, true], false);
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 100).unwrap();
        for i in 0..4 {
            assert_eq!(sched.node(NodeId(i)).unwrap().state, State::Committed);
        }
    }

    #[test]
    fn one_no_aborts_all() {
        let mut sched = build(&[true, false, true], false);
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 100).unwrap();
        for i in 0..4 {
            assert_eq!(sched.node(NodeId(i)).unwrap().state, State::Aborted);
        }
    }

    #[test]
    fn coordinator_crash_blocks_participants() {
        let mut sched = build(&[true, true, true], true);
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 100).unwrap();
        // Participants vote YES and reach Prepared, but the
        // coordinator crashes before broadcasting Decide. They
        // are blocked in Prepared.
        for i in 1..=3 {
            assert_eq!(sched.node(NodeId(i)).unwrap().state, State::Prepared);
        }
    }
}
