//! Module 0039: Atlas fast-quorum size helper.

#![warn(missing_docs)]

/// Atlas fast quorum size: `f + ceil(f/2) + floor(c/2)`.
#[must_use]
pub fn fast_quorum(f: usize, c: usize) -> usize {
    f + f.div_ceil(2) + c / 2
}

/// Atlas slow quorum (majority).
#[must_use]
pub fn slow_quorum(n: usize) -> usize {
    n / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atlas_n5_f2_low_conflict() {
        // n = 5, f = 2, c = 0: fast = 3.
        assert_eq!(fast_quorum(2, 0), 3);
        assert_eq!(slow_quorum(5), 3);
    }

    #[test]
    fn atlas_n5_f2_high_conflict() {
        // n = 5, f = 2, c = 2: fast = 4 (EPaxos-equivalent).
        assert_eq!(fast_quorum(2, 2), 4);
    }
}
