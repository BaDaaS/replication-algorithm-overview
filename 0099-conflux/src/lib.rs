//! Module 0099: Conflux tree-graph.
//!
//! Each block has a single parent (tree) plus a set of
//! reference edges (DAG). The pivot chain is computed by
//! GHOST on the tree alone.

#![warn(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

/// Block id.
pub type BlockId = u64;

/// Tree-graph: tree structure plus DAG reference edges.
#[derive(Clone, Debug, Default)]
pub struct TreeGraph {
    /// `block -> parent` (tree edge).
    pub parent: BTreeMap<BlockId, BlockId>,
    /// `block -> reference set` (DAG edges, non-parent).
    pub references: BTreeMap<BlockId, BTreeSet<BlockId>>,
    /// All known blocks.
    pub blocks: BTreeSet<BlockId>,
}

impl TreeGraph {
    /// Build a tree-graph with given root id.
    #[must_use]
    pub fn new(root: BlockId) -> Self {
        let mut blocks = BTreeSet::new();
        blocks.insert(root);
        Self {
            parent: BTreeMap::new(),
            references: BTreeMap::new(),
            blocks,
        }
    }

    /// Insert a block with given parent and reference edges.
    /// Returns false if any reference is unknown or the parent
    /// is unknown, or if `id` is already known.
    pub fn insert(
        &mut self,
        id: BlockId,
        parent: BlockId,
        refs: BTreeSet<BlockId>,
    ) -> bool {
        if !self.blocks.contains(&parent) {
            return false;
        }
        if !refs.iter().all(|r| self.blocks.contains(r)) {
            return false;
        }
        if !self.blocks.insert(id) {
            return false;
        }
        self.parent.insert(id, parent);
        self.references.insert(id, refs);
        true
    }

    /// Children of `b` in the tree (parent-edge only).
    #[must_use]
    pub fn tree_children(&self, b: BlockId) -> Vec<BlockId> {
        self.parent
            .iter()
            .filter_map(|(&c, &p)| if p == b { Some(c) } else { None })
            .collect()
    }

    /// Tree subtree size rooted at `b`.
    #[must_use]
    pub fn tree_subtree(&self, b: BlockId) -> usize {
        let mut count = 1;
        for c in self.tree_children(b) {
            count += self.tree_subtree(c);
        }
        count
    }

    /// Pivot chain: GHOST on the tree edges only.
    /// Returns the sequence from `root` to the GHOST tip.
    #[must_use]
    pub fn pivot_chain(&self, root: BlockId) -> Vec<BlockId> {
        let mut chain = vec![root];
        let mut node = root;
        loop {
            let kids = self.tree_children(node);
            if kids.is_empty() {
                return chain;
            }
            let mut best = kids[0];
            let mut best_w = self.tree_subtree(best);
            for &k in &kids[1..] {
                let w = self.tree_subtree(k);
                if w > best_w || (w == best_w && k < best) {
                    best = k;
                    best_w = w;
                }
            }
            node = best;
            chain.push(node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn refset(items: &[BlockId]) -> BTreeSet<BlockId> {
        items.iter().copied().collect()
    }

    /// Tree (parent edges only):
    /// ```text
    ///       0
    ///      / \
    ///     1   2
    ///    / \   \
    ///   3   4   5
    /// ```
    /// Reference edges: 4 -> 5, 5 -> 3 (DAG).
    fn build() -> TreeGraph {
        let mut g = TreeGraph::new(0);
        g.insert(1, 0, refset(&[]));
        g.insert(2, 0, refset(&[]));
        g.insert(3, 1, refset(&[]));
        g.insert(4, 1, refset(&[]));
        g.insert(5, 2, refset(&[]));
        g.references.entry(4).or_default().insert(5);
        g.references.entry(5).or_default().insert(3);
        g
    }

    #[test]
    fn pivot_chain_uses_tree_only() {
        let g = build();
        // Subtree sizes from root: 1 -> {3,4} = 3; 2 -> {5} = 2.
        // GHOST descends to 1, then to 3 (min-id leaf among
        // {3,4} both size-1).
        assert_eq!(g.pivot_chain(0), vec![0, 1, 3]);
    }

    #[test]
    fn references_do_not_affect_tree_subtree() {
        let g = build();
        assert_eq!(g.tree_subtree(2), 2);
    }

    #[test]
    fn rejects_unknown_parent() {
        let mut g = TreeGraph::new(0);
        assert!(!g.insert(1, 99, refset(&[])));
    }
}
