//! Module 0029: configuration types for Stoppable and Vertical
//! Paxos.

#![warn(missing_docs)]

use sim::NodeId;

/// A Vertical-Paxos configuration: a tuple identifying the
/// replica set and the slot at which it takes effect.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Configuration {
    /// Epoch number (monotone across reconfigs).
    pub epoch: u32,
    /// Replicas in this configuration.
    pub replicas: Vec<NodeId>,
    /// Slot at which this configuration takes effect.
    pub start_slot: u32,
}

impl Configuration {
    /// Build a configuration.
    #[must_use]
    pub fn new(epoch: u32, replicas: Vec<NodeId>, start_slot: u32) -> Self {
        Self {
            epoch,
            replicas,
            start_slot,
        }
    }

    /// Quorum size for this configuration.
    #[must_use]
    pub fn quorum(&self) -> usize {
        self.replicas.len() / 2 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new() {
        let cfg =
            Configuration::new(0, vec![NodeId(0), NodeId(1), NodeId(2)], 0);
        assert_eq!(cfg.epoch, 0);
        assert_eq!(cfg.replicas.len(), 3);
        assert_eq!(cfg.quorum(), 2);
    }
}
