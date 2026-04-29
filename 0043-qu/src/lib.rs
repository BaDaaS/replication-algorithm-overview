//! Module 0043: Q/U quorum-size helpers.

#![warn(missing_docs)]

/// Q/U quorum size (read = write = 4f + 1).
#[must_use]
pub fn quorum_size(f: usize) -> usize {
    4 * f + 1
}

/// Minimum n for Q/U with `f` faults.
#[must_use]
pub fn min_n(f: usize) -> usize {
    5 * f + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qu_n6_f1() {
        assert_eq!(min_n(1), 6);
        assert_eq!(quorum_size(1), 5);
    }
}
