//! Module 0083: Bitcoin / Nakamoto consensus block model.
//!
//! Minimal Bitcoin-style block: header, parent hash, nonce,
//! transactions. Tests verify a chain of 3 blocks satisfies the
//! longest-chain rule (each subsequent block extends the previous).
//!
//! A real proof-of-work miner (computing valid SHA-256 nonces) is
//! intentionally omitted; the simulation focuses on chain structure
//! rather than mining.

#![warn(missing_docs)]

/// 32-byte hash digest.
pub type Hash = [u8; 32];

/// All-zero genesis parent hash.
pub const GENESIS_PARENT: Hash = [0u8; 32];

/// A Bitcoin-style block header.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    /// Hash of the previous block.
    pub parent: Hash,
    /// Merkle root of transactions (placeholder).
    pub merkle_root: Hash,
    /// Block-creation timestamp.
    pub timestamp: u64,
    /// Difficulty target (lower is harder).
    pub target: u64,
    /// Proof-of-work witness (not validated here).
    pub nonce: u64,
}

/// A complete block: header plus opaque transactions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    /// Block header.
    pub header: BlockHeader,
    /// Block hash (placeholder, not verified).
    pub hash: Hash,
    /// Transactions in this block.
    pub txs: Vec<Vec<u8>>,
}

/// A chain of blocks; index 0 is genesis.
#[derive(Clone, Debug, Default)]
pub struct Chain {
    /// Blocks in linear order.
    pub blocks: Vec<Block>,
}

impl Chain {
    /// Build an empty chain.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a block. Returns false if the parent does not match
    /// the current tip (or the genesis sentinel for the first block).
    pub fn append(&mut self, b: Block) -> bool {
        let expected_parent =
            self.blocks.last().map_or(GENESIS_PARENT, |t| t.hash);
        if b.header.parent != expected_parent {
            return false;
        }
        self.blocks.push(b);
        true
    }

    /// Length in blocks.
    #[must_use]
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    /// True if no blocks yet.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// Longest-chain rule: pick the longer of two candidate chains.
    /// Ties broken by keeping `self`.
    #[must_use]
    pub fn longest<'a>(a: &'a Chain, b: &'a Chain) -> &'a Chain {
        if b.len() > a.len() { b } else { a }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_block(parent: Hash, hash_byte: u8) -> Block {
        Block {
            header: BlockHeader {
                parent,
                merkle_root: [0u8; 32],
                timestamp: 0,
                target: u64::MAX,
                nonce: 0,
            },
            hash: [hash_byte; 32],
            txs: vec![],
        }
    }

    #[test]
    fn three_block_chain_extends() {
        let mut chain = Chain::new();
        let b1 = mk_block(GENESIS_PARENT, 1);
        let b2 = mk_block([1u8; 32], 2);
        let b3 = mk_block([2u8; 32], 3);
        assert!(chain.append(b1));
        assert!(chain.append(b2));
        assert!(chain.append(b3));
        assert_eq!(chain.len(), 3);
    }

    #[test]
    fn rejects_wrong_parent() {
        let mut chain = Chain::new();
        let b1 = mk_block(GENESIS_PARENT, 1);
        assert!(chain.append(b1));
        let bad = mk_block([99u8; 32], 2);
        assert!(!chain.append(bad));
        assert_eq!(chain.len(), 1);
    }

    #[test]
    fn longest_chain_rule_picks_longer() {
        let mut short = Chain::new();
        short.append(mk_block(GENESIS_PARENT, 1));

        let mut long = Chain::new();
        long.append(mk_block(GENESIS_PARENT, 1));
        long.append(mk_block([1u8; 32], 2));

        assert_eq!(Chain::longest(&short, &long).len(), 2);
        assert_eq!(Chain::longest(&long, &short).len(), 2);
    }
}
