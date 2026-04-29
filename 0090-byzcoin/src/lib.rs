//! Module 0090: `ByzCoin` committee window.
//!
//! Tracks the sliding window of recent `PoW` key-block miners
//! that compose the BFT committee.

#![warn(missing_docs)]

use std::collections::VecDeque;

/// Miner identifier.
pub type MinerId = u64;

/// Sliding-window committee of recent miners.
#[derive(Clone, Debug)]
pub struct Committee {
    members: VecDeque<MinerId>,
    capacity: usize,
}

impl Committee {
    /// Build an empty committee with the given window size.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            members: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Rotate: a new key block was mined by `miner`. The oldest
    /// member is evicted if the window is full.
    pub fn rotate(&mut self, miner: MinerId) {
        if self.members.len() >= self.capacity {
            self.members.pop_front();
        }
        self.members.push_back(miner);
    }

    /// True if `miner` is currently in the committee.
    #[must_use]
    pub fn is_member(&self, miner: MinerId) -> bool {
        self.members.iter().any(|&m| m == miner)
    }

    /// Number of members currently in the committee.
    #[must_use]
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// Capacity (window size).
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// BFT quorum threshold `2f + 1` for the current size,
    /// where `f = floor((size - 1) / 3)`.
    #[must_use]
    pub fn quorum(&self) -> usize {
        let f = self.size().saturating_sub(1) / 3;
        2 * f + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_to_capacity_then_rotates_oldest_out() {
        let mut c = Committee::new(3);
        c.rotate(1);
        c.rotate(2);
        c.rotate(3);
        assert!(c.is_member(1));
        c.rotate(4);
        assert!(!c.is_member(1));
        assert!(c.is_member(4));
        assert_eq!(c.size(), 3);
    }

    #[test]
    fn quorum_matches_pbft_two_thirds_rule() {
        let mut c = Committee::new(7);
        for i in 0..7 {
            c.rotate(i);
        }
        // n = 7, f = 2, quorum = 5.
        assert_eq!(c.quorum(), 5);
    }

    #[test]
    fn empty_committee_has_zero_size() {
        let c = Committee::new(5);
        assert_eq!(c.size(), 0);
        assert!(!c.is_member(0));
    }
}
