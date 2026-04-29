//! Module 0102: Avalanche Snowball preference machine.
//!
//! Tracks per-value confidence counts and a current
//! preference. Each query reports `k` sampled peers'
//! preferences; if `>= alpha` agree, increment confidence and
//! possibly switch preference.

#![warn(missing_docs)]

use std::collections::BTreeMap;

/// A discrete value in the consensus.
pub type Value = u64;

/// Snowball state.
#[derive(Clone, Debug, Default)]
pub struct Snowball {
    /// Current preference (`None` until first successful query).
    pub preference: Option<Value>,
    /// Per-value confidence counter.
    pub confidence: BTreeMap<Value, u32>,
    /// Number of successive confirmations of the preference.
    pub last_count: u32,
    /// Last value that received a successful query.
    pub last_value: Option<Value>,
}

impl Snowball {
    /// Build a fresh state.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Apply a query whose `k` answers contain `agree_count`
    /// agreements on `value`. If `agree_count >= alpha`, this
    /// is a successful query.
    pub fn query(&mut self, value: Value, agree_count: u32, alpha: u32) {
        if agree_count < alpha {
            self.last_count = 0;
            self.last_value = None;
            return;
        }
        let current = self.confidence.entry(value).or_insert(0);
        *current += 1;
        let new_conf = *current;

        match self.preference {
            None => self.preference = Some(value),
            Some(p) if p == value => {}
            Some(p) => {
                let p_conf = *self.confidence.get(&p).unwrap_or(&0);
                if new_conf > p_conf {
                    self.preference = Some(value);
                }
            }
        }

        if Some(value) == self.last_value {
            self.last_count += 1;
        } else {
            self.last_count = 1;
            self.last_value = Some(value);
        }
    }

    /// True iff the preference has been confirmed `beta` times
    /// in a row.
    #[must_use]
    pub fn is_finalised(&self, beta: u32) -> bool {
        self.last_count >= beta
            && self.preference.is_some()
            && self.preference == self.last_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convergence_to_consistent_value() {
        let mut s = Snowball::new();
        for _ in 0..5 {
            s.query(1, 15, 12);
        }
        assert_eq!(s.preference, Some(1));
        assert!(s.is_finalised(5));
        assert!(!s.is_finalised(6));
    }

    #[test]
    fn switching_preference_when_other_value_outconfides() {
        let mut s = Snowball::new();
        s.query(1, 15, 12);
        s.query(1, 15, 12);
        // Now value 1 has confidence 2.
        // Three queries for value 2 push it past.
        s.query(2, 15, 12);
        assert_eq!(s.preference, Some(1));
        s.query(2, 15, 12);
        assert_eq!(s.preference, Some(1));
        s.query(2, 15, 12);
        assert_eq!(s.preference, Some(2));
    }

    #[test]
    fn unsuccessful_query_resets_streak() {
        let mut s = Snowball::new();
        s.query(1, 15, 12);
        s.query(1, 15, 12);
        s.query(1, 5, 12); // unsuccessful
        assert_eq!(s.last_count, 0);
        assert!(!s.is_finalised(2));
    }
}
