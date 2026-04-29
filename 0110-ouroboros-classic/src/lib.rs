//! Module 0110: Ouroboros Classic stake-weighted slot
//! leader.
//!
//! Given an epoch seed, a slot, and a stake distribution, picks
//! the single eligible leader. The selection is deterministic
//! and weighted by stake.

#![warn(missing_docs)]

use std::collections::BTreeMap;

/// Stakeholder identifier.
pub type StakeholderId = u64;

/// Pseudo-hash for deterministic selection.
fn h(seed: u64, slot: u64) -> u64 {
    let mut x = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ slot.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^= x >> 31;
    x
}

/// Stake-weighted slot leader: pick the stakeholder whose
/// cumulative stake interval contains the hash result.
#[must_use]
pub fn slot_leader(
    stake: &BTreeMap<StakeholderId, u64>,
    seed: u64,
    slot: u64,
) -> Option<StakeholderId> {
    let total: u64 = stake.values().sum();
    if total == 0 {
        return None;
    }
    let target = h(seed, slot) % total;
    let mut acc: u64 = 0;
    for (&id, &s) in stake {
        acc = acc.saturating_add(s);
        if acc > target {
            return Some(id);
        }
    }
    stake.keys().last().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_stake() -> BTreeMap<StakeholderId, u64> {
        let mut s = BTreeMap::new();
        s.insert(1, 100);
        s.insert(2, 100);
        s.insert(3, 100);
        s
    }

    #[test]
    fn deterministic_for_same_seed_and_slot() {
        let s = build_stake();
        assert_eq!(slot_leader(&s, 7, 0), slot_leader(&s, 7, 0));
    }

    #[test]
    fn empty_stake_yields_none() {
        let s = BTreeMap::new();
        assert!(slot_leader(&s, 7, 0).is_none());
    }

    #[test]
    fn balanced_distribution_over_many_slots() {
        let s = build_stake();
        let mut counts: BTreeMap<StakeholderId, u64> = BTreeMap::new();
        for slot in 0..900 {
            if let Some(id) = slot_leader(&s, 7, slot) {
                *counts.entry(id).or_insert(0) += 1;
            }
        }
        for v in counts.values() {
            // Each stakeholder gets roughly 300 slots; allow margin.
            assert!(*v > 200 && *v < 400, "got {v}");
        }
    }
}
