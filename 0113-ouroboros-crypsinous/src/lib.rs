//! Module 0113: Ouroboros Crypsinous stake commitment +
//! eligibility predicate (placeholder for ZK proofs).

#![warn(missing_docs)]

/// Stake commitment: opaque hash committing to (id, stake).
pub type StakeCommitment = u64;

/// Build a deterministic commitment for a `(id, stake)` pair.
#[must_use]
pub fn commit(id: u64, stake: u64) -> StakeCommitment {
    let mut x = id.wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ stake.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 27;
    x
}

/// Pseudo-`VRF`: hash mapping `(slot, commitment)` to value.
fn vrf(slot: u64, commitment: StakeCommitment) -> u64 {
    let mut x = slot.wrapping_mul(0x94d0_49bb_1331_11eb)
        ^ commitment.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 30;
    x = x.wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^= x >> 27;
    x
}

/// True if a stakeholder with hidden stake `stake` and
/// commitment `c` is eligible for `slot` under threshold
/// scaling. Stand-in for the ZK eligibility proof.
#[must_use]
pub fn prove_eligibility(
    c: StakeCommitment,
    slot: u64,
    threshold_per_million: u64,
) -> bool {
    let h = vrf(slot, c);
    let frac = h % 1_000_000;
    frac < threshold_per_million
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commitment_is_deterministic_per_id_stake() {
        assert_eq!(commit(7, 100), commit(7, 100));
        assert_ne!(commit(7, 100), commit(7, 101));
    }

    #[test]
    fn high_threshold_passes_often() {
        let c = commit(7, 100);
        let pass = (0..1000)
            .filter(|&s| prove_eligibility(c, s, 999_999))
            .count();
        assert!(pass > 900);
    }

    #[test]
    fn zero_threshold_never_passes() {
        let c = commit(7, 100);
        for slot in 0..1000 {
            assert!(!prove_eligibility(c, slot, 0));
        }
    }
}
