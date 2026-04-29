//! Module 0018: pedagogical common-coin oracles for use by ABA
//! modules.
//!
//! Each function is a deterministic, reproducible placeholder for
//! a real construction; the README discusses the cryptographic
//! requirements that production deployments must meet.

#![warn(missing_docs)]

/// Helper: hash a round number to a single bit.
fn hash_round_to_bit(round: u32, salt: &[u8]) -> bool {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    salt.hash(&mut h);
    round.hash(&mut h);
    h.finish() & 1 == 1
}

/// Threshold-BLS coin (simulated).
pub fn threshold_bls_coin(round: u32) -> bool {
    hash_round_to_bit(round, b"threshold-bls")
}

/// VRF-based coin (simulated as a deterministic function).
pub fn vrf_coin(round: u32) -> bool {
    hash_round_to_bit(round, b"vrf")
}

/// VDF-based coin (simulated).
pub fn vdf_coin(round: u32) -> bool {
    hash_round_to_bit(round, b"vdf")
}

/// drand beacon (simulated).
pub fn drand_coin(round: u32) -> bool {
    hash_round_to_bit(round, b"drand")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coins_are_deterministic() {
        assert_eq!(threshold_bls_coin(1), threshold_bls_coin(1));
        assert_eq!(vrf_coin(1), vrf_coin(1));
        assert_eq!(vdf_coin(1), vdf_coin(1));
        assert_eq!(drand_coin(1), drand_coin(1));
    }

    #[test]
    fn coins_distinguish_rounds() {
        // Probabilistic: with 4 round-pairs, expect at least one
        // distinct.
        let pairs = [(1, 2), (3, 4), (5, 6), (7, 8)];
        let mut diff = 0;
        for (a, b) in pairs {
            if threshold_bls_coin(a) != threshold_bls_coin(b) {
                diff += 1;
            }
        }
        assert!(diff >= 1, "all 4 round-pairs gave the same coin");
    }

    #[test]
    fn different_constructions_can_disagree() {
        // The four constructions use different salts and so
        // their outputs are independent; we just check they
        // exist as distinct functions.
        let r = 7;
        let _ = threshold_bls_coin(r);
        let _ = vrf_coin(r);
        let _ = vdf_coin(r);
        let _ = drand_coin(r);
    }
}
