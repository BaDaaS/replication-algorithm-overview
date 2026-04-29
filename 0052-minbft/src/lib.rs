//! Module 0052: `MinBFT` thresholds.

#![warn(missing_docs)]

/// `MinBFT` minimum n given f trusted-hardware-equipped
/// replicas: 2f + 1.
#[must_use]
pub fn min_n(f: usize) -> usize {
    2 * f + 1
}

/// `MinBFT` quorum: f + 1.
#[must_use]
pub fn quorum(f: usize) -> usize {
    f + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minbft_f1() {
        assert_eq!(min_n(1), 3);
        assert_eq!(quorum(1), 2);
    }
}
