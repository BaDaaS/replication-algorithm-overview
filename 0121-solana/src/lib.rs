//! Module 0121: Solana Proof-of-History sequential chain +
//! Tower BFT lockout helper.

#![warn(missing_docs)]

/// Sequential hash chain (Proof-of-History stand-in).
#[derive(Clone, Debug, Default)]
pub struct PohChain {
    /// Current head value.
    pub head: u64,
    /// Number of steps performed.
    pub steps: u64,
}

impl PohChain {
    /// Build a new chain rooted at `seed`.
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self {
            head: seed,
            steps: 0,
        }
    }

    /// Advance the chain by one step (`head -> hash(head)`).
    pub fn tick(&mut self) {
        let mut x = self.head;
        x ^= x >> 30;
        x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9);
        x ^= x >> 27;
        x = x.wrapping_mul(0x94d0_49bb_1331_11eb);
        x ^= x >> 31;
        self.head = x;
        self.steps += 1;
    }
}

/// Tower BFT lockout: each successive vote doubles the
/// lockout duration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TowerVote {
    /// Slot of the vote.
    pub slot: u64,
    /// Lockout duration: number of slots the validator is
    /// committed.
    pub lockout: u64,
}

/// Compute the lockout for the `k`-th confirmation
/// (1-indexed): doubles each step.
#[must_use]
pub fn tower_lockout(depth: u32) -> u64 {
    1u64 << depth.min(63)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poh_extends_deterministically() {
        let mut a = PohChain::new(7);
        let mut b = PohChain::new(7);
        for _ in 0..5 {
            a.tick();
            b.tick();
        }
        assert_eq!(a.head, b.head);
        assert_eq!(a.steps, 5);
    }

    #[test]
    fn lockout_doubles_per_depth() {
        assert_eq!(tower_lockout(0), 1);
        assert_eq!(tower_lockout(1), 2);
        assert_eq!(tower_lockout(2), 4);
        assert_eq!(tower_lockout(10), 1024);
    }

    #[test]
    fn lockout_caps_at_63_to_avoid_overflow() {
        assert_eq!(tower_lockout(100), 1u64 << 63);
    }
}
