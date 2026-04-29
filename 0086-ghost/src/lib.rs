//! Module 0086: GHOST (Greedy Heaviest-Observed Subtree)
//! fork-choice rule.
//!
//! Provides a minimal block tree and the `ghost_tip` function
//! implementing the heaviest-subtree selection rule.

#![warn(missing_docs)]

use std::collections::BTreeMap;

/// Block identifier in the tree.
pub type BlockId = u64;

/// A block tree. `root` is the genesis; each block records its
/// parent.
#[derive(Clone, Debug, Default)]
pub struct BlockTree {
    /// Parent map: child id -> parent id.
    pub parent_of: BTreeMap<BlockId, BlockId>,
    /// All known block ids (including the root).
    pub blocks: Vec<BlockId>,
}

impl BlockTree {
    /// Create a tree with a given root id.
    #[must_use]
    pub fn new(root: BlockId) -> Self {
        Self {
            parent_of: BTreeMap::new(),
            blocks: vec![root],
        }
    }

    /// Insert a block as a child of `parent`. Returns false if
    /// `parent` is unknown or `id` already exists.
    pub fn insert(&mut self, id: BlockId, parent: BlockId) -> bool {
        if !self.blocks.contains(&parent) || self.blocks.contains(&id) {
            return false;
        }
        self.parent_of.insert(id, parent);
        self.blocks.push(id);
        true
    }

    /// Children of `b`.
    #[must_use]
    pub fn children(&self, b: BlockId) -> Vec<BlockId> {
        self.parent_of
            .iter()
            .filter_map(|(&c, &p)| if p == b { Some(c) } else { None })
            .collect()
    }

    /// Subtree size rooted at `b` (including `b`).
    #[must_use]
    pub fn subtree_size(&self, b: BlockId) -> usize {
        let mut count = 1;
        for c in self.children(b) {
            count += self.subtree_size(c);
        }
        count
    }
}

/// GHOST rule: starting at `start`, descend by always picking the
/// child with the heaviest subtree. Ties broken by lowest id.
#[must_use]
pub fn ghost_tip(tree: &BlockTree, start: BlockId) -> BlockId {
    let mut node = start;
    loop {
        let kids = tree.children(node);
        if kids.is_empty() {
            return node;
        }
        let mut best = kids[0];
        let mut best_w = tree.subtree_size(best);
        for &k in &kids[1..] {
            let w = tree.subtree_size(k);
            if w > best_w || (w == best_w && k < best) {
                best = k;
                best_w = w;
            }
        }
        node = best;
    }
}

/// Longest-chain rule: descend by always picking the deepest
/// descendant. Ties broken by lowest id.
#[must_use]
pub fn longest_chain_tip(tree: &BlockTree, start: BlockId) -> BlockId {
    fn depth(t: &BlockTree, b: BlockId) -> usize {
        1 + t
            .children(b)
            .into_iter()
            .map(|c| depth(t, c))
            .max()
            .unwrap_or(0)
    }
    let mut node = start;
    loop {
        let kids = tree.children(node);
        if kids.is_empty() {
            return node;
        }
        let mut best = kids[0];
        let mut best_d = depth(tree, best);
        for &k in &kids[1..] {
            let d = depth(tree, k);
            if d > best_d || (d == best_d && k < best) {
                best = k;
                best_d = d;
            }
        }
        node = best;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tree:
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     /|\   \
    ///    3 4 5   6
    ///            |
    ///            7
    /// ```
    fn build_tree() -> BlockTree {
        let mut t = BlockTree::new(0);
        for (id, p) in [(1, 0), (2, 0), (3, 1), (4, 1), (5, 1), (6, 2), (7, 6)]
        {
            assert!(t.insert(id, p));
        }
        t
    }

    #[test]
    fn ghost_picks_heavier_subtree() {
        let t = build_tree();
        // Subtree sizes from root: 1 -> {3,4,5} = 4; 2 -> {6,7} = 3.
        // GHOST descends to 1, then picks min-id leaf among 3,4,5: 3.
        assert_eq!(ghost_tip(&t, 0), 3);
    }

    #[test]
    fn longest_chain_picks_deepest_branch() {
        let t = build_tree();
        // Branch 0->1->{3|4|5} has depth 3.
        // Branch 0->2->6->7 has depth 4.
        assert_eq!(longest_chain_tip(&t, 0), 7);
    }

    #[test]
    fn ghost_and_longest_can_diverge() {
        let t = build_tree();
        assert_ne!(ghost_tip(&t, 0), longest_chain_tip(&t, 0));
    }
}
