//! Module 0066: BEAT family discriminator.

#![warn(missing_docs)]

/// BEAT variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variant {
    /// BEAT0: direct `HoneyBadger` refinement.
    B0,
    /// BEAT1: latency for bandwidth.
    B1,
    /// BEAT2: bandwidth for latency.
    B2,
    /// BEAT3: KZG-based dispersal.
    B3,
    /// BEAT4: KZG + BEAT2 parallelism.
    B4,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn variants_distinct() {
        assert_ne!(Variant::B0, Variant::B4);
    }
}
