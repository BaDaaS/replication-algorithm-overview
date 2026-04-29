//! Module 0126: Internet Computer notarisation/finalisation.

#![warn(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

/// Block height.
pub type Height = u64;
/// Block id.
pub type BlockId = u64;

/// `ICP` consensus state for one height.
#[derive(Clone, Debug, Default)]
pub struct IcpState {
    /// `n` total validators.
    pub n: usize,
    /// Notarised blocks: `block -> set of signer ids`.
    pub notarised: BTreeMap<BlockId, BTreeSet<u64>>,
    /// Finalised blocks: `block -> set of signer ids`.
    pub finalised: BTreeMap<BlockId, BTreeSet<u64>>,
}

impl IcpState {
    /// Build with `n` validators.
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            notarised: BTreeMap::new(),
            finalised: BTreeMap::new(),
        }
    }

    /// Threshold for notarisation: `floor(n/3) + 1`.
    #[must_use]
    pub fn notar_threshold(&self) -> usize {
        self.n / 3 + 1
    }

    /// Threshold for finalisation: `2 * floor(n/3) + 1`.
    #[must_use]
    pub fn final_threshold(&self) -> usize {
        2 * (self.n / 3) + 1
    }

    /// Record a notar signature.
    pub fn notarise(&mut self, b: BlockId, signer: u64) {
        self.notarised.entry(b).or_default().insert(signer);
    }

    /// Record a final signature.
    pub fn finalise(&mut self, b: BlockId, signer: u64) {
        self.finalised.entry(b).or_default().insert(signer);
    }

    /// True if block has reached notarisation threshold.
    #[must_use]
    pub fn is_notarised(&self, b: BlockId) -> bool {
        self.notarised
            .get(&b)
            .is_some_and(|s| s.len() >= self.notar_threshold())
    }

    /// True if block has reached finalisation threshold.
    #[must_use]
    pub fn is_finalised(&self, b: BlockId) -> bool {
        self.finalised
            .get(&b)
            .is_some_and(|s| s.len() >= self.final_threshold())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notarisation_at_f_plus_one() {
        let mut s = IcpState::new(7);
        // f = 2, threshold = 3.
        s.notarise(100, 1);
        assert!(!s.is_notarised(100));
        s.notarise(100, 2);
        assert!(!s.is_notarised(100));
        s.notarise(100, 3);
        assert!(s.is_notarised(100));
    }

    #[test]
    fn finalisation_at_two_f_plus_one() {
        let mut s = IcpState::new(7);
        // 2f+1 = 5.
        for v in 1..=4 {
            s.finalise(100, v);
        }
        assert!(!s.is_finalised(100));
        s.finalise(100, 5);
        assert!(s.is_finalised(100));
    }

    #[test]
    fn duplicate_signatures_dont_count_twice() {
        let mut s = IcpState::new(7);
        for _ in 0..10 {
            s.notarise(100, 1);
        }
        assert!(!s.is_notarised(100));
    }
}
