//! Module 0011: cryptographic-primitive mocks for the course.
//!
//! These types are intentionally pedagogical, not secure. They
//! expose the *shape* of the cryptographic interface used by
//! later modules' tests so that protocol logic can be exercised
//! without committing to a concrete cryptographic library.

#![warn(missing_docs)]

use std::collections::BTreeSet;

use sha2::{Digest, Sha256};
use sim::NodeId;

/// Compute the SHA-256 digest of `bytes` and return its 32 bytes.
#[must_use]
pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let out = hasher.finalize();
    let mut buf = [0u8; 32];
    buf.copy_from_slice(&out);
    buf
}

/// A pedagogical "signature": a tag pairing the signer's id with
/// the SHA-256 hash of the signed message. Trivially forgeable;
/// adequate as a stand-in in simulator tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct MockSig {
    /// Signer.
    pub signer: NodeId,
    /// SHA-256 of the signed bytes.
    pub digest: [u8; 32],
}

/// Sign `msg` as `signer`. Returns the mock signature.
#[must_use]
pub fn mock_sign(signer: NodeId, msg: &[u8]) -> MockSig {
    MockSig {
        signer,
        digest: sha256(msg),
    }
}

/// Verify a mock signature.
#[must_use]
pub fn mock_verify(sig: &MockSig, signer: NodeId, msg: &[u8]) -> bool {
    sig.signer == signer && sig.digest == sha256(msg)
}

/// A pedagogical threshold accumulator. Record partial sigs;
/// combine succeeds once `threshold` distinct partials over the
/// same message are present.
#[derive(Clone, Debug)]
pub struct MockThresholdAccumulator {
    threshold: usize,
    partials: std::collections::BTreeMap<[u8; 32], BTreeSet<NodeId>>,
}

impl MockThresholdAccumulator {
    /// Build an accumulator requiring `threshold` partials.
    #[must_use]
    pub fn new(threshold: usize) -> Self {
        Self {
            threshold,
            partials: std::collections::BTreeMap::new(),
        }
    }

    /// Record a partial signature.
    pub fn add(&mut self, sig: MockSig) {
        self.partials
            .entry(sig.digest)
            .or_default()
            .insert(sig.signer);
    }

    /// Try to combine into a "threshold" signature for the message
    /// whose digest is `digest`. Returns `Some` iff at least
    /// `threshold` distinct signers have contributed.
    #[must_use]
    pub fn combine(&self, digest: &[u8; 32]) -> Option<MockThresholdSig> {
        let signers = self.partials.get(digest)?;
        if signers.len() >= self.threshold {
            Some(MockThresholdSig {
                digest: *digest,
                signers: signers.clone(),
            })
        } else {
            None
        }
    }
}

/// A pedagogical threshold signature: bundles the message digest
/// and the set of signers.
#[derive(Clone, Debug)]
pub struct MockThresholdSig {
    /// Hash of the signed message.
    pub digest: [u8; 32],
    /// Set of contributing signers.
    pub signers: BTreeSet<NodeId>,
}

impl MockThresholdSig {
    /// Verify against an expected message and threshold.
    #[must_use]
    pub fn verify(&self, msg: &[u8], threshold: usize) -> bool {
        sha256(msg) == self.digest && self.signers.len() >= threshold
    }
}

/// A pedagogical VRF: deterministic, public, but lets the test
/// expose a fixed "secret" so later modules can simulate sortition.
#[must_use]
pub fn mock_vrf_eval(secret: u64, x: &[u8]) -> u64 {
    let mut buf = secret.to_le_bytes().to_vec();
    buf.extend_from_slice(x);
    let h = sha256(&buf);
    u64::from_le_bytes(h[..8].try_into().expect("32 bytes"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_is_deterministic() {
        assert_eq!(sha256(b"abc"), sha256(b"abc"));
        assert_ne!(sha256(b"abc"), sha256(b"abd"));
    }

    #[test]
    fn mock_sign_verify_roundtrip() {
        let sig = mock_sign(NodeId(7), b"hello");
        assert!(mock_verify(&sig, NodeId(7), b"hello"));
        assert!(!mock_verify(&sig, NodeId(8), b"hello"));
        assert!(!mock_verify(&sig, NodeId(7), b"goodbye"));
    }

    #[test]
    fn threshold_accumulates() {
        let mut acc = MockThresholdAccumulator::new(3);
        for i in 0..5 {
            acc.add(mock_sign(NodeId(i), b"vote"));
        }
        let digest = sha256(b"vote");
        let combined = acc.combine(&digest).expect("combine ok");
        assert!(combined.verify(b"vote", 3));
        assert!(combined.verify(b"vote", 5));
        assert!(!combined.verify(b"vote", 6));
    }

    #[test]
    fn threshold_below_count_fails() {
        let mut acc = MockThresholdAccumulator::new(3);
        acc.add(mock_sign(NodeId(0), b"vote"));
        acc.add(mock_sign(NodeId(1), b"vote"));
        let digest = sha256(b"vote");
        assert!(acc.combine(&digest).is_none());
    }

    #[test]
    fn vrf_distinguishes_inputs() {
        let y1 = mock_vrf_eval(42, b"slot-1");
        let y2 = mock_vrf_eval(42, b"slot-2");
        let y3 = mock_vrf_eval(7, b"slot-1");
        assert_ne!(y1, y2);
        assert_ne!(y1, y3);
    }
}
