//! Module 0111: Ouroboros Praos slot eligibility.
//!
//! Computes whether a stakeholder is eligible to lead a slot
//! based on a deterministic pseudo-`VRF` and a stake-weighted
//! threshold derived from the active slot coefficient `f`.

#![warn(missing_docs)]

/// Stakeholder id.
pub type StakeholderId = u64;

/// Pseudo-`VRF`: deterministic hash-based map.
fn pseudo_vrf(slot: u64, id: StakeholderId, seed: u64) -> u64 {
    let mut x = slot.wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ id.wrapping_mul(0xbf58_476d_1ce4_e5b9)
        ^ seed.wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^= x >> 31;
    x
}

/// True if stakeholder is eligible to lead `slot`. The
/// eligibility threshold scales with `stake_fraction_x_n` (a
/// stake fraction expressed as numerator over `1_000_000`) and
/// the active slot coefficient `f_per_million` (e.g.,
/// `50_000` for `f = 1/20`).
#[must_use]
pub fn praos_eligible(
    slot: u64,
    id: StakeholderId,
    seed: u64,
    stake_fraction_x_n: u64,
    f_per_million: u64,
) -> bool {
    let stake_fraction_x_n = stake_fraction_x_n.min(1_000_000);
    let threshold_num = stake_fraction_x_n.saturating_mul(f_per_million);
    let threshold_den: u128 = 1_000_000_000_000;
    let h = pseudo_vrf(slot, id, seed);
    let h128 = u128::from(h);
    let max128 = u128::from(u64::MAX);
    h128 * threshold_den < max128 * u128::from(threshold_num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_stake_high_f_always_eligible() {
        // stake = 100%, f = 100%: always eligible.
        for slot in 0..50 {
            assert!(praos_eligible(slot, 1, 7, 1_000_000, 1_000_000));
        }
    }

    #[test]
    fn zero_stake_never_eligible() {
        for slot in 0..50 {
            assert!(!praos_eligible(slot, 1, 7, 0, 50_000));
        }
    }

    #[test]
    fn distribution_proportional_to_stake() {
        // Two stakeholders, 80/20; f = 100%.
        let mut a_count = 0;
        let mut b_count = 0;
        for slot in 0..1000 {
            if praos_eligible(slot, 1, 7, 800_000, 1_000_000) {
                a_count += 1;
            }
            if praos_eligible(slot, 2, 7, 200_000, 1_000_000) {
                b_count += 1;
            }
        }
        // a_count should be ~ 4 * b_count.
        assert!(a_count > 3 * b_count);
        assert!(a_count < 5 * b_count);
    }
}
