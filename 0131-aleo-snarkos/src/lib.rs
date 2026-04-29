//! Module 0131: Aleo `snarkOS` SNARK-tx verification stub.

#![warn(missing_docs)]

/// `SNARK` transaction (placeholder).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnarkTx {
    /// Transaction id.
    pub id: u64,
    /// Whether the embedded `SNARK` proof is valid.
    pub valid: bool,
}

/// Verify a `SnarkTx`. Real verifier runs Marlin verification.
#[must_use]
pub fn verify_tx(tx: &SnarkTx) -> bool {
    tx.valid
}

/// Verify a batch; returns `true` iff all `txs` verify.
#[must_use]
pub fn verify_batch(txs: &[SnarkTx]) -> bool {
    txs.iter().all(verify_tx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_tx_verifies() {
        let t = SnarkTx { id: 1, valid: true };
        assert!(verify_tx(&t));
    }

    #[test]
    fn batch_with_one_invalid_fails() {
        let txs = vec![
            SnarkTx { id: 1, valid: true },
            SnarkTx {
                id: 2,
                valid: false,
            },
            SnarkTx { id: 3, valid: true },
        ];
        assert!(!verify_batch(&txs));
    }

    #[test]
    fn empty_batch_trivially_verifies() {
        assert!(verify_batch(&[]));
    }
}
