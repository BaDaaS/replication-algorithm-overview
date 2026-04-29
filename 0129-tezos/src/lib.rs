//! Module 0129: Tezos Tenderbake round status.

#![warn(missing_docs)]

/// Round status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// No quorum yet.
    Pending,
    /// 2/3 preendorsements received.
    Preendorsed,
    /// 2/3 endorsements received; block committed.
    Committed,
}

/// Tenderbake round vote tracking.
#[derive(Clone, Debug)]
pub struct TenderbakeRound {
    /// Total validators.
    pub n: usize,
    /// Distinct preendorsement signers.
    pub preendorse: usize,
    /// Distinct endorsement signers.
    pub endorse: usize,
}

impl TenderbakeRound {
    /// Build for `n` validators.
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            preendorse: 0,
            endorse: 0,
        }
    }

    /// Quorum threshold (2/3 + 1).
    #[must_use]
    pub fn quorum(&self) -> usize {
        2 * self.n / 3 + 1
    }

    /// Add a preendorsement.
    pub fn add_preendorse(&mut self) {
        if self.preendorse < self.n {
            self.preendorse += 1;
        }
    }

    /// Add an endorsement.
    pub fn add_endorse(&mut self) {
        if self.endorse < self.n {
            self.endorse += 1;
        }
    }

    /// Current status.
    #[must_use]
    pub fn status(&self) -> Status {
        let q = self.quorum();
        if self.endorse >= q {
            Status::Committed
        } else if self.preendorse >= q {
            Status::Preendorsed
        } else {
            Status::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending_until_preendorse_quorum() {
        let mut r = TenderbakeRound::new(7);
        for _ in 0..4 {
            r.add_preendorse();
        }
        assert_eq!(r.status(), Status::Pending);
    }

    #[test]
    fn preendorsed_at_two_thirds() {
        let mut r = TenderbakeRound::new(7);
        for _ in 0..5 {
            r.add_preendorse();
        }
        assert_eq!(r.status(), Status::Preendorsed);
    }

    #[test]
    fn committed_after_endorse_quorum() {
        let mut r = TenderbakeRound::new(7);
        for _ in 0..5 {
            r.add_preendorse();
        }
        for _ in 0..5 {
            r.add_endorse();
        }
        assert_eq!(r.status(), Status::Committed);
    }
}
