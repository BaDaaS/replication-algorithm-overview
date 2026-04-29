//! Module 0092: Hybrid Consensus path selector.
//!
//! Models the fast/slow path split: fast (BFT) commits at
//! network speed; slow (chain) re-elects committee.

#![warn(missing_docs)]

/// Which path Hybrid Consensus is currently on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Path {
    /// `BFT` fast path: committee can commit responsively.
    Fast,
    /// Chain slow path: committee must reconfigure first.
    Slow,
}

/// Choose the path based on the committee health.
///
/// Returns `Fast` iff the number of online honest committee
/// members exceeds `2 * f` (i.e., the `BFT` quorum threshold
/// is reachable), where `f = (committee_size - 1) / 3`.
#[must_use]
pub fn select_path(committee_size: usize, online_honest: usize) -> Path {
    let f = committee_size.saturating_sub(1) / 3;
    let quorum = 2 * f + 1;
    if online_honest >= quorum {
        Path::Fast
    } else {
        Path::Slow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fast_path_when_quorum_is_reachable() {
        assert_eq!(select_path(7, 5), Path::Fast);
        assert_eq!(select_path(7, 7), Path::Fast);
    }

    #[test]
    fn slow_path_when_quorum_is_unreachable() {
        assert_eq!(select_path(7, 4), Path::Slow);
        assert_eq!(select_path(7, 0), Path::Slow);
    }

    #[test]
    fn one_or_two_member_committee_always_quorum_meets() {
        // n = 1: f = 0, quorum = 1.
        assert_eq!(select_path(1, 1), Path::Fast);
        assert_eq!(select_path(1, 0), Path::Slow);
        // n = 2: f = 0, quorum = 1.
        assert_eq!(select_path(2, 1), Path::Fast);
        assert_eq!(select_path(2, 0), Path::Slow);
    }
}
