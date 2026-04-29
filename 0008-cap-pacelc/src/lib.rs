//! Module 0008: a two-replica register illustrating CAP.
//!
//! The same wire protocol is exposed in two variants. The `Ap`
//! register answers reads from local state immediately (gives up
//! consistency under partition); the `Cp` register withholds the
//! read response until quorum acknowledgement returns (gives up
//! availability under partition). A `OnePartitionAdversary` cuts
//! the wire from `partition_at` to `heal_at`.

#![warn(missing_docs)]

use sim::{Adversary, Envelope, NodeId, Process, SimRng, StepCtx, Time};

/// Wire message type.
#[derive(Clone, Debug)]
pub enum Msg {
    /// Replicate a write to the peer.
    Write {
        /// Sequence number to break ties.
        seq: u64,
        /// New value.
        value: u64,
    },
    /// Acknowledge a write.
    Ack {
        /// Sequence number being acked.
        seq: u64,
    },
}

/// Variant: AP register answers reads from local state.
#[derive(Clone, Debug)]
pub struct ApRegister {
    /// This node's identifier.
    pub id: NodeId,
    /// Peer node identifier.
    pub peer: NodeId,
    /// Locally observed value.
    pub local: u64,
    /// Sequence number for outgoing writes.
    pub seq: u64,
    /// Last read result, if a read has been issued.
    pub last_read: Option<u64>,
    /// At what time the test should issue a read on this node.
    pub read_at: Option<Time>,
    /// Initial write to issue (only the writing replica).
    pub initial_write: Option<u64>,
    initial_write_done: bool,
}

impl ApRegister {
    /// Build an AP register node.
    pub fn new(
        id: NodeId,
        peer: NodeId,
        initial_write: Option<u64>,
        read_at: Option<Time>,
    ) -> Self {
        Self {
            id,
            peer,
            local: 0,
            seq: 0,
            last_read: None,
            read_at,
            initial_write,
            initial_write_done: false,
        }
    }
}

impl Process for ApRegister {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if !self.initial_write_done
            && let Some(v) = self.initial_write
        {
            self.initial_write_done = true;
            self.local = v;
            self.seq += 1;
            ctx.send(
                self.id,
                self.peer,
                Msg::Write {
                    seq: self.seq,
                    value: v,
                },
            );
        }
        if let Some(t) = self.read_at
            && t > ctx.now
        {
            // Schedule a timer for the read trigger.
            ctx.schedule_timer(t - ctx.now, 1);
        } else if let Some(t) = self.read_at
            && ctx.now >= t
            && self.last_read.is_none()
        {
            self.last_read = Some(self.local);
        }
    }

    fn on_timer(&mut self, _id: u64, ctx: &mut StepCtx<'_, Msg>) {
        if let Some(t) = self.read_at
            && ctx.now >= t
            && self.last_read.is_none()
        {
            self.last_read = Some(self.local);
        }
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::Write { seq, value } => {
                self.local = value;
                self.seq = self.seq.max(seq);
                ctx.send(self.id, env.from, Msg::Ack { seq });
            }
            Msg::Ack { .. } => {}
        }
    }
}

/// Variant: CP register withholds read until peer ack returns.
#[derive(Clone, Debug)]
pub struct CpRegister {
    /// This node's identifier.
    pub id: NodeId,
    /// Peer node identifier.
    pub peer: NodeId,
    /// Locally observed value.
    pub local: u64,
    /// Sequence number for outgoing writes.
    pub seq: u64,
    /// Last read result, if available.
    pub last_read: Option<u64>,
    /// At what time the test should issue a read.
    pub read_at: Option<Time>,
    /// Read is pending until we receive a fresh ack.
    pub pending_read: bool,
    /// Whether a "probe" message is in-flight.
    pub probe_seq: u64,
    /// Initial write to issue.
    pub initial_write: Option<u64>,
    initial_write_done: bool,
}

impl CpRegister {
    /// Build a CP register node.
    pub fn new(
        id: NodeId,
        peer: NodeId,
        initial_write: Option<u64>,
        read_at: Option<Time>,
    ) -> Self {
        Self {
            id,
            peer,
            local: 0,
            seq: 0,
            last_read: None,
            read_at,
            pending_read: false,
            probe_seq: 0,
            initial_write,
            initial_write_done: false,
        }
    }
}

impl Process for CpRegister {
    type Message = Msg;

    fn id(&self) -> NodeId {
        self.id
    }

    fn on_tick(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if !self.initial_write_done
            && let Some(v) = self.initial_write
        {
            self.initial_write_done = true;
            self.local = v;
            self.seq += 1;
            ctx.send(
                self.id,
                self.peer,
                Msg::Write {
                    seq: self.seq,
                    value: v,
                },
            );
        }
        if let Some(t) = self.read_at
            && t > ctx.now
        {
            ctx.schedule_timer(t - ctx.now, 1);
        }
        self.try_issue_read(ctx);
    }

    fn on_timer(&mut self, _id: u64, ctx: &mut StepCtx<'_, Msg>) {
        self.try_issue_read(ctx);
    }

    fn on_receive(&mut self, env: Envelope<Msg>, ctx: &mut StepCtx<'_, Msg>) {
        match env.msg {
            Msg::Write { seq, value } => {
                self.local = self.local.max(value);
                self.seq = self.seq.max(seq);
                ctx.send(self.id, env.from, Msg::Ack { seq });
            }
            Msg::Ack { .. } => {
                if self.pending_read {
                    self.pending_read = false;
                    self.last_read = Some(self.local);
                }
            }
        }
    }
}

impl CpRegister {
    fn try_issue_read(&mut self, ctx: &mut StepCtx<'_, Msg>) {
        if let Some(t) = self.read_at
            && ctx.now >= t
            && self.last_read.is_none()
            && !self.pending_read
        {
            self.pending_read = true;
            self.probe_seq += 1;
            ctx.send(
                self.id,
                self.peer,
                Msg::Write {
                    seq: self.probe_seq,
                    value: self.local,
                },
            );
        }
    }
}

/// Adversary that drops all messages from `partition_at` to
/// `heal_at`, regardless of direction.
pub struct OnePartitionAdversary {
    /// When the partition starts.
    pub partition_at: Time,
    /// When the partition heals.
    pub heal_at: Time,
}

impl<M: Clone + std::fmt::Debug> Adversary<M> for OnePartitionAdversary {
    fn intercept(
        &mut self,
        env: Envelope<M>,
        now: Time,
        _rng: &mut SimRng,
    ) -> Vec<(Time, Envelope<M>)> {
        if now >= self.partition_at && now < self.heal_at {
            return Vec::new();
        }
        vec![(now + 1, env)]
    }
}

#[cfg(test)]
mod tests {
    use sim::Scheduler;

    use super::*;

    #[test]
    fn ap_returns_stale_under_partition() {
        // Node 0 writes 42 at time 1; partition kicks in at time
        // 2 and heals at time 100. Node 1 reads at time 5.
        // AP register answers from local state: returns 0 (stale).
        let mut sched = Scheduler::<ApRegister>::new(0);
        sched
            .add_node(ApRegister::new(NodeId(0), NodeId(1), Some(42), None))
            .unwrap();
        sched
            .add_node(ApRegister::new(NodeId(1), NodeId(0), None, Some(5)))
            .unwrap();
        let mut adv = OnePartitionAdversary {
            partition_at: 0,
            heal_at: 100,
        };
        sched.run(&mut adv, 200).unwrap();
        let r1 = sched.node(NodeId(1)).unwrap();
        assert_eq!(r1.last_read, Some(0));
    }

    #[test]
    fn cp_blocks_read_under_partition() {
        // CP register issues a probe and waits for an ack. Under
        // partition the probe is dropped; without retry, the read
        // never resolves. This is the operational meaning of "CP
        // gives up availability under partition".
        let mut sched = Scheduler::<CpRegister>::new(0);
        sched
            .add_node(CpRegister::new(NodeId(0), NodeId(1), Some(42), None))
            .unwrap();
        sched
            .add_node(CpRegister::new(NodeId(1), NodeId(0), None, Some(5)))
            .unwrap();
        let mut adv = OnePartitionAdversary {
            partition_at: 0,
            heal_at: u64::MAX, // partition never heals
        };
        sched.run(&mut adv, 200).unwrap();
        let r1 = sched.node(NodeId(1)).unwrap();
        // The read does not complete: pending_read == true,
        // last_read == None.
        assert!(r1.last_read.is_none());
        assert!(r1.pending_read);
    }

    #[test]
    fn cp_returns_correct_under_no_partition() {
        let mut sched = Scheduler::<CpRegister>::new(0);
        sched
            .add_node(CpRegister::new(NodeId(0), NodeId(1), Some(42), None))
            .unwrap();
        sched
            .add_node(CpRegister::new(NodeId(1), NodeId(0), None, Some(10)))
            .unwrap();
        let mut adv = OnePartitionAdversary {
            partition_at: 1000,
            heal_at: 1001,
        };
        sched.run(&mut adv, 200).unwrap();
        let r1 = sched.node(NodeId(1)).unwrap();
        assert_eq!(r1.last_read, Some(42));
    }
}
