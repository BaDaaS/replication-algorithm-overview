//! Module 0056: `HotStuff` types -- block + quorum certificate.

#![warn(missing_docs)]

/// Quorum certificate: aggregated 2f + 1 votes on a block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QuorumCert {
    /// Height of the certified block.
    pub height: u32,
    /// Hash of the certified block (placeholder u64).
    pub block_hash: u64,
}

/// `HotStuff` block.
#[derive(Clone, Debug)]
pub struct Block {
    /// Parent block hash.
    pub parent: u64,
    /// QC over the parent.
    pub qc: QuorumCert,
    /// Block payload (placeholder).
    pub payload: Vec<u32>,
    /// Height.
    pub height: u32,
}

/// `HotStuff` phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// Prepare.
    Prepare,
    /// Pre-commit.
    PreCommit,
    /// Commit.
    Commit,
    /// Decide.
    Decide,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phases_distinct() {
        assert_ne!(Phase::Prepare, Phase::Commit);
    }

    #[test]
    fn block_construction() {
        let qc = QuorumCert {
            height: 1,
            block_hash: 0xABC,
        };
        let b = Block {
            parent: 0xABC,
            qc,
            payload: vec![1, 2, 3],
            height: 2,
        };
        assert_eq!(b.height, 2);
        assert_eq!(b.qc.height, 1);
    }
}
