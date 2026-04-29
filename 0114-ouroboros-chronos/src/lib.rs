//! Module 0114: Ouroboros Chronos median-clock aggregation.
//!
//! Given a slice of signed timestamps from stakeholders,
//! computes the trimmed median to drive local clock
//! adjustment.

#![warn(missing_docs)]

/// A signed timestamp from a stakeholder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StakeholderTime {
    /// Stakeholder id.
    pub id: u64,
    /// Reported local time.
    pub time: u64,
}

/// Trimmed median: drop top/bottom `trim_pct` percent of
/// timestamps, then return the median of the remainder.
#[must_use]
pub fn median_clock(votes: &[StakeholderTime], trim_pct: u32) -> Option<u64> {
    if votes.is_empty() {
        return None;
    }
    let mut times: Vec<u64> = votes.iter().map(|v| v.time).collect();
    times.sort_unstable();
    let n = times.len();
    let trim = n.saturating_mul(trim_pct as usize) / 100;
    if 2 * trim >= n {
        return None;
    }
    let kept = &times[trim..n - trim];
    let m = kept.len();
    Some(if m % 2 == 1 {
        kept[m / 2]
    } else {
        u64::midpoint(kept[m / 2 - 1], kept[m / 2])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(id: u64, time: u64) -> StakeholderTime {
        StakeholderTime { id, time }
    }

    #[test]
    fn median_of_odd_count() {
        let votes = vec![t(1, 10), t(2, 20), t(3, 30)];
        assert_eq!(median_clock(&votes, 0), Some(20));
    }

    #[test]
    fn median_of_even_count_takes_average() {
        let votes = vec![t(1, 10), t(2, 20)];
        assert_eq!(median_clock(&votes, 0), Some(15));
    }

    #[test]
    fn trimming_drops_outliers() {
        let votes = vec![
            t(1, 1),
            t(2, 9),
            t(3, 10),
            t(4, 11),
            t(5, 12),
            t(6, 13),
            t(7, 99),
        ];
        // 14% trim drops one from each end.
        let m = median_clock(&votes, 14).unwrap();
        assert!((10..=12).contains(&m));
    }

    #[test]
    fn empty_votes_yield_none() {
        assert_eq!(median_clock(&[], 0), None);
    }

    #[test]
    fn excessive_trim_yields_none() {
        let votes = vec![t(1, 10), t(2, 20)];
        // 60% trim drops both, leaving nothing.
        assert_eq!(median_clock(&votes, 60), None);
    }
}
