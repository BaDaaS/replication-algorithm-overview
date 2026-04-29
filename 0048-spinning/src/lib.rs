//! Module 0048: Spinning -- per-round primary rotation.

#![warn(missing_docs)]

/// Compute the primary for round `r` in a group of `n` replicas.
#[must_use]
pub fn primary_for_round(r: u32, n: u32) -> u32 {
    r % n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_cycles() {
        for r in 0..10 {
            assert_eq!(primary_for_round(r, 4), r % 4);
        }
    }
}
