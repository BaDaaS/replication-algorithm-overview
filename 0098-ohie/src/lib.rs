//! Module 0098: OHIE rank-merge ledger.
//!
//! Holds `m` parallel Nakamoto chains and produces a total
//! order by rank-interleaving:
//! `(0, 0), (0, 1), ..., (0, m-1), (1, 0), ...`.

#![warn(missing_docs)]

/// `m` parallel Nakamoto chains.
#[derive(Clone, Debug)]
pub struct OhieLedger {
    chains: Vec<Vec<u64>>,
}

impl OhieLedger {
    /// Create with `m` empty chains.
    #[must_use]
    pub fn new(m: usize) -> Self {
        Self {
            chains: vec![Vec::new(); m],
        }
    }

    /// Append `block_id` to chain `idx`. Returns false if `idx`
    /// is out of range.
    pub fn append(&mut self, idx: usize, block_id: u64) -> bool {
        if idx >= self.chains.len() {
            return false;
        }
        self.chains[idx].push(block_id);
        true
    }

    /// Number of parallel chains.
    #[must_use]
    pub fn m(&self) -> usize {
        self.chains.len()
    }

    /// Length of chain `idx`.
    #[must_use]
    pub fn len(&self, idx: usize) -> usize {
        self.chains.get(idx).map_or(0, std::vec::Vec::len)
    }

    /// Compute the rank-interleaved total order. Stops at the
    /// minimum chain length: rank `i` is included only when
    /// every chain has a block at index `i`.
    #[must_use]
    pub fn merge(&self) -> Vec<u64> {
        let min_len = self.chains.iter().map(Vec::len).min().unwrap_or(0);
        let mut result = Vec::with_capacity(min_len * self.chains.len());
        for i in 0..min_len {
            for c in &self.chains {
                result.push(c[i]);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_ledger_merges_to_empty_vec() {
        let l = OhieLedger::new(3);
        assert!(l.merge().is_empty());
    }

    #[test]
    fn rank_interleaving_groups_by_rank() {
        let mut l = OhieLedger::new(3);
        for i in 0..3 {
            l.append(i, 100 + i as u64);
            l.append(i, 200 + i as u64);
        }
        // Expected: 100, 101, 102, 200, 201, 202.
        assert_eq!(l.merge(), vec![100, 101, 102, 200, 201, 202]);
    }

    #[test]
    fn slow_chain_truncates_merged_output() {
        let mut l = OhieLedger::new(2);
        l.append(0, 1);
        l.append(0, 2);
        l.append(0, 3);
        l.append(1, 10);
        // Chain 1 has only one block; merge stops at rank 0.
        assert_eq!(l.merge(), vec![1, 10]);
    }

    #[test]
    fn append_to_invalid_chain_returns_false() {
        let mut l = OhieLedger::new(2);
        assert!(!l.append(99, 0));
    }
}
