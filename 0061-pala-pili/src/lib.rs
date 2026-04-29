//! Module 0061: Pala/Pili family discriminator.

#![warn(missing_docs)]

/// Pala/Pili variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variant {
    /// Pala (2018): two-chain partial-synchrony BFT.
    Pala,
    /// Pili (2019): Pala + asynchronous fallback.
    Pili,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_distinct() {
        assert_ne!(Variant::Pala, Variant::Pili);
    }
}
