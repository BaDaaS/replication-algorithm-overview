//! Module 0079: Shoal++ pipeline depth.

#![warn(missing_docs)]

/// Shoal++ default pipeline depth.
pub const PIPELINE_DEPTH: u32 = 8;

#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn pipeline_depth() {
        assert!(super::PIPELINE_DEPTH >= 4);
    }
}
