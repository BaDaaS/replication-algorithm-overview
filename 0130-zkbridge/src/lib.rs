//! Module 0130: zkBridge proof verification stub.

#![warn(missing_docs)]

/// A bridge proof: stand-in for a `zk-SNARK` over the source
/// chain's consensus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BridgeProof {
    /// Source chain id (e.g., 1 = Ethereum).
    pub source_chain: u32,
    /// Source block height being attested.
    pub source_height: u64,
    /// Hash of the attested block (placeholder).
    pub block_hash: [u8; 32],
    /// Whether the proof verifies (stand-in for `SNARK`
    /// verification).
    pub valid: bool,
}

/// Verify a bridge proof against an expected block hash.
#[must_use]
pub fn verify_bridge_proof(p: &BridgeProof, expected_hash: &[u8; 32]) -> bool {
    p.valid && &p.block_hash == expected_hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_proof_with_matching_hash_verifies() {
        let h = [42u8; 32];
        let p = BridgeProof {
            source_chain: 1,
            source_height: 100,
            block_hash: h,
            valid: true,
        };
        assert!(verify_bridge_proof(&p, &h));
    }

    #[test]
    fn invalid_proof_does_not_verify() {
        let h = [42u8; 32];
        let p = BridgeProof {
            source_chain: 1,
            source_height: 100,
            block_hash: h,
            valid: false,
        };
        assert!(!verify_bridge_proof(&p, &h));
    }

    #[test]
    fn mismatched_hash_does_not_verify() {
        let h = [42u8; 32];
        let p = BridgeProof {
            source_chain: 1,
            source_height: 100,
            block_hash: h,
            valid: true,
        };
        let other = [99u8; 32];
        assert!(!verify_bridge_proof(&p, &other));
    }
}
