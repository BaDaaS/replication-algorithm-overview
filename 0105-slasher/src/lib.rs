//! Module 0105: Slasher equivocation detection.
//!
//! Detects double-vote evidence: two signed messages by the
//! same staker at the same height for different blocks.

#![warn(missing_docs)]

/// A signed vote message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedVote {
    /// Staker who signed.
    pub staker: u64,
    /// Height of the vote.
    pub height: u64,
    /// Block voted for.
    pub block: u64,
}

/// Slashable-evidence variants.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Slashable {
    /// Double-vote: two votes at the same height by the same
    /// staker.
    DoubleVote(SignedVote, SignedVote),
}

/// Scan a list of votes; return any slashable double-vote
/// pair found. Returns the first such pair.
#[must_use]
pub fn detect(votes: &[SignedVote]) -> Option<Slashable> {
    for (i, a) in votes.iter().enumerate() {
        for b in &votes[i + 1..] {
            if a.staker == b.staker
                && a.height == b.height
                && a.block != b.block
            {
                return Some(Slashable::DoubleVote(a.clone(), b.clone()));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(staker: u64, height: u64, block: u64) -> SignedVote {
        SignedVote {
            staker,
            height,
            block,
        }
    }

    #[test]
    fn double_vote_detected() {
        let votes = vec![v(1, 5, 10), v(1, 5, 11)];
        let r = detect(&votes);
        assert!(matches!(r, Some(Slashable::DoubleVote(_, _))));
    }

    #[test]
    fn votes_at_different_heights_not_slashable() {
        let votes = vec![v(1, 5, 10), v(1, 6, 11)];
        assert_eq!(detect(&votes), None);
    }

    #[test]
    fn votes_by_different_stakers_not_slashable() {
        let votes = vec![v(1, 5, 10), v(2, 5, 11)];
        assert_eq!(detect(&votes), None);
    }

    #[test]
    fn duplicate_vote_not_slashable() {
        let votes = vec![v(1, 5, 10), v(1, 5, 10)];
        assert_eq!(detect(&votes), None);
    }
}
