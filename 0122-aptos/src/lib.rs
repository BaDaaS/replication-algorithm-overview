//! Module 0122: Aptos `Quorum-store` batching + `Block-STM`
//! conflict-detection skeleton.

#![warn(missing_docs)]

use std::collections::BTreeSet;

/// Quorum-store batch: a digest of accepted transactions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuorumStoreBatch {
    /// Batch identifier.
    pub id: u64,
    /// Transactions (placeholder).
    pub txs: Vec<u64>,
}

/// Result of speculative execution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecResult {
    /// Transactions that executed without conflict.
    pub committed: Vec<u64>,
    /// Transactions that need re-execution.
    pub re_run: Vec<u64>,
}

/// Block-STM stand-in: detect transactions that touch the same
/// key (modeled by a touched-set per tx) and need
/// re-execution.
#[must_use]
pub fn block_stm_run(txs: &[(u64, BTreeSet<u64>)]) -> ExecResult {
    let mut seen: BTreeSet<u64> = BTreeSet::new();
    let mut committed = Vec::new();
    let mut re_run = Vec::new();
    for (tx_id, touched) in txs {
        if touched.is_disjoint(&seen) {
            committed.push(*tx_id);
            seen.extend(touched.iter().copied());
        } else {
            re_run.push(*tx_id);
        }
    }
    ExecResult { committed, re_run }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(items: &[u64]) -> BTreeSet<u64> {
        items.iter().copied().collect()
    }

    #[test]
    fn disjoint_writes_all_commit() {
        let txs = vec![(1, s(&[10])), (2, s(&[20])), (3, s(&[30]))];
        let r = block_stm_run(&txs);
        assert_eq!(r.committed, vec![1, 2, 3]);
        assert!(r.re_run.is_empty());
    }

    #[test]
    fn conflicting_writes_trigger_rerun() {
        let txs = vec![(1, s(&[10])), (2, s(&[10, 20]))];
        let r = block_stm_run(&txs);
        assert_eq!(r.committed, vec![1]);
        assert_eq!(r.re_run, vec![2]);
    }

    #[test]
    fn batch_records_id_and_txs() {
        let b = QuorumStoreBatch {
            id: 7,
            txs: vec![1, 2, 3],
        };
        assert_eq!(b.id, 7);
        assert_eq!(b.txs.len(), 3);
    }
}
