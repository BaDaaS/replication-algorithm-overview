//! Module 0128: Hyperledger Fabric endorsement policy.

#![warn(missing_docs)]

use std::collections::BTreeSet;

/// Peer (endorser) identifier.
pub type PeerId = u64;

/// Simple m-of-n endorsement policy: requires `m` distinct
/// endorsers from the configured allowlist.
#[derive(Clone, Debug, Default)]
pub struct EndorsementPolicy {
    /// Required threshold.
    pub m: usize,
    /// Allowed endorser ids.
    pub allowed: BTreeSet<PeerId>,
}

impl EndorsementPolicy {
    /// Build a new policy.
    #[must_use]
    pub fn new(m: usize, allowed: BTreeSet<PeerId>) -> Self {
        Self { m, allowed }
    }

    /// True if the set of endorsers meets the policy.
    #[must_use]
    pub fn is_satisfied(&self, endorsers: &BTreeSet<PeerId>) -> bool {
        let valid_count = endorsers.intersection(&self.allowed).count();
        valid_count >= self.m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(items: &[PeerId]) -> BTreeSet<PeerId> {
        items.iter().copied().collect()
    }

    #[test]
    fn two_of_three_satisfied_by_two_allowed() {
        let p = EndorsementPolicy::new(2, s(&[1, 2, 3]));
        assert!(p.is_satisfied(&s(&[1, 2])));
        assert!(p.is_satisfied(&s(&[1, 2, 3])));
    }

    #[test]
    fn two_of_three_not_satisfied_by_one() {
        let p = EndorsementPolicy::new(2, s(&[1, 2, 3]));
        assert!(!p.is_satisfied(&s(&[1])));
    }

    #[test]
    fn endorsers_outside_allowlist_dont_count() {
        let p = EndorsementPolicy::new(2, s(&[1, 2, 3]));
        // Two endorsers, but only one is in allowlist.
        assert!(!p.is_satisfied(&s(&[1, 99])));
    }
}
