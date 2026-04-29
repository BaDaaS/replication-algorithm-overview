//! Module 0046: Aardvark robustness primitives -- conceptual.

#![warn(missing_docs)]

/// Performance-monitoring trigger for view change.
#[derive(Clone, Copy, Debug)]
pub struct PerfMonitor {
    /// Throughput below which to trigger a view change.
    pub threshold_ops_per_sec: u32,
    /// Window over which throughput is measured.
    pub window_ops: u32,
}

impl PerfMonitor {
    /// Returns true if the observed throughput is below the
    /// threshold.
    #[must_use]
    pub fn should_view_change(&self, observed: u32) -> bool {
        observed < self.threshold_ops_per_sec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perf_threshold_triggers() {
        let m = PerfMonitor {
            threshold_ops_per_sec: 1000,
            window_ops: 100,
        };
        assert!(m.should_view_change(500));
        assert!(!m.should_view_change(1500));
    }
}
