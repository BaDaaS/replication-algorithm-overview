//! Module 0119: Cosmos / `CometBFT` per-block finaliser.
//!
//! Records precommit signatures from validators; declares
//! a block finalised when 2/3 voting power has signed.

#![warn(missing_docs)]

use std::collections::BTreeMap;

/// Validator id and voting power.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Validator {
    /// Identifier.
    pub id: u64,
    /// Voting power (e.g., delegated stake).
    pub power: u64,
}

/// `CometBFT` finaliser for one block at one height.
#[derive(Clone, Debug, Default)]
pub struct Finaliser {
    /// Validator set.
    pub validators: Vec<Validator>,
    /// Total voting power.
    pub total_power: u64,
    /// Signatures collected.
    pub signers: BTreeMap<u64, u64>,
}

impl Finaliser {
    /// Build a finaliser with the given validator set.
    #[must_use]
    pub fn new(validators: Vec<Validator>) -> Self {
        let total_power = validators.iter().map(|v| v.power).sum();
        Self {
            validators,
            total_power,
            signers: BTreeMap::new(),
        }
    }

    /// Record a precommit from validator `id`. Returns false if
    /// `id` is unknown or already signed.
    pub fn precommit(&mut self, id: u64) -> bool {
        let Some(v) = self.validators.iter().find(|v| v.id == id) else {
            return false;
        };
        self.signers.insert(id, v.power).is_none()
    }

    /// Total voting power that has precommitted.
    #[must_use]
    pub fn signed_power(&self) -> u64 {
        self.signers.values().sum()
    }

    /// True if 2/3 voting power has precommitted.
    #[must_use]
    pub fn finalised(&self) -> bool {
        self.signed_power() * 3 >= 2 * self.total_power
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn val_set() -> Vec<Validator> {
        vec![
            Validator { id: 1, power: 30 },
            Validator { id: 2, power: 30 },
            Validator { id: 3, power: 40 },
        ]
    }

    #[test]
    fn one_validator_below_quorum() {
        let mut f = Finaliser::new(val_set());
        f.precommit(1);
        assert!(!f.finalised());
    }

    #[test]
    fn two_thirds_precommit_finalises() {
        let mut f = Finaliser::new(val_set());
        f.precommit(2);
        f.precommit(3);
        // Power 70 / total 100 >= 2/3 -> finalised.
        assert!(f.finalised());
    }

    #[test]
    fn duplicate_precommit_returns_false() {
        let mut f = Finaliser::new(val_set());
        assert!(f.precommit(1));
        assert!(!f.precommit(1));
    }
}
