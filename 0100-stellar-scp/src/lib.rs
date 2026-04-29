//! Module 0100: Stellar Consensus Protocol federated quorums.
//!
//! Each node has a list of quorum slices. A quorum is a set
//! `U` such that every member of `U` has a slice contained in
//! `U`. Two quorums must intersect for SCP safety.

#![warn(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

/// Node identifier.
pub type NodeId = u64;

/// A node's quorum-slice configuration: each entry is one
/// possible slice (a set of nodes whose agreement the owner
/// accepts).
#[derive(Clone, Debug, Default)]
pub struct QuorumSlice {
    /// Possible slices for this node.
    pub slices: Vec<BTreeSet<NodeId>>,
}

/// `FBA` configuration: maps node id to its slices.
#[derive(Clone, Debug, Default)]
pub struct FbaConfig {
    /// Per-node slice configuration.
    pub nodes: BTreeMap<NodeId, QuorumSlice>,
}

impl FbaConfig {
    /// Empty config.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a node with its slice list.
    pub fn register(&mut self, id: NodeId, slices: Vec<BTreeSet<NodeId>>) {
        self.nodes.insert(id, QuorumSlice { slices });
    }

    /// Check whether `u` is a quorum: every member of `u` has
    /// at least one slice contained in `u`.
    #[must_use]
    pub fn is_quorum(&self, u: &BTreeSet<NodeId>) -> bool {
        u.iter().all(|m| {
            self.nodes
                .get(m)
                .is_some_and(|qs| qs.slices.iter().any(|s| s.is_subset(u)))
        })
    }

    /// Check whether two given quorums intersect.
    #[must_use]
    pub fn quorums_intersect(
        a: &BTreeSet<NodeId>,
        b: &BTreeSet<NodeId>,
    ) -> bool {
        !a.is_disjoint(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(items: &[NodeId]) -> BTreeSet<NodeId> {
        items.iter().copied().collect()
    }

    #[test]
    fn singleton_slice_is_quorum() {
        let mut cfg = FbaConfig::new();
        cfg.register(1, vec![s(&[1])]);
        assert!(cfg.is_quorum(&s(&[1])));
    }

    #[test]
    fn two_node_mutual_slice_is_quorum() {
        let mut cfg = FbaConfig::new();
        cfg.register(1, vec![s(&[1, 2])]);
        cfg.register(2, vec![s(&[1, 2])]);
        assert!(cfg.is_quorum(&s(&[1, 2])));
    }

    #[test]
    fn missing_member_breaks_quorum() {
        let mut cfg = FbaConfig::new();
        cfg.register(1, vec![s(&[1, 2])]);
        cfg.register(2, vec![s(&[1, 2])]);
        // Drop node 2: only {1} alone, slice {1, 2} not subset.
        assert!(!cfg.is_quorum(&s(&[1])));
    }

    #[test]
    fn quorums_intersect_when_sharing_node() {
        let a = s(&[1, 2, 3]);
        let b = s(&[3, 4, 5]);
        assert!(FbaConfig::quorums_intersect(&a, &b));
    }

    #[test]
    fn disjoint_quorums_do_not_intersect() {
        let a = s(&[1, 2, 3]);
        let b = s(&[4, 5, 6]);
        assert!(!FbaConfig::quorums_intersect(&a, &b));
    }
}
