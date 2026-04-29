//! Module 0091: Solida reward-split helper.
//!
//! Splits a `PoW` block reward between the miner and the
//! committee, reflecting Solida's incentive-compatible
//! distribution.

#![warn(missing_docs)]

/// Result of splitting a block reward.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RewardSplit {
    /// Reward paid to the `PoW` miner.
    pub miner: u64,
    /// Reward paid to each committee member (uniform share).
    pub per_member: u64,
    /// Total committee reward (`per_member * committee_size`).
    pub committee_total: u64,
    /// Dust remaining due to integer rounding.
    pub dust: u64,
}

/// Compute the reward split.
///
/// `miner_share_bps` is in basis points (10000 = 100%).
#[must_use]
pub fn split_reward(
    total: u64,
    committee_size: usize,
    miner_share_bps: u64,
) -> RewardSplit {
    let miner_share_bps = miner_share_bps.min(10_000);
    let miner = total * miner_share_bps / 10_000;
    let committee_total = total - miner;
    let per_member = if committee_size == 0 {
        0
    } else {
        committee_total / committee_size as u64
    };
    let actual_committee = per_member * committee_size as u64;
    let dust = committee_total - actual_committee;
    RewardSplit {
        miner,
        per_member,
        committee_total: actual_committee,
        dust,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn miner_only_when_share_is_full() {
        let s = split_reward(1000, 5, 10_000);
        assert_eq!(s.miner, 1000);
        assert_eq!(s.committee_total, 0);
        assert_eq!(s.per_member, 0);
    }

    #[test]
    fn equal_split_among_committee_with_zero_miner_share() {
        let s = split_reward(1000, 4, 0);
        assert_eq!(s.miner, 0);
        assert_eq!(s.per_member, 250);
        assert_eq!(s.committee_total, 1000);
        assert_eq!(s.dust, 0);
    }

    #[test]
    fn fifty_fifty_split_with_seven_members_has_dust() {
        let s = split_reward(1000, 7, 5000);
        assert_eq!(s.miner, 500);
        // 500 / 7 = 71 remainder 3.
        assert_eq!(s.per_member, 71);
        assert_eq!(s.committee_total, 71 * 7);
        assert_eq!(s.dust, 500 - 71 * 7);
        assert_eq!(s.miner + s.committee_total + s.dust, 1000);
    }

    #[test]
    fn capped_share_at_100_percent() {
        let s = split_reward(1000, 5, 20_000);
        assert_eq!(s.miner, 1000);
        assert_eq!(s.committee_total, 0);
    }
}
