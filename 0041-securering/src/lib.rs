//! Module 0041: `SecureRing` -- ring-topology helpers.

#![warn(missing_docs)]

use sim::NodeId;

/// Logical ring of replicas; the next-in-ring relation.
#[derive(Clone, Debug)]
pub struct Ring {
    /// Ordered list of replicas.
    pub order: Vec<NodeId>,
}

impl Ring {
    /// Build a ring.
    #[must_use]
    pub fn new(order: Vec<NodeId>) -> Self {
        Self { order }
    }

    /// Next-in-ring after `id`, wrapping.
    #[must_use]
    pub fn next(&self, id: NodeId) -> Option<NodeId> {
        let pos = self.order.iter().position(|&n| n == id)?;
        let n = self.order.len();
        Some(self.order[(pos + 1) % n])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_next_wraps() {
        let r = Ring::new(vec![NodeId(0), NodeId(1), NodeId(2)]);
        assert_eq!(r.next(NodeId(0)), Some(NodeId(1)));
        assert_eq!(r.next(NodeId(2)), Some(NodeId(0)));
    }
}
