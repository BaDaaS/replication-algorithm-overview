//! Module 0063: SBFT -- conceptual placeholder.

#![warn(missing_docs)]

/// SBFT path classification: fast (all responsive) vs slow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Path {
    /// Fast: all 3f + 1 replicas responsive.
    Fast,
    /// Slow: HotStuff-style two-chain fallback.
    Slow,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paths_distinct() {
        assert_ne!(Path::Fast, Path::Slow);
    }
}
