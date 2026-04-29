//! Module 0075: Shoal pipelined-anchor placeholder.

#![warn(missing_docs)]

/// Maximum number of anchors in flight (pipeline depth).
pub const PIPELINE_DEPTH: u32 = 4;

#[cfg(test)]
mod tests {
    #[test]
    fn pipeline_depth() {
        assert_eq!(super::PIPELINE_DEPTH, 4);
    }
}
