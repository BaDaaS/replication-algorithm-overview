//! Module 0002: failure-mode adversaries for the course simulator.
//!
//! Three reusable adversaries layered over the simulator:
//!
//! - [`CrashStopAdversary`]: each designated node crashes at a
//!   specified time; subsequent outgoing messages are dropped.
//! - [`OmissionAdversary`]: each designated node loses a fraction
//!   of its outgoing messages.
//! - [`EquivocatingAdversary`]: each designated node may emit
//!   different message values to different recipients in the
//!   *same* logical broadcast. The adversary is parameterised by
//!   a swap function, so it works on any message type.

#![warn(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

use rand::RngExt;
use sim::{Adversary, Envelope, NodeId, SimRng, Time};

// =====================================================================
// Crash-stop
// =====================================================================

/// Crash-stop adversary. For each entry `(node, t_c)` in
/// `crash_at`, drop every outgoing message from `node` whose
/// `now >= t_c`.
///
/// Wraps an inner adversary that handles the surviving messages
/// (timing).
pub struct CrashStopAdversary<A> {
    /// When each node crashes. Nodes not in the map never crash.
    pub crash_at: BTreeMap<NodeId, Time>,
    /// Inner adversary applied to surviving messages.
    pub inner: A,
}

impl<A> CrashStopAdversary<A> {
    /// Build a crash-stop adversary on top of `inner`.
    pub fn new(crash_at: BTreeMap<NodeId, Time>, inner: A) -> Self {
        Self { crash_at, inner }
    }
}

impl<A, M> Adversary<M> for CrashStopAdversary<A>
where
    A: Adversary<M>,
    M: Clone + Debug,
{
    fn intercept(
        &mut self,
        env: Envelope<M>,
        now: Time,
        rng: &mut SimRng,
    ) -> Vec<(Time, Envelope<M>)> {
        if let Some(&t_c) = self.crash_at.get(&env.from)
            && now >= t_c
        {
            return Vec::new();
        }
        self.inner.intercept(env, now, rng)
    }
}

// =====================================================================
// Omission
// =====================================================================

/// Omission adversary. Drops outgoing messages with per-node
/// probability `drop_prob`.
pub struct OmissionAdversary<A> {
    /// Per-node drop probability in `[0, 1]`. Missing nodes are
    /// treated as `0.0`.
    pub drop_prob: BTreeMap<NodeId, f64>,
    /// Inner adversary applied to surviving messages.
    pub inner: A,
}

impl<A> OmissionAdversary<A> {
    /// Build an omission adversary.
    pub fn new(drop_prob: BTreeMap<NodeId, f64>, inner: A) -> Self {
        Self { drop_prob, inner }
    }
}

impl<A, M> Adversary<M> for OmissionAdversary<A>
where
    A: Adversary<M>,
    M: Clone + Debug,
{
    fn intercept(
        &mut self,
        env: Envelope<M>,
        now: Time,
        rng: &mut SimRng,
    ) -> Vec<(Time, Envelope<M>)> {
        let p = self.drop_prob.get(&env.from).copied().unwrap_or(0.0);
        if p > 0.0 && rng.inner().random::<f64>() < p {
            return Vec::new();
        }
        self.inner.intercept(env, now, rng)
    }
}

// =====================================================================
// Equivocation
// =====================================================================

/// Equivocating adversary. For each Byzantine sender, when an
/// outgoing envelope's recipient is in `swap_to`, the adversary
/// replaces the message body with the result of `swap_fn(env)`.
/// The swap function is responsible for producing a valid alternate
/// message of type `M`.
pub struct EquivocatingAdversary<A, M, F>
where
    F: FnMut(&Envelope<M>) -> M,
    M: Clone + Debug,
{
    /// Byzantine sender identities.
    pub byzantine: BTreeSet<NodeId>,
    /// Recipients for which the message is mutated.
    pub swap_to: BTreeSet<NodeId>,
    /// Mutation function.
    pub swap_fn: F,
    /// Inner adversary.
    pub inner: A,
    _phantom: std::marker::PhantomData<M>,
}

impl<A, M, F> EquivocatingAdversary<A, M, F>
where
    F: FnMut(&Envelope<M>) -> M,
    M: Clone + Debug,
{
    /// Build an equivocating adversary.
    pub fn new(
        byzantine: BTreeSet<NodeId>,
        swap_to: BTreeSet<NodeId>,
        swap_fn: F,
        inner: A,
    ) -> Self {
        Self {
            byzantine,
            swap_to,
            swap_fn,
            inner,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<A, M, F> Adversary<M> for EquivocatingAdversary<A, M, F>
where
    A: Adversary<M>,
    M: Clone + Debug,
    F: FnMut(&Envelope<M>) -> M,
{
    fn intercept(
        &mut self,
        env: Envelope<M>,
        now: Time,
        rng: &mut SimRng,
    ) -> Vec<(Time, Envelope<M>)> {
        if self.byzantine.contains(&env.from) && self.swap_to.contains(&env.to)
        {
            let new_msg = (self.swap_fn)(&env);
            let mutated = Envelope {
                from: env.from,
                to: env.to,
                sent_at: env.sent_at,
                msg: new_msg,
            };
            return self.inner.intercept(mutated, now, rng);
        }
        self.inner.intercept(env, now, rng)
    }
}

// =====================================================================
// Tests
// =====================================================================

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Process, Scheduler, StepCtx};

    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Counter(u32);

    struct Beacon {
        id: NodeId,
        peer: NodeId,
        sent: u32,
        received: u32,
    }

    impl Process for Beacon {
        type Message = Counter;
        fn id(&self) -> NodeId {
            self.id
        }
        fn on_tick(&mut self, ctx: &mut StepCtx<'_, Counter>) {
            // Send 10 beacons on tick.
            if self.id.0 == 0 && self.sent < 10 {
                for k in 0..10 {
                    ctx.send(self.id, self.peer, Counter(k));
                }
                self.sent = 10;
            }
        }
        fn on_receive(
            &mut self,
            _env: Envelope<Counter>,
            _ctx: &mut StepCtx<'_, Counter>,
        ) {
            self.received += 1;
        }
    }

    fn pair() -> Scheduler<Beacon> {
        let mut sched = Scheduler::<Beacon>::new(0);
        sched
            .add_node(Beacon {
                id: NodeId(0),
                peer: NodeId(1),
                sent: 0,
                received: 0,
            })
            .unwrap();
        sched
            .add_node(Beacon {
                id: NodeId(1),
                peer: NodeId(0),
                sent: 0,
                received: 0,
            })
            .unwrap();
        sched
    }

    #[test]
    fn crash_at_zero_silences_sender() {
        let mut sched = pair();
        let mut crash_at = BTreeMap::new();
        crash_at.insert(NodeId(0), 0);
        let mut adv = CrashStopAdversary::new(crash_at, NoOpAdversary);
        sched.run(&mut adv, 1000).unwrap();
        let r1 = sched.node(NodeId(1)).unwrap();
        assert_eq!(r1.received, 0);
    }

    #[test]
    fn crash_after_partial_send_preserves_initial_messages() {
        let mut sched = pair();
        // Sender starts at time 1; we crash at time 1, so all
        // its first-tick messages are emitted before the crash
        // takes effect. With an interleaved scheduler the result
        // depends on tick ordering; the test just asserts that
        // either all 10 went through (sender was alive at the
        // tick) or zero did (crash applied at tick), but never
        // a non-zero-non-ten count.
        let mut crash_at = BTreeMap::new();
        crash_at.insert(NodeId(0), 2);
        let mut adv = CrashStopAdversary::new(crash_at, NoOpAdversary);
        sched.run(&mut adv, 1000).unwrap();
        let r1 = sched.node(NodeId(1)).unwrap();
        assert_eq!(r1.received, 10);
    }

    #[test]
    fn omission_with_50_percent_drops_about_half() {
        let mut sched = pair();
        let mut probs = BTreeMap::new();
        probs.insert(NodeId(0), 0.5);
        let mut adv = OmissionAdversary::new(probs, NoOpAdversary);
        sched.run(&mut adv, 1000).unwrap();
        let r1 = sched.node(NodeId(1)).unwrap();
        // Probabilistic; with seed 0, we expect 3 to 7.
        assert!(
            (3..=7).contains(&r1.received),
            "received {} of 10",
            r1.received
        );
    }
}
