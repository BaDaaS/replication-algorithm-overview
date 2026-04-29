//! Module 0060: Streamlet -- chain-of-three finality rule.

#![warn(missing_docs)]

/// Streamlet's finality threshold: 3 consecutive notarised
/// blocks finalise the middle.
pub const FINALITY_CHAIN_LENGTH: usize = 3;

#[cfg(test)]
mod tests {
    #[test]
    fn finality_three() {
        assert_eq!(super::FINALITY_CHAIN_LENGTH, 3);
    }
}
