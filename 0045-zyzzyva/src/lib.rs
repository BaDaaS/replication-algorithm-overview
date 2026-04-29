//! Module 0045: Zyzzyva path-decision helper.

#![warn(missing_docs)]

/// Zyzzyva client decision based on number of matching replies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClientDecision {
    /// All 3f + 1 matched: fast-path commit.
    FastCommit,
    /// 2f + 1 matched: trigger commit phase.
    SlowCommit,
    /// Fewer matched: fall back to PBFT.
    Fallback,
}

/// Compute the client's decision given `matched` replies out of
/// `n` replicas with `f` Byzantine bound.
#[must_use]
pub fn classify(matched: usize, n: usize, f: usize) -> ClientDecision {
    if matched == n {
        ClientDecision::FastCommit
    } else if matched >= 2 * f + 1 {
        ClientDecision::SlowCommit
    } else {
        ClientDecision::Fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_n4_f1() {
        // n = 4, f = 1: fast = 4, slow = 3, fallback = otherwise.
        assert_eq!(classify(4, 4, 1), ClientDecision::FastCommit);
        assert_eq!(classify(3, 4, 1), ClientDecision::SlowCommit);
        assert_eq!(classify(2, 4, 1), ClientDecision::Fallback);
    }
}
