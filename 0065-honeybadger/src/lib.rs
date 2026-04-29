//! Module 0065: HoneyBadger BFT layer types.

#![warn(missing_docs)]

/// HoneyBadger architectural layer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    /// AVID asynchronous verifiable info dispersal.
    Avid,
    /// Parallel asynchronous binary agreement.
    Aba,
    /// Threshold encryption.
    ThresholdEnc,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layers_distinct() {
        assert_ne!(Layer::Avid, Layer::Aba);
    }
}
