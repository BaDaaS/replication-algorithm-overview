//! Module 0037: `TrueTime` API simulation.

#![warn(missing_docs)]

use sim::Time;

/// `TrueTime`: a clock with bounded uncertainty.
#[derive(Clone, Copy, Debug)]
pub struct TrueTime {
    /// Logical "now" (centre of the uncertainty interval).
    pub now: Time,
    /// Half-width of the uncertainty interval.
    pub epsilon: Time,
}

impl TrueTime {
    /// Build a `TrueTime` instance.
    #[must_use]
    pub fn new(now: Time, epsilon: Time) -> Self {
        Self { now, epsilon }
    }

    /// Current uncertainty interval.
    #[must_use]
    pub fn interval(&self) -> (Time, Time) {
        (
            self.now.saturating_sub(self.epsilon),
            self.now + self.epsilon,
        )
    }

    /// Returns true if `t < earliest`.
    #[must_use]
    pub fn after(&self, t: Time) -> bool {
        let (earliest, _) = self.interval();
        t < earliest
    }

    /// Returns true if `t > latest`.
    #[must_use]
    pub fn before(&self, t: Time) -> bool {
        let (_, latest) = self.interval();
        t > latest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truetime_intervals() {
        let tt = TrueTime::new(100, 7);
        assert_eq!(tt.interval(), (93, 107));
        assert!(tt.after(50));
        assert!(!tt.after(95));
        assert!(tt.before(120));
        assert!(!tt.before(105));
    }

    #[test]
    fn commit_wait_external_consistency() {
        // Transaction T_1: pick t_commit > latest = 107.
        let tt_at_commit = TrueTime::new(100, 7);
        let t_commit_1 = tt_at_commit.interval().1 + 1; // 108

        // Wait until tt.after(t_commit_1), i.e. earliest > 108.
        // Earliest = now - 7, so now > 115.
        let tt_after_wait = TrueTime::new(116, 7);
        assert!(tt_after_wait.after(t_commit_1));

        // T_2 starts after T_1's ack: pick t_commit > latest =
        // 116 + 7 = 123.
        let t_commit_2 = tt_after_wait.interval().1 + 1; // 124
        assert!(t_commit_1 < t_commit_2);
    }
}
