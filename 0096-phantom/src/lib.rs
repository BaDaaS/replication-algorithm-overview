//! Module 0096: PHANTOM/GHOSTDAG anticone computation.
//!
//! Provides a `BlockDag` and an `anticone` function. A full
//! `k`-cluster blue-coloring is intentionally omitted; this
//! module focuses on the structural primitives.

#![warn(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

/// Block identifier.
pub type BlockId = u64;

/// Block-DAG.
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
    /// any parent is unknown.
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

    /// All ancestors of `b` (transitive parents, excluding `b`).
    #[must_use]
    pub fn past(&self, b: BlockId) -> BTreeSet<BlockId> {
        let mut result = BTreeSet::new();
        let mut frontier: Vec<BlockId> = self
            .parents
            .get(&b)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default();
        while let Some(x) = frontier.pop() {
            if result.insert(x)
                && let Some(ps) = self.parents.get(&x)
            {
                frontier.extend(ps.iter().copied());
            }
        }
        result
    }

    /// All descendants of `b` (transitive children, excluding
    /// `b`).
    #[must_use]
    pub fn future(&self, b: BlockId) -> BTreeSet<BlockId> {
        let mut result = BTreeSet::new();
        for (&c, ps) in &self.parents {
            if ps.contains(&b) {
                result.insert(c);
            }
        }
        let mut changed = true;
        while changed {
            changed = false;
            let snapshot: Vec<BlockId> = result.iter().copied().collect();
            for x in snapshot {
                for (&c, ps) in &self.parents {
                    if ps.contains(&x) && result.insert(c) {
                        changed = true;
                    }
                }
            }
        }
        result
    }

    /// Anticone of `b`: blocks neither in past nor future, and
    /// not `b` itself.
    #[must_use]
    pub fn anticone(&self, b: BlockId) -> BTreeSet<BlockId> {
        let past = self.past(b);
        let future = self.future(b);
        self.parents
            .keys()
            .copied()
            .filter(|&x| x != b && !past.contains(&x) && !future.contains(&x))
            .collect()
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
    ///       3   4    (4 is parallel branch off 0)
    ///       |   |
    ///       5   6
    /// ```
    fn build() -> BlockDag {
        let mut d = BlockDag::new();
        d.insert(0, parents(&[]));
        d.insert(1, parents(&[0]));
        d.insert(2, parents(&[0]));
        d.insert(3, parents(&[1, 2]));
        d.insert(4, parents(&[0]));
        d.insert(5, parents(&[3]));
        d.insert(6, parents(&[4]));
        d
    }

    #[test]
    fn past_and_future() {
        let d = build();
        assert!(d.past(3).contains(&0));
        assert!(d.past(3).contains(&1));
        assert!(d.past(3).contains(&2));
        assert!(d.future(0).contains(&3));
        assert!(d.future(0).contains(&5));
    }

    #[test]
    fn anticone_of_block_3_includes_parallel_branches() {
        let d = build();
        let ac = d.anticone(3);
        assert!(ac.contains(&4));
        assert!(ac.contains(&6));
        assert!(!ac.contains(&1));
        assert!(!ac.contains(&2));
        assert!(!ac.contains(&5));
    }

    #[test]
    fn anticone_of_genesis_is_empty() {
        let d = build();
        assert!(d.anticone(0).is_empty());
    }
}
