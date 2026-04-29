//! Module 0074: Cordial Miners -- conceptual placeholder.

#![warn(missing_docs)]

/// Per-round overhead complexity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overhead {
    /// `O(n)` per block: just signed broadcast.
    Linear,
    /// `O(n^2)` per block: certified RB.
    Quadratic,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overhead_distinct() {
        assert_ne!(Overhead::Linear, Overhead::Quadratic);
    }
}
