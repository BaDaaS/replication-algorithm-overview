//! Module 0073: Bullshark anchor-commit rule.

#![warn(missing_docs)]

/// Number of Narwhal rounds for Bullshark's fast-path
/// anchor commit (partial synchrony).
pub const FAST_PATH_ROUNDS: u32 = 2;

#[cfg(test)]
mod tests {
    #[test]
    fn fast_path_two() {
        assert_eq!(super::FAST_PATH_ROUNDS, 2);
    }
}
