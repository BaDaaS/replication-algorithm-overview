//! Module 0076: Mysticeti commit-path discriminator.

#![warn(missing_docs)]

/// Commit path taken by a Mysticeti block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitPath {
    /// Fast: 2/3 super-majority in next round.
    Fast,
    /// Steady: 2f + 1 in 2 rounds.
    Steady,
    /// Async fallback: wave anchor.
    Async,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paths_distinct() {
        assert_ne!(CommitPath::Fast, CommitPath::Steady);
        assert_ne!(CommitPath::Steady, CommitPath::Async);
    }
}
