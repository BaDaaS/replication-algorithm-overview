//! Module 0031: Flexible Paxos quorum verifiers.

#![warn(missing_docs)]

use std::collections::BTreeSet;

use sim::NodeId;

/// A pair of quorum families satisfying the Flexible Paxos
/// intersection property.
#[derive(Clone, Debug)]
pub struct FlexibleQuorums {
    /// Allowed Phase 1 quorums.
    pub p1: Vec<BTreeSet<NodeId>>,
    /// Allowed Phase 2 quorums.
    pub p2: Vec<BTreeSet<NodeId>>,
}

impl FlexibleQuorums {
    /// Verify the intersection property: every Q1 intersects
    /// every Q2.
    #[must_use]
    pub fn verify_intersection(&self) -> bool {
        for q1 in &self.p1 {
            for q2 in &self.p2 {
                if q1.intersection(q2).next().is_none() {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(ids: &[u32]) -> BTreeSet<NodeId> {
        ids.iter().copied().map(NodeId).collect()
    }

    #[test]
    fn classic_majorities_intersect() {
        // n = 5; all majorities of size 3.
        let majorities: Vec<BTreeSet<NodeId>> = vec![
            s(&[0, 1, 2]),
            s(&[0, 1, 3]),
            s(&[0, 1, 4]),
            s(&[0, 2, 3]),
            // ...
        ];
        let q = FlexibleQuorums {
            p1: majorities.clone(),
            p2: majorities,
        };
        assert!(q.verify_intersection());
    }

    #[test]
    fn read_optimised_intersect() {
        // P1 = all of {0..4}, P2 = any single element.
        let q = FlexibleQuorums {
            p1: vec![s(&[0, 1, 2, 3, 4])],
            p2: vec![s(&[0]), s(&[1]), s(&[2]), s(&[3]), s(&[4])],
        };
        assert!(q.verify_intersection());
    }

    #[test]
    fn under_size_fails() {
        // P1 = pair {0,1}, P2 = pair {2,3} -> empty
        // intersection.
        let q = FlexibleQuorums {
            p1: vec![s(&[0, 1])],
            p2: vec![s(&[2, 3])],
        };
        assert!(!q.verify_intersection());
    }
}
