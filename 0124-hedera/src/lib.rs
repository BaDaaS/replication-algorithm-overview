//! Module 0124: Hedera Council validator set skeleton.

#![warn(missing_docs)]

/// Permissioned governing council.
#[derive(Clone, Debug, Default)]
pub struct Council {
    /// Council member validator ids.
    pub members: Vec<u64>,
    /// Total stake (sum of per-member weights).
    pub total_stake: u64,
}

impl Council {
    /// Empty council.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a validator with stake. Returns false if already
    /// present.
    pub fn add(&mut self, id: u64, stake: u64) -> bool {
        if self.members.contains(&id) {
            return false;
        }
        self.members.push(id);
        self.total_stake = self.total_stake.saturating_add(stake);
        true
    }

    /// Number of council members.
    #[must_use]
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// `2 * stake / 3` quorum threshold.
    #[must_use]
    pub fn quorum(&self) -> u64 {
        (2 * self.total_stake).div_ceil(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn members_added_uniquely() {
        let mut c = Council::new();
        for i in 0..30 {
            assert!(c.add(i, 100));
        }
        assert_eq!(c.size(), 30);
        assert!(!c.add(0, 100));
    }

    #[test]
    fn quorum_is_two_thirds() {
        let mut c = Council::new();
        for i in 0..3 {
            c.add(i, 100);
        }
        // total = 300, quorum >= 200.
        assert_eq!(c.quorum(), 200);
    }
}
