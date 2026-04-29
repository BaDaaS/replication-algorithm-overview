//! Module 0021: a placeholder crate for VR Revisited.
//!
//! The 2012 paper's protocol is structurally the same as VR
//! 1988 (module 0020); the differences are exposition,
//! persistence-field discipline, and a reconfiguration
//! operation. This crate exposes only the type-level
//! representation of those additions; the full simulation
//! lives in module 0020.

#![warn(missing_docs)]

use sim::NodeId;

/// VR Revisited status.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VrStatus {
    /// Normal operation.
    Normal,
    /// View change in progress.
    ViewChange,
    /// Replica recovering after a crash.
    Recovering,
    /// Reconfiguration in progress, with the proposed new set.
    Reconfiguring(Vec<NodeId>),
}

/// Persistence record: the fields that must be on stable
/// storage before responding to peers (per Liskov-Cowling 2012,
/// Section 4.1).
#[derive(Clone, Debug, Default)]
pub struct PersistenceRecord {
    /// Current view number.
    pub view_num: u32,
    /// Sequence number of latest accepted op.
    pub op_num: u32,
    /// Latest committed op number.
    pub commit_num: u32,
    /// Log of (view, op) tuples.
    pub log: Vec<(u32, u32)>,
    /// Last view-change vote (None if none).
    pub last_dvc_vote: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persistence_default() {
        let p = PersistenceRecord::default();
        assert_eq!(p.view_num, 0);
        assert_eq!(p.op_num, 0);
        assert_eq!(p.commit_num, 0);
        assert!(p.log.is_empty());
        assert!(p.last_dvc_vote.is_none());
    }

    #[test]
    fn status_distinct() {
        let n = VrStatus::Normal;
        let r = VrStatus::Reconfiguring(vec![NodeId(0), NodeId(1)]);
        assert_ne!(n, r);
    }
}
