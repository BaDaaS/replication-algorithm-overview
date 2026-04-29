//! Module 0071: DAG-Rider wave structure.

#![warn(missing_docs)]

/// Number of rounds per DAG-Rider wave.
pub const WAVE_LENGTH: u32 = 4;

#[cfg(test)]
mod tests {
    #[test]
    fn wave_four() {
        assert_eq!(super::WAVE_LENGTH, 4);
    }
}
