//! Module 0049: Prime pre-order primitive.

#![warn(missing_docs)]

/// Per-replica pre-order: a list of pending request ids.
#[derive(Clone, Debug, Default)]
pub struct PreOrder {
    /// Pending request ids in the order this replica saw them.
    pub pending: Vec<u32>,
}

impl PreOrder {
    /// Verify that `primary_order` respects this pre-order
    /// (every prefix of pending must appear in primary_order
    /// in the same relative order).
    #[must_use]
    pub fn respects(&self, primary_order: &[u32]) -> bool {
        let mut idx = 0;
        for id in primary_order {
            if idx < self.pending.len() && *id == self.pending[idx] {
                idx += 1;
            }
        }
        idx == self.pending.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_respecting_pre_order() {
        let p = PreOrder {
            pending: vec![1, 2, 3],
        };
        assert!(p.respects(&[1, 2, 3]));
        assert!(p.respects(&[1, 4, 2, 5, 3]));
        assert!(!p.respects(&[2, 1, 3]));
    }
}
