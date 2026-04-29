//! Module 0106: Casper FFG checkpoint justification and
//! finalisation.

#![warn(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

/// Checkpoint identifier (monotone height).
pub type Height = u64;

/// FFG vote: link from `source` to `target` checkpoint by
/// `staker`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vote {
    /// Source checkpoint height.
    pub source: Height,
    /// Target checkpoint height.
    pub target: Height,
    /// Staker (validator) id.
    pub staker: u64,
}

/// FFG state: tracks per-link votes and computes justification
/// and finalisation.
#[derive(Clone, Debug, Default)]
pub struct Ffg {
    /// Total stake (uniform 1 per validator in this model).
    pub total: u64,
    /// `(source, target) -> set of voters`.
    pub votes: BTreeMap<(Height, Height), BTreeSet<u64>>,
    /// Justified checkpoint heights (genesis = 0 by default).
    pub justified: BTreeSet<Height>,
    /// Finalised checkpoint heights.
    pub finalised: BTreeSet<Height>,
}

impl Ffg {
    /// Build state with `total` stake and genesis at height 0.
    #[must_use]
    pub fn new(total: u64) -> Self {
        let mut justified = BTreeSet::new();
        let mut finalised = BTreeSet::new();
        justified.insert(0);
        finalised.insert(0);
        Self {
            total,
            votes: BTreeMap::new(),
            justified,
            finalised,
        }
    }

    /// Record a vote and update justification/finalisation.
    pub fn record(&mut self, v: &Vote) {
        if !self.justified.contains(&v.source) {
            return;
        }
        let voters = self.votes.entry((v.source, v.target)).or_default();
        voters.insert(v.staker);
        let stake = voters.len() as u64;
        if 3 * stake >= 2 * self.total
            && self.justified.insert(v.target)
            && v.target == v.source + 1
        {
            self.finalised.insert(v.source);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn justification_at_two_thirds_plus_one() {
        let mut f = Ffg::new(3);
        f.record(&Vote {
            source: 0,
            target: 1,
            staker: 1,
        });
        assert!(!f.justified.contains(&1));
        f.record(&Vote {
            source: 0,
            target: 1,
            staker: 2,
        });
        assert!(f.justified.contains(&1));
    }

    #[test]
    fn finalisation_via_consecutive_justification() {
        let mut f = Ffg::new(3);
        // Justify 1 from genesis 0.
        f.record(&Vote {
            source: 0,
            target: 1,
            staker: 1,
        });
        f.record(&Vote {
            source: 0,
            target: 1,
            staker: 2,
        });
        // Justify 2 from 1; should also finalise 1.
        f.record(&Vote {
            source: 1,
            target: 2,
            staker: 1,
        });
        f.record(&Vote {
            source: 1,
            target: 2,
            staker: 2,
        });
        assert!(f.justified.contains(&2));
        assert!(f.finalised.contains(&1));
    }

    #[test]
    fn vote_from_unjustified_source_is_dropped() {
        let mut f = Ffg::new(3);
        f.record(&Vote {
            source: 99,
            target: 100,
            staker: 1,
        });
        assert!(!f.justified.contains(&100));
    }
}
