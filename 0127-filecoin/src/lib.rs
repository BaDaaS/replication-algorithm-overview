//! Module 0127: Filecoin Expected Consensus tipset weight.

#![warn(missing_docs)]

use std::collections::BTreeSet;

/// Block id.
pub type BlockId = u64;

/// A Filecoin tipset: set of blocks at the same height with
/// the same parent tipset.
#[derive(Clone, Debug, Default)]
pub struct Tipset {
    /// Blocks in the tipset.
    pub blocks: BTreeSet<BlockId>,
    /// Aggregate weight (sum of per-block weights).
    pub weight: u64,
}

impl Tipset {
    /// Build with the given block set and weight.
    #[must_use]
    pub fn new(blocks: BTreeSet<BlockId>, weight: u64) -> Self {
        Self { blocks, weight }
    }

    /// Returns the heavier of two tipsets; ties broken by `a`.
    #[must_use]
    pub fn heavier<'a>(a: &'a Tipset, b: &'a Tipset) -> &'a Tipset {
        if b.weight > a.weight { b } else { a }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ts(blocks: &[BlockId], weight: u64) -> Tipset {
        Tipset::new(blocks.iter().copied().collect(), weight)
    }

    #[test]
    fn heavier_tipset_wins() {
        let a = ts(&[1, 2], 10);
        let b = ts(&[3, 4, 5], 30);
        assert_eq!(Tipset::heavier(&a, &b).weight, 30);
    }

    #[test]
    fn tied_weight_keeps_first() {
        let a = ts(&[1], 10);
        let b = ts(&[2], 10);
        assert_eq!(Tipset::heavier(&a, &b).weight, 10);
        assert!(Tipset::heavier(&a, &b).blocks.contains(&1));
    }

    #[test]
    fn tipset_can_have_multiple_blocks() {
        let t = ts(&[1, 2, 3, 4], 100);
        assert_eq!(t.blocks.len(), 4);
    }
}
