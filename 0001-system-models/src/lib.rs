//! Module 0001: timing-model adversaries for the course simulator.
//!
//! Each adversary realises one of the three DLS timing models:
//! synchronous, asynchronous, and partially synchronous (Variant B
//! with a known `T_GST`). Subsequent modules reuse these adversaries
//! to instantiate concrete failure-and-timing pairs.

#![warn(missing_docs)]

use std::fmt::Debug;

use rand::RngExt;
use sim::{Adversary, Envelope, SimRng, Time};

// =====================================================================
// Synchronous adversary
// =====================================================================

/// Synchronous network: every message is delivered exactly `delay`
/// ticks after sending.
pub struct SynchronousAdversary {
    /// Fixed delivery delay `D`.
    pub delay: Time,
}

impl SynchronousAdversary {
    /// Build a synchronous adversary with delay `D`.
    #[must_use]
    pub fn new(delay: Time) -> Self {
        Self { delay }
    }
}

impl<M: Clone + Debug> Adversary<M> for SynchronousAdversary {
    fn intercept(
        &mut self,
        env: Envelope<M>,
        now: Time,
        _rng: &mut SimRng,
    ) -> Vec<(Time, Envelope<M>)> {
        vec![(now + self.delay, env)]
    }
}

// =====================================================================
// Asynchronous adversary
// =====================================================================

/// Asynchronous network: every message is delivered after a
/// uniformly random delay in `[1, max_delay]`. Set `max_delay`
/// arbitrarily large to approximate pure asynchrony.
pub struct AsynchronousAdversary {
    /// Upper bound on per-message delay. Lower bound is always 1.
    pub max_delay: Time,
}

impl AsynchronousAdversary {
    /// Build an asynchronous adversary with `max_delay` upper
    /// bound.
    #[must_use]
    pub fn new(max_delay: Time) -> Self {
        Self { max_delay }
    }
}

impl<M: Clone + Debug> Adversary<M> for AsynchronousAdversary {
    fn intercept(
        &mut self,
        env: Envelope<M>,
        now: Time,
        rng: &mut SimRng,
    ) -> Vec<(Time, Envelope<M>)> {
        let delay = rng.inner().random_range(1..=self.max_delay.max(1));
        vec![(now + delay, env)]
    }
}

// =====================================================================
// Partially synchronous adversary (DLS Variant B, T_GST known)
// =====================================================================

/// Partially synchronous network. Before `gst`, deliveries are
/// asynchronous up to `async_max`; from `gst` onwards, deliveries
/// take exactly `sync_delay` ticks.
pub struct PartiallySynchronousAdversary {
    /// Global stabilisation time.
    pub gst: Time,
    /// Synchronous delivery delay after GST.
    pub sync_delay: Time,
    /// Asynchronous upper bound before GST.
    pub async_max: Time,
}

impl PartiallySynchronousAdversary {
    /// Build a partially synchronous adversary.
    #[must_use]
    pub fn new(gst: Time, sync_delay: Time, async_max: Time) -> Self {
        Self {
            gst,
            sync_delay,
            async_max,
        }
    }
}

impl<M: Clone + Debug> Adversary<M> for PartiallySynchronousAdversary {
    fn intercept(
        &mut self,
        env: Envelope<M>,
        now: Time,
        rng: &mut SimRng,
    ) -> Vec<(Time, Envelope<M>)> {
        let delay = if now < self.gst {
            rng.inner().random_range(1..=self.async_max.max(1))
        } else {
            self.sync_delay
        };
        vec![(now + delay, env)]
    }
}

// =====================================================================
// Tests
// =====================================================================

#[cfg(test)]
mod tests {
    use sim::{NodeId, Process, Scheduler, StepCtx};

    use super::*;

    #[derive(Clone, Debug)]
    enum Msg {
        Hello,
    }

    struct Listener {
        id: NodeId,
        peer: NodeId,
        send_at: Time,
        sent: bool,
        received_at: Option<Time>,
    }

    impl Process for Listener {
        type Message = Msg;
        fn id(&self) -> NodeId {
            self.id
        }
        fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
            if !self.sent && ctx.now >= self.send_at && self.id.0 == 0 {
                ctx.send(self.id, self.peer, Msg::Hello);
                self.sent = true;
            }
        }
        fn on_receive(
            &mut self,
            _env: Envelope<Msg>,
            ctx: &mut StepCtx<'_, Msg>,
        ) {
            self.received_at = Some(ctx.now);
        }
    }

    fn listener_pair(send_at: Time) -> Scheduler<Listener> {
        let mut sched = Scheduler::<Listener>::new(0);
        sched
            .add_node(Listener {
                id: NodeId(0),
                peer: NodeId(1),
                send_at,
                sent: false,
                received_at: None,
            })
            .unwrap();
        sched
            .add_node(Listener {
                id: NodeId(1),
                peer: NodeId(0),
                send_at,
                sent: false,
                received_at: None,
            })
            .unwrap();
        // Force ticks at every time step up to a bound by adding
        // self-pings is unnecessary here: the simulator only ticks
        // a node once at time 1. We instead set send_at = 0 so the
        // initial tick triggers the send.
        sched
    }

    #[test]
    fn synchronous_delivers_within_bound() {
        let mut sched = listener_pair(0);
        let mut adv = SynchronousAdversary::new(5);
        sched.run(&mut adv, 100).unwrap();
        let r1 = sched.node(NodeId(1)).unwrap();
        // The send happens at time 1 (first tick), delivery 5 later.
        assert_eq!(r1.received_at, Some(6));
    }

    #[test]
    fn asynchronous_delivers_eventually() {
        let mut sched = listener_pair(0);
        let mut adv = AsynchronousAdversary::new(50);
        sched.run(&mut adv, 1000).unwrap();
        let r1 = sched.node(NodeId(1)).unwrap();
        let t = r1.received_at.expect("delivery");
        assert!((2..=51).contains(&t));
    }

    #[test]
    fn partial_synchrony_stabilises_at_gst() {
        let mut sched = listener_pair(0);
        let mut adv = PartiallySynchronousAdversary::new(20, 1, 200);
        sched.run(&mut adv, 1000).unwrap();
        let r1 = sched.node(NodeId(1)).unwrap();
        // Whatever the pre-GST schedule, delivery happens.
        assert!(r1.received_at.is_some());
    }
}
