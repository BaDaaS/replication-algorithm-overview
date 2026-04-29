//! Module 0067: Dumbo variant discriminator.

#![warn(missing_docs)]

/// Dumbo variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variant {
    /// Dumbo1: committee-based, O(log n) ABAs per epoch.
    D1,
    /// Dumbo2: single-batch ABA per epoch.
    D2,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn variants_distinct() {
        assert_ne!(Variant::D1, Variant::D2);
    }
}
