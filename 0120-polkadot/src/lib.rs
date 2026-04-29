//! Module 0120: Polkadot BABE + GRANDPA + BEEFY skeletons.

#![warn(missing_docs)]

use std::collections::BTreeSet;

/// `BABE` slot leader assignment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BabeLeader {
    /// Slot number.
    pub slot: u64,
    /// Leader id (after `VRF` lottery).
    pub leader: u64,
}

/// `GRANDPA` chain finaliser: when 2/3 of validators vote
/// for the highest block, the chain is finalised up to that
/// block.
#[derive(Clone, Debug, Default)]
pub struct Grandpa {
    /// Total validators.
    pub n: usize,
    /// Validators who voted for `target_block`.
    pub voters: BTreeSet<u64>,
    /// Voted-on target.
    pub target_block: u64,
}

impl Grandpa {
    /// Build with `n` validators voting for `target_block`.
    #[must_use]
    pub fn new(n: usize, target_block: u64) -> Self {
        Self {
            n,
            voters: BTreeSet::new(),
            target_block,
        }
    }

    /// Record a vote.
    pub fn vote(&mut self, validator: u64) {
        self.voters.insert(validator);
    }

    /// True if 2/3 of validators voted; chain finalised up to
    /// `target_block`.
    #[must_use]
    pub fn finalised(&self) -> bool {
        3 * self.voters.len() >= 2 * self.n
    }
}

/// `BEEFY` succinct commitment: a `MMR` root signed by
/// validators.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BeefyCommitment {
    /// Block number.
    pub block: u64,
    /// `MMR` root over chain state.
    pub mmr_root: [u8; 32],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn babe_leader_records_slot_and_id() {
        let bl = BabeLeader {
            slot: 100,
            leader: 7,
        };
        assert_eq!(bl.slot, 100);
        assert_eq!(bl.leader, 7);
    }

    #[test]
    fn grandpa_finalises_at_two_thirds() {
        let mut g = Grandpa::new(3, 100);
        g.vote(1);
        assert!(!g.finalised());
        g.vote(2);
        assert!(g.finalised());
    }

    #[test]
    fn beefy_commitment_holds_mmr_root() {
        let c = BeefyCommitment {
            block: 12345,
            mmr_root: [42u8; 32],
        };
        assert_eq!(c.block, 12345);
        assert_eq!(c.mmr_root[0], 42);
    }
}
