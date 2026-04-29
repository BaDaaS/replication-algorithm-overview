//! Module 0112: Ouroboros Genesis chain-density rule.
//!
//! Counts blocks within a `(start, end)` window and selects
//! the chain with greater density.

#![warn(missing_docs)]

/// Block with a slot number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DenseBlock {
    /// Block id.
    pub id: u64,
    /// Slot at which the block was produced.
    pub slot: u64,
}

/// Density: number of blocks in `[start, end]`.
#[must_use]
pub fn density(chain: &[DenseBlock], start: u64, end: u64) -> usize {
    chain
        .iter()
        .filter(|b| b.slot >= start && b.slot <= end)
        .count()
}

/// Pick the chain with greater density in the given window;
/// ties broken by `a`.
#[must_use]
pub fn prefer_dense<'a>(
    a: &'a [DenseBlock],
    b: &'a [DenseBlock],
    start: u64,
    end: u64,
) -> &'a [DenseBlock] {
    if density(b, start, end) > density(a, start, end) {
        b
    } else {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(id: u64, slot: u64) -> DenseBlock {
        DenseBlock { id, slot }
    }

    #[test]
    fn dense_chain_wins_in_overlapping_window() {
        let dense = vec![b(1, 10), b(2, 11), b(3, 12), b(4, 13)];
        let sparse = vec![b(10, 10), b(11, 13)];
        let pick = prefer_dense(&sparse, &dense, 10, 13);
        assert_eq!(pick[0].id, 1);
    }

    #[test]
    fn ties_keep_the_first_argument() {
        let one = vec![b(1, 10)];
        let two = vec![b(2, 10)];
        let pick = prefer_dense(&one, &two, 10, 10);
        assert_eq!(pick[0].id, 1);
    }

    #[test]
    fn empty_window_yields_zero_density() {
        let chain = vec![b(1, 10), b(2, 20)];
        assert_eq!(density(&chain, 30, 40), 0);
    }
}
