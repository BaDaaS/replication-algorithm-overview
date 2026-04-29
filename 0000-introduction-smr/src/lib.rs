//! Module 0000: Introduction to State-Machine Replication.
//!
//! The crate implements the abstract SMR pattern from Schneider 1990
//! over the course's deterministic simulator. There is no agreement
//! protocol yet: a designated *leader* replica plays the role of
//! the ordering oracle by broadcasting client operations to every
//! follower, and the network model is left to the simulator's
//! adversary. With a `NoOpAdversary`, total-order delivery is free
//! and the SMR safety theorem (see this module's README) can be
//! observed experimentally.

#![warn(missing_docs)]

use std::fmt::Debug;
use std::marker::PhantomData;

use sim::{Envelope, NodeId, Process, StepCtx};

// =====================================================================
// State machine abstraction
// =====================================================================

/// A deterministic state machine, in the sense of Schneider 1990.
///
/// The contract is that for any state `s` and operation `op`, the
/// pair `(apply(s, op), observe(s, op))` is a function of `(s, op)`
/// with no hidden state, no timing dependence, and no platform-
/// specific behaviour.
pub trait StateMachine: Clone + Debug {
    /// Operation type (commands).
    type Op: Clone + Debug;
    /// Output type (the value returned to the client).
    type Out: Clone + Debug + PartialEq;

    /// Apply `op` to the state, mutating in place.
    fn apply(&mut self, op: &Self::Op) -> Self::Out;
}

/// A trivial increment-by-`u64` counter. Used in tests and exercises.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Counter {
    /// Current counter value.
    pub value: u64,
}

/// Operations on `Counter`.
#[derive(Clone, Debug)]
pub enum CounterOp {
    /// Add `delta` to the counter.
    Add(u64),
    /// Multiply the counter by `factor`.
    Mul(u64),
    /// Read the current counter value.
    Read,
}

impl StateMachine for Counter {
    type Op = CounterOp;
    type Out = u64;

    fn apply(&mut self, op: &CounterOp) -> u64 {
        match *op {
            CounterOp::Add(d) => {
                self.value = self.value.wrapping_add(d);
            }
            CounterOp::Mul(f) => {
                self.value = self.value.wrapping_mul(f);
            }
            CounterOp::Read => {}
        }
        self.value
    }
}

// =====================================================================
// Replica wrapper
// =====================================================================

/// A replica of a state machine `S`. The replica applies committed
/// operations in delivery order and remembers the prefix it has
/// observed.
#[derive(Clone, Debug)]
pub struct Replica<S: StateMachine> {
    state: S,
    log: Vec<S::Op>,
    outputs: Vec<S::Out>,
}

impl<S: StateMachine> Replica<S> {
    /// Build a replica wrapping the given initial state.
    pub fn new(initial: S) -> Self {
        Self {
            state: initial,
            log: Vec::new(),
            outputs: Vec::new(),
        }
    }

    /// Apply `op` to the local state, recording it in the log.
    pub fn commit(&mut self, op: S::Op) -> S::Out {
        let out = self.state.apply(&op);
        self.log.push(op);
        self.outputs.push(out.clone());
        out
    }

    /// Borrow the current state.
    pub fn state(&self) -> &S {
        &self.state
    }

    /// Borrow the committed log.
    pub fn log(&self) -> &[S::Op] {
        &self.log
    }

    /// Borrow the produced output sequence.
    pub fn outputs(&self) -> &[S::Out] {
        &self.outputs
    }
}

// =====================================================================
// Leader-broadcast SMR over the course simulator
// =====================================================================

/// Wire message used by the leader-broadcast oracle.
#[derive(Clone, Debug)]
pub enum SmrMsg<S: StateMachine> {
    /// Leader announces operation `op` at sequence number `seq`.
    Commit {
        /// Monotone sequence number.
        seq: u64,
        /// The operation to apply.
        op: S::Op,
    },
}

/// A node in the leader-broadcast SMR.
///
/// One replica is designated the leader; on its first tick it drains
/// its pending operation queue and broadcasts each operation in
/// sequence to every node (including itself, to keep the leader's
/// log construction symmetric with the followers'). All nodes commit
/// operations in the order they receive them.
pub struct LeaderBroadcastNode<S: StateMachine> {
    id: NodeId,
    leader: NodeId,
    peers: Vec<NodeId>,
    pending: Vec<S::Op>,
    next_seq: u64,
    replica: Replica<S>,
    _phantom: PhantomData<S>,
}

impl<S: StateMachine> LeaderBroadcastNode<S> {
    /// Build a node.
    ///
    /// `peers` should include the node's own id. `pending` is only
    /// consumed by the leader; followers should pass an empty
    /// vector.
    pub fn new(
        id: NodeId,
        leader: NodeId,
        peers: Vec<NodeId>,
        pending: Vec<S::Op>,
        initial: S,
    ) -> Self {
        Self {
            id,
            leader,
            peers,
            pending,
            next_seq: 0,
            replica: Replica::new(initial),
            _phantom: PhantomData,
        }
    }

    /// Borrow the underlying replica.
    pub fn replica(&self) -> &Replica<S> {
        &self.replica
    }
}

impl<S: StateMachine + 'static> Process for LeaderBroadcastNode<S> {
    type Message = SmrMsg<S>;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Self::Message>) {
        if self.id != self.leader {
            return;
        }
        let pending = std::mem::take(&mut self.pending);
        for op in pending {
            let seq = self.next_seq;
            self.next_seq += 1;
            for &peer in &self.peers {
                ctx.send(
                    self.id,
                    peer,
                    SmrMsg::Commit {
                        seq,
                        op: op.clone(),
                    },
                );
            }
        }
    }

    fn on_receive(
        &mut self,
        env: Envelope<Self::Message>,
        _ctx: &mut StepCtx<'_, Self::Message>,
    ) {
        let SmrMsg::Commit { seq: _, op } = env.msg;
        self.replica.commit(op);
    }
}

// =====================================================================
// Tests
// =====================================================================

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Scheduler};

    use super::*;

    fn build_smr<S: StateMachine + 'static>(
        n: usize,
        leader_pending: &[S::Op],
        initial: &S,
    ) -> Scheduler<LeaderBroadcastNode<S>> {
        let ids: Vec<NodeId> =
            (0..n).map(|i| NodeId(u32::try_from(i).unwrap())).collect();
        let leader = ids[0];
        let mut sched = Scheduler::<LeaderBroadcastNode<S>>::new(0);
        for (i, id) in ids.iter().enumerate() {
            let pending = if i == 0 {
                leader_pending.to_vec()
            } else {
                Vec::new()
            };
            sched
                .add_node(LeaderBroadcastNode::new(
                    *id,
                    leader,
                    ids.clone(),
                    pending,
                    initial.clone(),
                ))
                .unwrap();
        }
        sched
    }

    #[test]
    fn smr_safety_with_reliable_network() {
        let ops = vec![
            CounterOp::Add(1),
            CounterOp::Add(2),
            CounterOp::Mul(10),
            CounterOp::Add(5),
        ];
        let mut sched = build_smr(4, &ops, &Counter::default());
        let mut adv = NoOpAdversary;
        sched.run(&mut adv, 1000).unwrap();

        let mut states = Vec::new();
        let mut logs = Vec::new();
        for i in 0..4 {
            let n = sched.node(NodeId(i)).unwrap();
            states.push(n.replica().state().clone());
            logs.push(n.replica().log().to_vec());
        }
        // SMR safety: every replica reaches the same state and
        // has the same log of committed operations.
        for s in &states {
            assert_eq!(*s, states[0]);
        }
        for log in &logs {
            assert_eq!(log.len(), ops.len());
        }
        // The state machine is deterministic: 0 + 1 + 2 = 3,
        // 3 * 10 = 30, 30 + 5 = 35.
        assert_eq!(states[0].value, 35);
    }

    #[test]
    fn smr_safety_under_one_drop() {
        // An adversary that drops messages from leader to node 3
        // only. The premise of total-order is broken (eventual
        // delivery fails), but SMR safety on the prefix that did
        // arrive is preserved.
        struct DropToOne;
        impl<M: Clone + std::fmt::Debug> sim::Adversary<M> for DropToOne {
            fn intercept(
                &mut self,
                env: Envelope<M>,
                now: sim::Time,
                _rng: &mut sim::SimRng,
            ) -> Vec<(sim::Time, Envelope<M>)> {
                if env.to == NodeId(3) {
                    Vec::new()
                } else {
                    vec![(now + 1, env)]
                }
            }
        }

        let ops = vec![CounterOp::Add(7), CounterOp::Add(11)];
        let mut sched = build_smr(4, &ops, &Counter::default());
        let mut adv = DropToOne;
        sched.run(&mut adv, 1000).unwrap();

        let n0 = sched.node(NodeId(0)).unwrap().replica().state().value;
        let n1 = sched.node(NodeId(1)).unwrap().replica().state().value;
        let n2 = sched.node(NodeId(2)).unwrap().replica().state().value;
        let n3 = sched.node(NodeId(3)).unwrap().replica().state().value;

        assert_eq!(n0, 18);
        assert_eq!(n1, 18);
        assert_eq!(n2, 18);
        // node 3 got nothing; safety is preserved on the empty
        // prefix
        assert_eq!(n3, 0);
    }
}
