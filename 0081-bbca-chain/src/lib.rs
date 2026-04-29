//! Module 0081: BBCA-chain placeholder.

#![warn(missing_docs)]

/// BBCA-chain default lane count.
pub const LANES: u32 = 4;

#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn lanes() {
        assert!(super::LANES >= 1);
    }
}
