//! Module 0094: SPECTRE block-DAG.
//!
//! Models a directed acyclic graph of blocks; each block
//! references a set of parent blocks. Provides tip detection
//! and ancestry queries.

#![warn(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

/// Block identifier.
pub type BlockId = u64;

/// Block-DAG: each block records its parent set.
#[derive(Clone, Debug, Default)]
pub struct BlockDag {
    /// `block -> set of parents`.
    pub parents: BTreeMap<BlockId, BTreeSet<BlockId>>,
}

impl BlockDag {
    /// Empty DAG.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a block with the given parents. Returns false if
    /// any parent is unknown (and the block is not the genesis,
    /// which has empty parents).
    pub fn insert(
        &mut self,
        id: BlockId,
        new_parents: BTreeSet<BlockId>,
    ) -> bool {
        if !new_parents.iter().all(|p| self.parents.contains_key(p)) {
            return false;
        }
        self.parents.insert(id, new_parents);
        true
    }

    /// True if `b` is in the DAG.
    #[must_use]
    pub fn contains(&self, b: BlockId) -> bool {
        self.parents.contains_key(&b)
    }

    /// Tips of the DAG: blocks that are not parents of any
    /// other block.
    #[must_use]
    pub fn tips(&self) -> BTreeSet<BlockId> {
        let mut tips: BTreeSet<BlockId> =
            self.parents.keys().copied().collect();
        for ps in self.parents.values() {
            for p in ps {
                tips.remove(p);
            }
        }
        tips
    }

    /// True if `a` is an ancestor of `b` (transitive parent).
    #[must_use]
    pub fn is_ancestor(&self, a: BlockId, b: BlockId) -> bool {
        if a == b {
            return false;
        }
        let mut frontier: Vec<BlockId> = self
            .parents
            .get(&b)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default();
        let mut visited = BTreeSet::new();
        while let Some(x) = frontier.pop() {
            if x == a {
                return true;
            }
            if !visited.insert(x) {
                continue;
            }
            if let Some(ps) = self.parents.get(&x) {
                frontier.extend(ps.iter().copied());
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parents(items: &[BlockId]) -> BTreeSet<BlockId> {
        items.iter().copied().collect()
    }

    /// DAG:
    /// ```text
    ///       0
    ///      / \
    ///     1   2
    ///      \ /
    ///       3
    /// ```
    fn build() -> BlockDag {
        let mut d = BlockDag::new();
        d.insert(0, parents(&[]));
        d.insert(1, parents(&[0]));
        d.insert(2, parents(&[0]));
        d.insert(3, parents(&[1, 2]));
        d
    }

    #[test]
    fn tip_detection() {
        let d = build();
        let mut t = d.tips();
        assert!(t.contains(&3));
        t.remove(&3);
        assert!(t.is_empty());
    }

    #[test]
    fn ancestor_traversal_finds_indirect_ancestor() {
        let d = build();
        assert!(d.is_ancestor(0, 3));
        assert!(d.is_ancestor(1, 3));
        assert!(d.is_ancestor(2, 3));
        assert!(!d.is_ancestor(3, 0));
    }

    #[test]
    fn unknown_parent_rejected() {
        let mut d = BlockDag::new();
        d.insert(0, parents(&[]));
        assert!(!d.insert(1, parents(&[99])));
    }
}
