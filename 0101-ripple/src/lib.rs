//! Module 0101: Ripple `RPCA` 80%-UNL voting.
//!
//! Each node has a Unique Node List (UNL); a proposed ledger
//! passes when 80% of the UNL signs it.

#![warn(missing_docs)]

use std::collections::BTreeSet;

/// Validator id.
pub type ValidatorId = u64;

/// A Ripple node with its UNL.
#[derive(Clone, Debug, Default)]
pub struct RippleNode {
    /// Trusted validators.
    pub unl: BTreeSet<ValidatorId>,
}

impl RippleNode {
    /// Build a node with the given UNL.
    #[must_use]
    pub fn new<I: IntoIterator<Item = ValidatorId>>(unl: I) -> Self {
        Self {
            unl: unl.into_iter().collect(),
        }
    }

    /// Returns true if the proposed signature set covers at
    /// least 80% of the node's UNL.
    #[must_use]
    pub fn vote_pass(&self, signers: &BTreeSet<ValidatorId>) -> bool {
        if self.unl.is_empty() {
            return false;
        }
        let agreed = self.unl.intersection(signers).count();
        // Threshold: at least ceil(0.80 * unl.len()).
        let threshold = (4 * self.unl.len()).div_ceil(5);
        agreed >= threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(items: &[ValidatorId]) -> BTreeSet<ValidatorId> {
        items.iter().copied().collect()
    }

    #[test]
    fn full_signers_passes() {
        let n = RippleNode::new([1, 2, 3, 4, 5]);
        assert!(n.vote_pass(&s(&[1, 2, 3, 4, 5])));
    }

    #[test]
    fn four_of_five_passes_eighty_percent() {
        let n = RippleNode::new([1, 2, 3, 4, 5]);
        assert!(n.vote_pass(&s(&[1, 2, 3, 4])));
    }

    #[test]
    fn three_of_five_fails_eighty_percent() {
        let n = RippleNode::new([1, 2, 3, 4, 5]);
        assert!(!n.vote_pass(&s(&[1, 2, 3])));
    }

    #[test]
    fn signatures_outside_unl_do_not_count() {
        let n = RippleNode::new([1, 2, 3, 4, 5]);
        // Three from UNL plus two strangers.
        assert!(!n.vote_pass(&s(&[1, 2, 3, 99, 100])));
    }

    #[test]
    fn empty_unl_never_passes() {
        let n = RippleNode::default();
        assert!(!n.vote_pass(&s(&[1, 2, 3])));
    }
}
