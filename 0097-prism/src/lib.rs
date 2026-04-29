//! Module 0097: Prism three-block-type ledger.
//!
//! Models the three Prism block types (proposer, voter,
//! transaction) and a `Ledger` that stores them per role.

#![warn(missing_docs)]

/// Proposer block: orders content, low-rate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProposerBlock {
    /// Index in the proposer chain.
    pub height: u64,
    /// Hashes of transaction blocks referenced.
    pub tx_refs: Vec<u64>,
}

/// Voter block: votes on a proposer block in one of `m` voter
/// chains.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VoterBlock {
    /// Voter chain index in `0..m`.
    pub chain_idx: u32,
    /// Position within this voter chain.
    pub height: u64,
    /// Proposer block being voted for.
    pub votes_for: u64,
}

/// Transaction block: bandwidth-saturating, no `PoW` finality.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TxBlock {
    /// Block id.
    pub id: u64,
    /// Opaque transactions.
    pub txs: Vec<Vec<u8>>,
}

/// Prism ledger: parallel storage of all three block types.
#[derive(Clone, Debug, Default)]
pub struct Ledger {
    /// Proposer chain.
    pub proposer: Vec<ProposerBlock>,
    /// Voter chains: outer index = chain idx; inner = chain.
    pub voters: Vec<Vec<VoterBlock>>,
    /// Free pool of transaction blocks.
    pub tx_pool: Vec<TxBlock>,
}

impl Ledger {
    /// Empty ledger.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a proposer block.
    pub fn append_proposer(&mut self, b: ProposerBlock) {
        self.proposer.push(b);
    }

    /// Append a voter block. Grows the voter-chain vector if
    /// the index is unseen.
    pub fn append_voter(&mut self, b: VoterBlock) {
        let idx = b.chain_idx as usize;
        if self.voters.len() <= idx {
            self.voters.resize(idx + 1, Vec::new());
        }
        self.voters[idx].push(b);
    }

    /// Add a transaction block to the pool.
    pub fn add_tx_block(&mut self, b: TxBlock) {
        self.tx_pool.push(b);
    }

    /// Vote count for proposer block at height `h` across all
    /// voter chains.
    #[must_use]
    pub fn vote_count(&self, h: u64) -> usize {
        self.voters
            .iter()
            .flat_map(|chain| chain.iter())
            .filter(|v| v.votes_for == h)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_block_types_independent() {
        let mut l = Ledger::new();
        l.append_proposer(ProposerBlock {
            height: 0,
            tx_refs: vec![1, 2, 3],
        });
        l.add_tx_block(TxBlock { id: 1, txs: vec![] });
        l.append_voter(VoterBlock {
            chain_idx: 0,
            height: 0,
            votes_for: 0,
        });
        assert_eq!(l.proposer.len(), 1);
        assert_eq!(l.tx_pool.len(), 1);
        assert_eq!(l.voters[0].len(), 1);
    }

    #[test]
    fn voter_chains_grow_independently() {
        let mut l = Ledger::new();
        l.append_voter(VoterBlock {
            chain_idx: 5,
            height: 0,
            votes_for: 0,
        });
        assert_eq!(l.voters.len(), 6);
    }

    #[test]
    fn vote_count_aggregates_across_chains() {
        let mut l = Ledger::new();
        for chain in 0..3 {
            l.append_voter(VoterBlock {
                chain_idx: chain,
                height: 0,
                votes_for: 7,
            });
        }
        l.append_voter(VoterBlock {
            chain_idx: 0,
            height: 1,
            votes_for: 8,
        });
        assert_eq!(l.vote_count(7), 3);
        assert_eq!(l.vote_count(8), 1);
    }
}
