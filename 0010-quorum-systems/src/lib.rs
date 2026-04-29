//! Module 0010: a small quorum-system helper.

#![warn(missing_docs)]

use std::collections::BTreeSet;

use sim::NodeId;

/// A threshold quorum system: every quorum is a size-`threshold`
/// subset of a universe of `n` processes.
#[derive(Clone, Debug)]
pub struct ThresholdQuorum {
    /// Total number of processes.
    pub n: usize,
    /// Required quorum size.
    pub threshold: usize,
}

impl ThresholdQuorum {
    /// Build a threshold quorum.
    #[must_use]
    pub fn new(n: usize, threshold: usize) -> Self {
        Self { n, threshold }
    }

    /// Is `set` a valid quorum?
    pub fn is_quorum(&self, set: &BTreeSet<NodeId>) -> bool {
        set.len() >= self.threshold && set.len() <= self.n
    }

    /// Lower bound on the size of the intersection of any two
    /// quorums under this system.
    #[must_use]
    pub fn intersection_lower_bound(&self) -> usize {
        // |Q1| + |Q2| - n = 2 * threshold - n (if non-negative).
        (2 * self.threshold).saturating_sub(self.n)
    }

    /// `f`-Byzantine? Returns true if every pairwise
    /// intersection contains more than `f` processes.
    #[must_use]
    pub fn is_byzantine_resilient(&self, f: usize) -> bool {
        self.intersection_lower_bound() > f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_f_plus_one_is_byzantine_resilient() {
        // n = 3f + 1, threshold = 2f + 1.
        for f in 1..=10 {
            let n = 3 * f + 1;
            let q = ThresholdQuorum::new(n, 2 * f + 1);
            assert!(q.is_byzantine_resilient(f));
            assert_eq!(q.intersection_lower_bound(), f + 1);
        }
    }

    #[test]
    fn majority_is_crash_resilient_only() {
        // n = 2f + 1 majority gives intersection 1 > 0 (crash)
        // but not > f for f >= 1 (Byzantine).
        let q = ThresholdQuorum::new(5, 3);
        assert_eq!(q.intersection_lower_bound(), 1);
        assert!(q.is_byzantine_resilient(0));
        assert!(!q.is_byzantine_resilient(1));
    }

    #[test]
    fn quorum_membership() {
        let q = ThresholdQuorum::new(4, 3);
        let mut s: BTreeSet<NodeId> = BTreeSet::new();
        s.insert(NodeId(0));
        s.insert(NodeId(1));
        assert!(!q.is_quorum(&s));
        s.insert(NodeId(2));
        assert!(q.is_quorum(&s));
        s.insert(NodeId(3));
        assert!(q.is_quorum(&s));
    }
}
