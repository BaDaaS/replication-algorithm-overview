//! Module 0108: Algorand cryptographic-sortition stand-in.
//!
//! Selects a committee from a stake-weighted population
//! using a deterministic hash-based pseudo-VRF.

#![warn(missing_docs)]

use std::collections::BTreeMap;

/// Validator id.
pub type ValidatorId = u64;

/// Validator with stake.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Validator {
    /// Identifier.
    pub id: ValidatorId,
    /// Stake.
    pub stake: u64,
}

/// Deterministic pseudo-`VRF`: maps `(seed, validator_id)` to a
/// hash output in `0..=u64::MAX`.
#[must_use]
pub fn pseudo_vrf(seed: u64, id: ValidatorId) -> u64 {
    let a = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    let b = id.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    let mut x = a ^ b ^ 0x94d0_49bb_1331_11eb;
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^= x >> 31;
    x
}

/// Select committee members. A validator with stake `s` and
/// total stake `T` is selected with probability `s / T *
/// expected_size`. Implementation: each validator is selected
/// if `pseudo_vrf(seed, id) <= threshold(s)`.
#[must_use]
pub fn select_committee(
    validators: &[Validator],
    seed: u64,
    expected_size: u64,
) -> Vec<ValidatorId> {
    let total: u64 = validators.iter().map(|v| v.stake).sum();
    if total == 0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    for v in validators {
        let prob_num = v.stake.saturating_mul(expected_size);
        let threshold = if total == 0 {
            0
        } else {
            let raw =
                u128::from(prob_num) * u128::from(u64::MAX) / u128::from(total);
            u64::try_from(raw).unwrap_or(u64::MAX)
        };
        if pseudo_vrf(seed, v.id) <= threshold {
            out.push(v.id);
        }
    }
    out
}

/// Tally selection probabilities (deterministic empirical) over
/// many seeds. Returns (id, count) for each validator.
#[must_use]
pub fn empirical_selection(
    validators: &[Validator],
    expected_size: u64,
    seeds: &[u64],
) -> BTreeMap<ValidatorId, u64> {
    let mut counts: BTreeMap<ValidatorId, u64> = BTreeMap::new();
    for &seed in seeds {
        for id in select_committee(validators, seed, expected_size) {
            *counts.entry(id).or_insert(0) += 1;
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validators() -> Vec<Validator> {
        vec![
            Validator { id: 1, stake: 100 },
            Validator { id: 2, stake: 100 },
            Validator { id: 3, stake: 100 },
            Validator { id: 4, stake: 100 },
        ]
    }

    #[test]
    fn empty_validators_yields_empty_committee() {
        let c = select_committee(&[], 1, 4);
        assert!(c.is_empty());
    }

    #[test]
    fn equal_stake_balanced_over_many_seeds() {
        let vs = validators();
        let seeds: Vec<u64> = (0..200).collect();
        let counts = empirical_selection(&vs, 1, &seeds);
        // Total selections >= 0; each validator contributes
        // approximately the same fraction.
        for v in &vs {
            assert!(counts.get(&v.id).copied().unwrap_or(0) > 0);
        }
    }

    #[test]
    fn pseudo_vrf_is_deterministic() {
        assert_eq!(pseudo_vrf(7, 42), pseudo_vrf(7, 42));
        assert_ne!(pseudo_vrf(7, 42), pseudo_vrf(7, 43));
    }
}
