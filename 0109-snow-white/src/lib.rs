//! Module 0109: Snow White snapshot-delayed slot leader.
//!
//! Holds a stake snapshot at some slot and computes the
//! eligible leader for slot `s + delta` based on the
//! snapshot.

#![warn(missing_docs)]

use std::collections::BTreeMap;

/// Validator id.
pub type ValidatorId = u64;

/// Stake snapshot: validator -> stake.
#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    /// Stake distribution at the snapshot moment.
    pub stake: BTreeMap<ValidatorId, u64>,
    /// Slot at which this snapshot is valid.
    pub slot: u64,
}

impl Snapshot {
    /// Empty snapshot at slot 0.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Total stake.
    #[must_use]
    pub fn total(&self) -> u64 {
        self.stake.values().sum()
    }

    /// Compute the slot leader for `target_slot`. Snapshot must
    /// be at slot `target_slot - delta` or earlier.
    ///
    /// Algorithm: for each validator, hash `(seed, target_slot,
    /// id)`; pick the smallest hash among validators whose
    /// hash is below their stake-proportional threshold.
    #[must_use]
    pub fn slot_leader(
        &self,
        seed: u64,
        target_slot: u64,
        delta: u64,
    ) -> Option<ValidatorId> {
        if target_slot < self.slot.saturating_add(delta) {
            return None;
        }
        let total = self.total();
        if total == 0 {
            return None;
        }
        let mut best: Option<(u64, ValidatorId)> = None;
        for (&id, &stake) in &self.stake {
            let h = hash_three(seed, target_slot, id);
            let threshold =
                u128::from(stake) * u128::from(u64::MAX) / u128::from(total);
            let threshold = u64::try_from(threshold).unwrap_or(u64::MAX);
            if h <= threshold {
                best = match best {
                    None => Some((h, id)),
                    Some((bh, bid)) => {
                        if h < bh || (h == bh && id < bid) {
                            Some((h, id))
                        } else {
                            Some((bh, bid))
                        }
                    }
                };
            }
        }
        best.map(|(_, id)| id)
    }
}

fn hash_three(seed: u64, slot: u64, id: ValidatorId) -> u64 {
    let mut x = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ slot.wrapping_mul(0xbf58_476d_1ce4_e5b9)
        ^ id.wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^= x >> 31;
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_snapshot() -> Snapshot {
        let mut s = Snapshot::new();
        s.slot = 100;
        s.stake.insert(1, 10);
        s.stake.insert(2, 10);
        s.stake.insert(3, 10);
        s
    }

    #[test]
    fn target_slot_too_close_returns_none() {
        let s = build_snapshot();
        assert!(s.slot_leader(7, 105, 10).is_none());
    }

    #[test]
    fn future_slot_returns_some_leader() {
        let s = build_snapshot();
        let l = s.slot_leader(7, 200, 10);
        assert!(l.is_some());
        let id = l.unwrap();
        assert!(matches!(id, 1..=3));
    }

    #[test]
    fn empty_snapshot_returns_none() {
        let s = Snapshot::new();
        assert!(s.slot_leader(7, 200, 10).is_none());
    }
}
