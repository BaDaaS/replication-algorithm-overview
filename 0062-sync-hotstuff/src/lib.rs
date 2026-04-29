//! Module 0062: Sync `HotStuff` resilience helper.

#![warn(missing_docs)]

/// Sync `HotStuff` minimum n: 2f + 1.
#[must_use]
pub fn min_n(f: usize) -> usize {
    2 * f + 1
}

/// Sync `HotStuff` quorum: f + 1.
#[must_use]
pub fn quorum(f: usize) -> usize {
    f + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sync_hotstuff_f1() {
        assert_eq!(min_n(1), 3);
        assert_eq!(quorum(1), 2);
    }
}
