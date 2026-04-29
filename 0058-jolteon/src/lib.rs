//! Module 0058: Jolteon two-chain commit rule.

#![warn(missing_docs)]

/// Number of consecutive QC-extending children required to
/// commit, per protocol family.
#[must_use]
pub fn commit_chain_length(family: ChainFamily) -> usize {
    match family {
        ChainFamily::HotStuff => 3,
        ChainFamily::Jolteon => 2,
    }
}

/// HotStuff vs Jolteon chain-length classification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChainFamily {
    /// `HotStuff`: three-chain commit.
    HotStuff,
    /// Jolteon: two-chain commit.
    Jolteon,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jolteon_shorter() {
        assert!(
            commit_chain_length(ChainFamily::Jolteon)
                < commit_chain_length(ChainFamily::HotStuff)
        );
    }
}
