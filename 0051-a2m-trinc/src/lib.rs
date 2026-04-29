//! Module 0051: trusted-counter primitive (TrInc-like).

#![warn(missing_docs)]

/// Trusted increment counter: monotone, no rewinds.
#[derive(Clone, Debug, Default)]
pub struct TrInc {
    /// Current counter value.
    pub counter: u64,
}

impl TrInc {
    /// Increment and return the new (counter, value) tuple.
    pub fn attest(&mut self, value: u64) -> (u64, u64) {
        self.counter += 1;
        (self.counter, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_monotone() {
        let mut t = TrInc::default();
        let (c1, _) = t.attest(100);
        let (c2, _) = t.attest(200);
        assert!(c2 > c1);
    }
}
