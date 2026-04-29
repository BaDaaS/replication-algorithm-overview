//! Module 0057: `DiemBFT` version enum.

#![warn(missing_docs)]

/// `DiemBFT` major version (lineage tracker).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Version {
    /// `LibraBFT` 2019.
    LibraV1,
    /// `DiemBFT` v2 (2020): pacemaker improvements.
    V2,
    /// `DiemBFT` v3 (2020): Jolteon two-chain.
    V3,
    /// `DiemBFT` v4 (2021): Quorum Store + Shoal.
    V4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_distinct() {
        assert_ne!(Version::LibraV1, Version::V4);
    }
}
