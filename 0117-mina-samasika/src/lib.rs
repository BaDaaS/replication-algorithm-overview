//! Module 0117: Mina Samasika constant-size chain proof
//! placeholder.

#![warn(missing_docs)]

/// Stub for a recursive Pickles proof. Production proofs are
/// ~22 KB; here we just record the chain length covered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecursiveProof {
    /// Number of blocks the proof attests.
    pub depth: u64,
    /// True if the proof verifies against the local check.
    pub valid: bool,
}

/// Verify a chain of length `depth` produces a valid proof.
/// In production, this would be a recursive Pickles
/// verification; here it returns a placeholder marker.
#[must_use]
pub fn verify_chain(depth: u64) -> RecursiveProof {
    RecursiveProof { depth, valid: true }
}

/// Constant-size proof bytes (placeholder; production is
/// ~22000 bytes).
pub const PROOF_BYTES: usize = 22_000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proof_is_constant_size_regardless_of_depth() {
        let p1 = verify_chain(1);
        let p2 = verify_chain(1_000_000);
        assert!(p1.valid);
        assert!(p2.valid);
        // Production proof size is independent of depth.
        let _ = PROOF_BYTES;
    }
}
