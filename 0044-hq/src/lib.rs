//! Module 0044: HQ Replication -- conceptual placeholder.

#![warn(missing_docs)]

/// HQ Replication path taken by an operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Path {
    /// Fast path (no contention): 1 round-trip.
    Fast,
    /// Slow path (PBFT-style): three phases.
    Slow,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_distinct() {
        assert_ne!(Path::Fast, Path::Slow);
    }
}
