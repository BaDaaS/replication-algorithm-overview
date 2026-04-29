//! Module 0107: Casper CBC estimator skeleton.
//!
//! Provides an `Estimator` trait and a `MajorityEstimator`
//! implementation: maps a view (multiset of values) to the
//! most-frequent value.

#![warn(missing_docs)]

use std::collections::BTreeMap;

/// CBC `Estimator`: maps a view to a candidate value.
pub trait Estimator<V> {
    /// Compute the estimator's value for the given view.
    fn estimate(&self, view: &[V]) -> Option<V>;
}

/// Picks the value with the most occurrences. Ties broken by
/// the smallest value (deterministic).
pub struct MajorityEstimator;

impl<V: Clone + Ord> Estimator<V> for MajorityEstimator {
    fn estimate(&self, view: &[V]) -> Option<V> {
        if view.is_empty() {
            return None;
        }
        let mut counts: BTreeMap<V, u64> = BTreeMap::new();
        for v in view {
            *counts.entry(v.clone()).or_insert(0) += 1;
        }
        let mut best: Option<(V, u64)> = None;
        for (k, c) in counts {
            best = match best {
                None => Some((k, c)),
                Some((bk, bc)) => {
                    if c > bc || (c == bc && k < bk) {
                        Some((k, c))
                    } else {
                        Some((bk, bc))
                    }
                }
            };
        }
        best.map(|(k, _)| k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn majority_value_returned() {
        let est = MajorityEstimator;
        assert_eq!(est.estimate(&[1, 1, 1, 2, 3]), Some(1));
    }

    #[test]
    fn ties_broken_by_min_value() {
        let est = MajorityEstimator;
        assert_eq!(est.estimate(&[1, 2, 3]), Some(1));
    }

    #[test]
    fn empty_view_returns_none() {
        let est = MajorityEstimator;
        let view: [u64; 0] = [];
        assert_eq!(est.estimate(&view), None);
    }
}
