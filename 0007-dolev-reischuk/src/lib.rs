//! Module 0007: a message-counting adversary wrapper.
//!
//! Wraps any inner adversary and records the total number of
//! envelopes intercepted. Reusable across all later modules to
//! check empirical message complexity against the
//! Dolev-Reischuk bound.

#![warn(missing_docs)]

use std::fmt::Debug;

use sim::{Adversary, Envelope, SimRng, Time};

/// Wrapper adversary that counts every intercepted envelope.
pub struct CountingAdversary<A> {
    /// Inner adversary the protocol actually depends on.
    pub inner: A,
    /// Running count of intercepted envelopes.
    pub count: u64,
}

impl<A> CountingAdversary<A> {
    /// Build a counting wrapper.
    pub fn new(inner: A) -> Self {
        Self { inner, count: 0 }
    }
}

impl<A, M> Adversary<M> for CountingAdversary<A>
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
        self.count += 1;
        self.inner.intercept(env, now, rng)
    }
}

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, NodeId, Process, Scheduler, StepCtx};

    use super::*;

    #[derive(Clone, Debug)]
    struct Hello;

    struct AllToAll {
        id: NodeId,
        peers: Vec<NodeId>,
        sent: bool,
    }

    impl Process for AllToAll {
        type Message = Hello;
        fn id(&self) -> NodeId {
            self.id
        }
        fn on_tick(&mut self, ctx: &mut StepCtx<'_, Hello>) {
            if self.sent {
                return;
            }
            self.sent = true;
            for &p in &self.peers {
                if p != self.id {
                    ctx.send(self.id, p, Hello);
                }
            }
        }
        fn on_receive(
            &mut self,
            _env: Envelope<Hello>,
            _ctx: &mut StepCtx<'_, Hello>,
        ) {
        }
    }

    fn run_all_to_all(n: u32) -> u64 {
        let ids: Vec<NodeId> = (0..n).map(NodeId).collect();
        let mut sched = Scheduler::<AllToAll>::new(0);
        for &id in &ids {
            sched
                .add_node(AllToAll {
                    id,
                    peers: ids.clone(),
                    sent: false,
                })
                .unwrap();
        }
        let mut adv = CountingAdversary::new(NoOpAdversary);
        sched.run(&mut adv, 10_000).unwrap();
        adv.count
    }

    #[test]
    fn all_to_all_is_quadratic() {
        // n nodes each send n - 1 messages: total n * (n - 1).
        assert_eq!(run_all_to_all(4), 4 * 3);
        assert_eq!(run_all_to_all(8), 8 * 7);
        assert_eq!(run_all_to_all(16), 16 * 15);
    }
}
