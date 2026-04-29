//! Deterministic simulator for the replication-algorithm-overview
//! course.
//!
//! The simulator is intentionally small and pedagogical. It models a
//! distributed system as a set of nodes that exchange typed messages
//! through a network mediated by an adversary. Logical time is a
//! `u64` tick counter. Determinism is provided by a single seeded
//! ChaCha-based RNG and a stable tie-breaker on the event queue.
//!
//! # Design
//!
//! - A [`Process`] is a node's local protocol state. It reacts to
//!   ticks, message receipts, and timer firings.
//! - The [`Scheduler`] owns the priority queue of pending events
//!   and dispatches them in non-decreasing time order.
//! - Every outgoing message is handed to an [`Adversary`], which
//!   returns the list of deliveries to schedule (zero deliveries
//!   model a drop, two or more model duplication or equivocation).
//! - Network conditions (synchronous, partially synchronous,
//!   asynchronous, partition, equivocation) are encoded entirely in
//!   the adversary, not in the scheduler.
//!
//! # Determinism
//!
//! Two runs with the same seed and the same protocol code produce
//! identical event traces. The event queue is keyed on
//! `(time, monotonic_seq)`; ties at the same time are broken by
//! insertion order.
//!
//! # Example
//!
//! ```
//! use sim::{Adversary, Envelope, NodeId, NoOpAdversary, Process,
//!           Scheduler, StepCtx};
//!
//! #[derive(Clone, Debug)]
//! struct Ping;
//!
//! struct Echo {
//!     id: NodeId,
//!     peer: NodeId,
//!     pinged: bool,
//!     received: u32,
//! }
//!
//! impl Process for Echo {
//!     type Message = Ping;
//!     fn id(&self) -> NodeId { self.id }
//!     fn on_tick(&mut self, ctx: &mut StepCtx<'_, Ping>) {
//!         if !self.pinged {
//!             ctx.send(self.id, self.peer, Ping);
//!             self.pinged = true;
//!         }
//!     }
//!     fn on_receive(&mut self, _env: Envelope<Ping>,
//!                   _ctx: &mut StepCtx<'_, Ping>) {
//!         self.received += 1;
//!     }
//! }
//!
//! let mut sched = Scheduler::new(0);
//! sched.add_node(Echo { id: NodeId(0), peer: NodeId(1),
//!                       pinged: false, received: 0 });
//! sched.add_node(Echo { id: NodeId(1), peer: NodeId(0),
//!                       pinged: false, received: 0 });
//! let mut adv = NoOpAdversary::default();
//! sched.run(&mut adv, 100).unwrap();
//! ```

#![warn(missing_docs)]

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{self, Debug};

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use thiserror::Error;

// =====================================================================
// Time and identity
// =====================================================================

/// Logical simulation time, measured in ticks.
pub type Time = u64;

/// Identifier of a node (process) in the simulated system.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct NodeId(pub u32);

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "n{}", self.0)
    }
}

// =====================================================================
// RNG
// =====================================================================

/// Wrapper around a deterministic ChaCha-based RNG.
///
/// All randomness in the simulator (adversary decisions, leader
/// election, sortition, timer jitter) is funneled through this one
/// source so that re-running with the same seed is exactly
/// reproducible.
pub struct SimRng {
    inner: ChaCha8Rng,
}

impl SimRng {
    /// Create a new RNG seeded by `seed`.
    #[must_use]
    pub fn from_seed(seed: u64) -> Self {
        Self {
            inner: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    /// Borrow the inner `rand`-compatible RNG.
    pub fn inner(&mut self) -> &mut ChaCha8Rng {
        &mut self.inner
    }
}

// =====================================================================
// Messages and processes
// =====================================================================

/// A message envelope carried between nodes.
#[derive(Clone, Debug)]
pub struct Envelope<M: Clone + Debug> {
    /// Sending node.
    pub from: NodeId,
    /// Recipient node.
    pub to: NodeId,
    /// Time at which the envelope was emitted.
    pub sent_at: Time,
    /// Wrapped protocol message.
    pub msg: M,
}

/// Trait implemented by each protocol's local process state.
///
/// A process is a state machine driven by three event kinds:
///
/// - [`on_tick`]: a once-per-time-unit pulse used to bootstrap
///   processes and drive periodic actions.
/// - [`on_receive`]: a message has arrived from another node.
/// - [`on_timer`]: a previously requested timer has fired.
///
/// During any handler the process can append outgoing messages and
/// timer requests through the [`StepCtx`].
///
/// [`on_tick`]: Process::on_tick
/// [`on_receive`]: Process::on_receive
/// [`on_timer`]: Process::on_timer
pub trait Process {
    /// The protocol's message type.
    type Message: Clone + Debug;

    /// Identifier of this process.
    fn id(&self) -> NodeId;

    /// Action taken on a clock tick. Default: no-op.
    fn on_tick(&mut self, _ctx: &mut StepCtx<'_, Self::Message>) {}

    /// Action taken on receipt of an envelope.
    fn on_receive(
        &mut self,
        env: Envelope<Self::Message>,
        ctx: &mut StepCtx<'_, Self::Message>,
    );

    /// Action on a timer firing. Default: no-op.
    fn on_timer(
        &mut self,
        _timer_id: u64,
        _ctx: &mut StepCtx<'_, Self::Message>,
    ) {
    }
}

/// Borrowed context exposed to a process during a step.
///
/// Outgoing messages and timer requests are appended here and
/// scheduled by the [`Scheduler`] after the handler returns.
pub struct StepCtx<'a, M: Clone + Debug> {
    /// Current logical time.
    pub now: Time,
    /// Outgoing message buffer; messages added here are passed to
    /// the adversary for scheduling.
    pub outbox: &'a mut Vec<Envelope<M>>,
    /// Timer requests as `(delay, timer_id)`. The timer fires at
    /// `now + delay`.
    pub timers: &'a mut Vec<(Time, u64)>,
    /// Shared deterministic random source.
    pub rng: &'a mut SimRng,
}

impl<M: Clone + Debug> StepCtx<'_, M> {
    /// Send `msg` from `from` to `to`, stamped with the current
    /// time.
    pub fn send(&mut self, from: NodeId, to: NodeId, msg: M) {
        self.outbox.push(Envelope {
            from,
            to,
            msg,
            sent_at: self.now,
        });
    }

    /// Schedule a timer to fire `delay` ticks from now with the
    /// given numeric id.
    pub fn schedule_timer(&mut self, delay: Time, id: u64) {
        self.timers.push((delay, id));
    }
}

// =====================================================================
// Adversary
// =====================================================================

/// Trait implemented by an adversary (network model and Byzantine
/// behaviour, if any).
///
/// On every outgoing envelope produced by a process, the scheduler
/// calls [`intercept`](Adversary::intercept). The adversary returns
/// a list of deliveries to schedule. Each delivery is an absolute
/// time `at >= now` paired with the envelope to deliver. Returning
/// an empty list models a dropped message; returning more than one
/// delivery models duplication or equivocation (each delivery may
/// carry a mutated envelope).
pub trait Adversary<M: Clone + Debug> {
    /// React to one outgoing envelope.
    fn intercept(
        &mut self,
        env: Envelope<M>,
        now: Time,
        rng: &mut SimRng,
    ) -> Vec<(Time, Envelope<M>)>;
}

/// Default adversary: deliver every message exactly once, one tick
/// later. Models a synchronous, reliable, non-faulty network.
#[derive(Default)]
pub struct NoOpAdversary;

impl<M: Clone + Debug> Adversary<M> for NoOpAdversary {
    fn intercept(
        &mut self,
        env: Envelope<M>,
        now: Time,
        _rng: &mut SimRng,
    ) -> Vec<(Time, Envelope<M>)> {
        vec![(now + 1, env)]
    }
}

// =====================================================================
// Scheduler
// =====================================================================

/// Errors raised by the simulator.
#[derive(Debug, Error)]
pub enum ScheduleError {
    /// A node referenced in a message is not registered.
    #[error("unknown node: {0}")]
    UnknownNode(NodeId),
    /// The simulator hit its step bound without quiescence.
    #[error("step bound reached after {0} steps")]
    StepBound(usize),
    /// Adversary returned a delivery time strictly before `now`.
    #[error("adversary returned past delivery time {past} < {now}")]
    PastDelivery {
        /// The illegal delivery time.
        past: Time,
        /// The current simulation time.
        now: Time,
    },
    /// A duplicate node id was registered.
    #[error("duplicate node id: {0}")]
    DuplicateNode(NodeId),
}

#[derive(Debug)]
enum Event<M: Clone + Debug> {
    Deliver(Envelope<M>),
    Tick(NodeId),
    Timer { node: NodeId, id: u64 },
}

#[derive(Debug)]
struct ScheduledEvent<M: Clone + Debug> {
    at: Time,
    seq: u64,
    event: Event<M>,
}

impl<M: Clone + Debug> Eq for ScheduledEvent<M> {}

impl<M: Clone + Debug> PartialEq for ScheduledEvent<M> {
    fn eq(&self, o: &Self) -> bool {
        self.at == o.at && self.seq == o.seq
    }
}

impl<M: Clone + Debug> PartialOrd for ScheduledEvent<M> {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

impl<M: Clone + Debug> Ord for ScheduledEvent<M> {
    fn cmp(&self, o: &Self) -> Ordering {
        // BinaryHeap is a max-heap; reverse so the earliest event
        // pops first. Ties at the same time are broken by insertion
        // order via `seq`.
        (self.at, self.seq).cmp(&(o.at, o.seq)).reverse()
    }
}

/// The deterministic event-driven simulator.
pub struct Scheduler<P: Process> {
    now: Time,
    queue: BinaryHeap<ScheduledEvent<P::Message>>,
    next_seq: u64,
    nodes: HashMap<NodeId, P>,
    rng: SimRng,
}

impl<P: Process> Scheduler<P> {
    /// Create a new scheduler with the given seed.
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self {
            now: 0,
            queue: BinaryHeap::new(),
            next_seq: 0,
            nodes: HashMap::new(),
            rng: SimRng::from_seed(seed),
        }
    }

    /// Register a node and seed its first tick at time 1.
    ///
    /// # Errors
    ///
    /// Returns [`ScheduleError::DuplicateNode`] if the same id is
    /// added twice.
    pub fn add_node(&mut self, node: P) -> Result<(), ScheduleError> {
        let id = node.id();
        if self.nodes.contains_key(&id) {
            return Err(ScheduleError::DuplicateNode(id));
        }
        self.nodes.insert(id, node);
        self.push(1, Event::Tick(id));
        Ok(())
    }

    /// Borrow a node by id.
    pub fn node(&self, id: NodeId) -> Option<&P> {
        self.nodes.get(&id)
    }

    /// Borrow a node mutably by id.
    pub fn node_mut(&mut self, id: NodeId) -> Option<&mut P> {
        self.nodes.get_mut(&id)
    }

    /// Iterate over registered nodes.
    pub fn nodes(&self) -> impl Iterator<Item = (&NodeId, &P)> {
        self.nodes.iter()
    }

    /// Current simulation time.
    #[must_use]
    pub fn now(&self) -> Time {
        self.now
    }

    fn push(&mut self, at: Time, event: Event<P::Message>) {
        let seq = self.next_seq;
        self.next_seq = self.next_seq.wrapping_add(1);
        self.queue.push(ScheduledEvent { at, seq, event });
    }

    /// Run until the queue empties or `max_steps` is reached.
    ///
    /// # Errors
    ///
    /// - [`ScheduleError::StepBound`] if `max_steps` is reached.
    /// - [`ScheduleError::PastDelivery`] if the adversary requests
    ///   a delivery time in the past.
    /// - [`ScheduleError::UnknownNode`] if a message targets an
    ///   unregistered node.
    pub fn run<A: Adversary<P::Message>>(
        &mut self,
        adversary: &mut A,
        max_steps: usize,
    ) -> Result<usize, ScheduleError> {
        let mut steps = 0;
        while let Some(top) = self.queue.pop() {
            if steps >= max_steps {
                return Err(ScheduleError::StepBound(steps));
            }
            self.now = top.at;
            self.dispatch(top.event, adversary)?;
            steps += 1;
        }
        Ok(steps)
    }

    fn dispatch<A: Adversary<P::Message>>(
        &mut self,
        event: Event<P::Message>,
        adversary: &mut A,
    ) -> Result<(), ScheduleError> {
        let (target, mut outbox, timers) = match event {
            Event::Deliver(env) => {
                let to = env.to;
                let node = self
                    .nodes
                    .get_mut(&to)
                    .ok_or(ScheduleError::UnknownNode(to))?;
                let mut outbox = Vec::new();
                let mut timers = Vec::new();
                {
                    let mut ctx = StepCtx {
                        now: self.now,
                        outbox: &mut outbox,
                        timers: &mut timers,
                        rng: &mut self.rng,
                    };
                    node.on_receive(env, &mut ctx);
                }
                (to, outbox, timers)
            }
            Event::Tick(id) => {
                let node = self
                    .nodes
                    .get_mut(&id)
                    .ok_or(ScheduleError::UnknownNode(id))?;
                let mut outbox = Vec::new();
                let mut timers = Vec::new();
                {
                    let mut ctx = StepCtx {
                        now: self.now,
                        outbox: &mut outbox,
                        timers: &mut timers,
                        rng: &mut self.rng,
                    };
                    node.on_tick(&mut ctx);
                }
                (id, outbox, timers)
            }
            Event::Timer { node: id, id: tid } => {
                let node = self
                    .nodes
                    .get_mut(&id)
                    .ok_or(ScheduleError::UnknownNode(id))?;
                let mut outbox = Vec::new();
                let mut timers = Vec::new();
                {
                    let mut ctx = StepCtx {
                        now: self.now,
                        outbox: &mut outbox,
                        timers: &mut timers,
                        rng: &mut self.rng,
                    };
                    node.on_timer(tid, &mut ctx);
                }
                (id, outbox, timers)
            }
        };

        for env in outbox.drain(..) {
            let deliveries = adversary.intercept(env, self.now, &mut self.rng);
            for (at, env) in deliveries {
                if at < self.now {
                    return Err(ScheduleError::PastDelivery {
                        past: at,
                        now: self.now,
                    });
                }
                if !self.nodes.contains_key(&env.to) {
                    return Err(ScheduleError::UnknownNode(env.to));
                }
                self.push(at, Event::Deliver(env));
            }
        }

        for (delay, tid) in timers {
            self.push(
                self.now + delay,
                Event::Timer {
                    node: target,
                    id: tid,
                },
            );
        }

        Ok(())
    }
}

// =====================================================================
// Tests
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    enum Msg {
        Ping,
        Pong,
    }

    struct PingPong {
        id: NodeId,
        peer: NodeId,
        sent_ping: bool,
        received_ping: bool,
        received_pong: bool,
    }

    impl Process for PingPong {
        type Message = Msg;
        fn id(&self) -> NodeId {
            self.id
        }
        fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
            if !self.sent_ping && self.id.0 == 0 {
                ctx.send(self.id, self.peer, Msg::Ping);
                self.sent_ping = true;
            }
        }
        fn on_receive(
            &mut self,
            env: Envelope<Msg>,
            ctx: &mut StepCtx<'_, Msg>,
        ) {
            match env.msg {
                Msg::Ping => {
                    self.received_ping = true;
                    ctx.send(self.id, env.from, Msg::Pong);
                }
                Msg::Pong => {
                    self.received_pong = true;
                }
            }
        }
    }

    #[test]
    fn ping_pong_terminates_with_noop_adversary() {
        let mut sched = Scheduler::<PingPong>::new(0);
        sched
            .add_node(PingPong {
                id: NodeId(0),
                peer: NodeId(1),
                sent_ping: false,
                received_ping: false,
                received_pong: false,
            })
            .unwrap();
        sched
            .add_node(PingPong {
                id: NodeId(1),
                peer: NodeId(0),
                sent_ping: false,
                received_ping: false,
                received_pong: false,
            })
            .unwrap();
        let mut adv = NoOpAdversary;
        let steps = sched.run(&mut adv, 100).unwrap();
        assert!(steps > 0);
        let n0 = sched.node(NodeId(0)).unwrap();
        let n1 = sched.node(NodeId(1)).unwrap();
        assert!(n0.sent_ping);
        assert!(n1.received_ping);
        assert!(n0.received_pong);
    }

    #[test]
    fn duplicate_node_is_rejected() {
        let mut sched = Scheduler::<PingPong>::new(0);
        sched
            .add_node(PingPong {
                id: NodeId(0),
                peer: NodeId(1),
                sent_ping: false,
                received_ping: false,
                received_pong: false,
            })
            .unwrap();
        let err = sched
            .add_node(PingPong {
                id: NodeId(0),
                peer: NodeId(1),
                sent_ping: false,
                received_ping: false,
                received_pong: false,
            })
            .unwrap_err();
        matches!(err, ScheduleError::DuplicateNode(_));
    }

    struct DropAll;
    impl<M: Clone + Debug> Adversary<M> for DropAll {
        fn intercept(
            &mut self,
            _env: Envelope<M>,
            _now: Time,
            _rng: &mut SimRng,
        ) -> Vec<(Time, Envelope<M>)> {
            Vec::new()
        }
    }

    #[test]
    fn drop_all_adversary_silences_network() {
        let mut sched = Scheduler::<PingPong>::new(0);
        sched
            .add_node(PingPong {
                id: NodeId(0),
                peer: NodeId(1),
                sent_ping: false,
                received_ping: false,
                received_pong: false,
            })
            .unwrap();
        sched
            .add_node(PingPong {
                id: NodeId(1),
                peer: NodeId(0),
                sent_ping: false,
                received_ping: false,
                received_pong: false,
            })
            .unwrap();
        let mut adv = DropAll;
        sched.run(&mut adv, 100).unwrap();
        let n1 = sched.node(NodeId(1)).unwrap();
        assert!(!n1.received_ping);
    }
}
