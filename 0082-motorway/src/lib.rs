//! Module 0082: Motorway placeholder.

#![warn(missing_docs)]

/// Motorway lane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane {
    /// Fast lane: low-latency replicas.
    Fast,
    /// Slow lane: heterogeneous replicas.
    Slow,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lanes_distinct() {
        assert_ne!(Lane::Fast, Lane::Slow);
    }
}
