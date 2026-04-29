//! Module 0012: a heartbeat-based failure detector.

#![warn(missing_docs)]

use std::collections::BTreeMap;

use sim::{Envelope, NodeId, Process, StepCtx, Time};

/// Wire message type.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Periodic heartbeat: "I, sender, am alive at time `at`".
    Heartbeat {
        /// Sender's local time at send.
        at: Time,
    },
}

/// A heartbeat-based detector node.
pub struct HeartbeatNode {
    id: NodeId,
    peers: Vec<NodeId>,
    /// Detection timeout: suspect `j` if no heartbeat in
    /// `timeout` ticks.
    pub timeout: Time,
    /// Heartbeat send interval.
    pub interval: Time,
    /// Last received heartbeat from each peer.
    last_seen: BTreeMap<NodeId, Time>,
    /// Tick counter for heartbeats.
    next_beat: Time,
    /// Whether this node has crashed (used to silence outgoing
    /// heartbeats in tests; the simulator does not natively
    /// model crashes).
    pub crashed: bool,
    /// Currently suspected peers.
    pub suspected: std::collections::BTreeSet<NodeId>,
}

impl HeartbeatNode {
    /// Build a heartbeat detector node.
    pub fn new(
        id: NodeId,
        peers: Vec<NodeId>,
        timeout: Time,
        interval: Time,
    ) -> Self {
        Self {
            id,
            peers,
            timeout,
            interval,
            last_seen: BTreeMap::new(),
            next_beat: 0,
            crashed: false,
            suspected: std::collections::BTreeSet::new(),
        }
    }

    fn refresh_suspects(&mut self, now: Time) {
        self.suspected.clear();
        for &p in &self.peers {
            if p == self.id {
                continue;
            }
            let last = self.last_seen.get(&p).copied().unwrap_or(0);
            if now > last + self.timeout {
                self.suspected.insert(p);
            }
        }
    }
}

impl Process for HeartbeatNode {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        self.refresh_suspects(ctx.now);
        if !self.crashed && ctx.now >= self.next_beat {
            self.next_beat = ctx.now + self.interval;
            for &peer in &self.peers {
                if peer == self.id {
                    continue;
                }
                ctx.send(self.id, peer, Msg::Heartbeat { at: ctx.now });
            }
        }
        // Reschedule a tick at the next interesting time.
        let next = self.next_beat.max(ctx.now + 1);
        if next > ctx.now {
            ctx.schedule_timer(next - ctx.now, 0);
        }
    }

    fn on_timer(&mut self, _id: u64, ctx: &mut StepCtx<'_, Msg>) {
        self.on_tick(ctx);
    }

    fn on_receive(&mut self, env: Envelope<Msg>, _ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::Heartbeat { at: _ } => {
                self.last_seen.insert(env.from, env.sent_at);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sim::{NoOpAdversary, Scheduler};

    use super::*;

    fn build(n: u32) -> Scheduler<HeartbeatNode> {
        let ids: Vec<NodeId> = (0..n).map(NodeId).collect();
        let mut sched = Scheduler::<HeartbeatNode>::new(0);
        for &id in &ids {
            sched
                .add_node(HeartbeatNode::new(id, ids.clone(), 5, 1))
                .unwrap();
        }
        sched
    }

    #[test]
    fn no_one_suspected_under_no_op() {
        let mut sched = build(3);
        let mut adv = NoOpAdversary;
        // The detector reschedules itself forever; we run until
        // the step bound, which triggers a `StepBound` error. We
        // intentionally ignore it: the protocol-level state is
        // what we test.
        let _ = sched.run(&mut adv, 200);
        for id in [NodeId(0), NodeId(1), NodeId(2)] {
            let n = sched.node(id).unwrap();
            assert!(
                n.suspected.is_empty(),
                "node {id} suspects {:?}",
                n.suspected
            );
        }
    }

    #[test]
    fn crashed_node_is_eventually_suspected() {
        let mut sched = build(3);
        // Crash node 1 at time 0 (before it can ever beat).
        sched.node_mut(NodeId(1)).unwrap().crashed = true;
        let mut adv = NoOpAdversary;
        // The detector reschedules itself forever; we run until
        // the step bound, which triggers a `StepBound` error. We
        // intentionally ignore it: the protocol-level state is
        // what we test.
        let _ = sched.run(&mut adv, 200);
        // Node 0 should suspect node 1 by the end of the run.
        let n0 = sched.node(NodeId(0)).unwrap();
        assert!(n0.suspected.contains(&NodeId(1)));
    }
}
