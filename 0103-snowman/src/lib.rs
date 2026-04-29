//! Module 0103: Snowman linear-chain consensus.
//!
//! A linear chain of blocks; each candidate block goes
//! through Snowball voting before acceptance.

#![warn(missing_docs)]

/// Block id.
pub type BlockId = u64;

/// Snowman chain.
#[derive(Clone, Debug, Default)]
pub struct SnowmanChain {
    /// Finalised blocks in order.
    pub blocks: Vec<BlockId>,
    /// Currently-considered block id, if any.
    pub pending: Option<BlockId>,
    /// Confidence count for the current pending block.
    pub confidence: u32,
}

impl SnowmanChain {
    /// Empty chain.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Apply a query result on candidate block `b` with `agree`
    /// agreements out of `k` samples; threshold is `alpha`.
    /// Returns true if the block was accepted (finalised).
    pub fn query(
        &mut self,
        b: BlockId,
        agree: u32,
        alpha: u32,
        beta: u32,
    ) -> bool {
        if agree < alpha {
            self.pending = None;
            self.confidence = 0;
            return false;
        }
        if self.pending == Some(b) {
            self.confidence += 1;
        } else {
            self.pending = Some(b);
            self.confidence = 1;
        }
        if self.confidence >= beta {
            self.blocks.push(b);
            self.pending = None;
            self.confidence = 0;
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_accepted_after_beta_successes() {
        let mut c = SnowmanChain::new();
        for i in 0..4 {
            assert!(!c.query(7, 15, 12, 5), "iter {i}");
        }
        assert!(c.query(7, 15, 12, 5));
        assert_eq!(c.blocks, vec![7]);
    }

    #[test]
    fn streak_resets_on_failure() {
        let mut c = SnowmanChain::new();
        c.query(1, 15, 12, 3);
        c.query(1, 15, 12, 3);
        c.query(1, 5, 12, 3); // failure
        assert_eq!(c.confidence, 0);
        assert!(c.blocks.is_empty());
    }

    #[test]
    fn switching_block_resets_streak() {
        let mut c = SnowmanChain::new();
        c.query(1, 15, 12, 3);
        c.query(1, 15, 12, 3);
        c.query(2, 15, 12, 3); // switch to block 2
        assert_eq!(c.pending, Some(2));
        assert_eq!(c.confidence, 1);
    }
}
