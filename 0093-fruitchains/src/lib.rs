//! Module 0093: `FruitChains` inclusion-window helper.
//!
//! Models the relationship between fruits and their stem
//! blocks, checking whether a fruit is recorded within `kappa`
//! blocks of its stem on the canonical chain.

#![warn(missing_docs)]

/// Block height (chain index).
pub type Height = u64;

/// A fruit references a stem block by height.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fruit {
    /// Fruit identifier.
    pub id: u64,
    /// Height of the stem block.
    pub stem: Height,
}

/// True if a fruit with stem `s` is recorded in a block at
/// height `h`, given inclusion window `kappa`.
///
/// A fruit is recorded if `s <= h` and `h - s <= kappa`.
#[must_use]
pub fn is_recorded(fruit: &Fruit, included_in: Height, kappa: u64) -> bool {
    included_in >= fruit.stem && included_in - fruit.stem <= kappa
}

/// Compute reward share given total fruits, fruits mined by
/// the miner, and the per-fruit reward (uniform).
#[must_use]
pub fn miner_reward(
    total_fruits: u64,
    mine_fruits: u64,
    per_fruit: u64,
) -> u64 {
    if total_fruits == 0 {
        return 0;
    }
    mine_fruits * per_fruit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fruit_recorded_within_window() {
        let f = Fruit { id: 1, stem: 10 };
        assert!(is_recorded(&f, 10, 5));
        assert!(is_recorded(&f, 15, 5));
        assert!(!is_recorded(&f, 16, 5));
    }

    #[test]
    fn fruit_with_future_stem_is_not_recorded() {
        let f = Fruit { id: 1, stem: 20 };
        assert!(!is_recorded(&f, 10, 5));
    }

    #[test]
    fn proportional_reward() {
        // 100 fruits total, miner has 30, per-fruit reward 10.
        assert_eq!(miner_reward(100, 30, 10), 300);
    }
}
