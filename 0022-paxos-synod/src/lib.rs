//! Module 0022: Paxos Synod (single-decree).
//!
//! Three combined proposer-acceptor nodes; one initial
//! proposer drives a ballot. Acceptors respond with promises
//! and accepteds. The proposer decides on a value once a
//! quorum has accepted.

#![warn(missing_docs)]
#![allow(clippy::cast_possible_truncation)]

use sim::{Envelope, NodeId, Process, StepCtx};

/// Wire message.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Proposer -> acceptors: prepare for ballot `b`.
    Prepare {
        /// Ballot number.
        b: u32,
    },
    /// Acceptor -> proposer: promise to ignore lower ballots.
    Promise {
        /// Ballot.
        b: u32,
        /// Last accepted (ballot, value), if any.
        last: Option<(u32, u32)>,
    },
    /// Proposer -> acceptors: accept value `v` at ballot `b`.
    Accept {
        /// Ballot.
        b: u32,
        /// Value.
        v: u32,
    },
    /// Acceptor -> proposer: accepted at ballot `b`.
    Accepted {
        /// Ballot.
        b: u32,
        /// Value.
        v: u32,
    },
}

/// Synod node combining proposer and acceptor roles.
pub struct SynodNode {
    id: NodeId,
    everyone: Vec<NodeId>,
    /// This node's preferred value (used if proposer).
    pub preferred: u32,
    /// Whether this node is the initial proposer.
    pub is_proposer: bool,
    /// Initial ballot (proposer only).
    pub initial_ballot: u32,
    /// Acceptor's promised ballot (highest seen).
    promised: Option<u32>,
    /// Acceptor's accepted (ballot, value).
    accepted: Option<(u32, u32)>,
    /// Proposer's collected promises.
    promises: std::collections::BTreeMap<NodeId, Option<(u32, u32)>>,
    /// Proposer's collected accepteds.
    accepteds: std::collections::BTreeSet<NodeId>,
    /// Proposer's current ballot.
    pub current_ballot: u32,
    /// Proposer's chosen value (after Phase 1).
    proposer_value: Option<u32>,
    /// Final decision.
    pub decision: Option<u32>,
    started: bool,
}

impl SynodNode {
    /// Build a Synod node.
    pub fn new(
        id: NodeId,
        everyone: Vec<NodeId>,
        preferred: u32,
        is_proposer: bool,
        initial_ballot: u32,
    ) -> Self {
        Self {
            id,
            everyone,
            preferred,
            is_proposer,
            initial_ballot,
            promised: None,
            accepted: None,
            promises: std::collections::BTreeMap::new(),
            accepteds: std::collections::BTreeSet::new(),
            current_ballot: 0,
            proposer_value: None,
            decision: None,
            started: false,
        }
    }

    fn quorum(&self) -> usize {
        self.everyone.len() / 2 + 1
    }
}

impl Process for SynodNode {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if !self.is_proposer || self.started {
            return;
        }
        self.started = true;
        self.current_ballot = self.initial_ballot;
        for &peer in &self.everyone {
            ctx.send(
                self.id,
                peer,
                Msg::Prepare {
                    b: self.current_ballot,
                },
            );
        }
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::Prepare { b } => {
                if self.promised.is_none_or(|p| b > p) {
                    self.promised = Some(b);
                    ctx.send(
                        self.id,
                        env.from,
                        Msg::Promise {
                            b,
                            last: self.accepted,
                        },
                    );
                }
            }
            Msg::Promise { b, last } => {
                if b != self.current_ballot {
                    return;
                }
                self.promises.insert(env.from, last);
                if self.promises.len() >= self.quorum()
                    && self.proposer_value.is_none()
                {
                    let highest_accepted = self
                        .promises
                        .values()
                        .filter_map(|v| *v)
                        .max_by_key(|(b, _)| *b);
                    let v = match highest_accepted {
                        Some((_, v)) => v,
                        None => self.preferred,
                    };
                    self.proposer_value = Some(v);
                    for &peer in &self.everyone {
                        ctx.send(self.id, peer, Msg::Accept { b, v });
                    }
                }
            }
            Msg::Accept { b, v } => {
                if self.promised.is_none_or(|p| b >= p) {
                    self.promised = Some(b);
                    self.accepted = Some((b, v));
                    ctx.send(self.id, env.from, Msg::Accepted { b, v });
                }
            }
            Msg::Accepted { b, v: _ } => {
                if b != self.current_ballot {
                    return;
                }
                self.accepteds.insert(env.from);
                if self.accepteds.len() >= self.quorum() {
                    self.decision = self.proposer_value;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Scheduler};

    use super::*;

    #[test]
    fn single_proposer_decides() {
        let everyone = vec![NodeId(0), NodeId(1), NodeId(2)];
        let mut sched = Scheduler::<SynodNode>::new(0);
        sched
            .add_node(SynodNode::new(NodeId(0), everyone.clone(), 42, true, 1))
            .unwrap();
        for &id in &everyone[1..] {
            sched
                .add_node(SynodNode::new(id, everyone.clone(), 0, false, 0))
                .unwrap();
        }
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 1000).unwrap();
        let proposer = sched.node(NodeId(0)).unwrap();
        assert_eq!(proposer.decision, Some(42));
    }
}
