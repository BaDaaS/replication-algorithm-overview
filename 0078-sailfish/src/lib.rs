//! Module 0078: Sailfish direct-commit constant.

#![warn(missing_docs)]

/// Sailfish direct-commit rounds.
pub const COMMIT_ROUNDS: u32 = 1;

#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn one_round() {
        assert_eq!(super::COMMIT_ROUNDS, 1);
    }
}
