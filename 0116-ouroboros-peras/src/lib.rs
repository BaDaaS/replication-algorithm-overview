//! Module 0116: Ouroboros Peras finality status machine.
//!
//! Two-phase commit (prepare/commit) on a single block.
//! Status transitions Pending -> Prepared -> Finalised when
//! 2/3 of the committee signs each phase.

#![warn(missing_docs)]

/// Status of a block under Peras finality.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// No quorum reached yet.
    Pending,
    /// 2/3 prepare signatures collected.
    Prepared,
    /// 2/3 commit signatures collected.
    Finalised,
}

/// Tracker for prepare/commit on one candidate block.
#[derive(Clone, Debug)]
pub struct PerasState {
    /// Committee size.
    pub n: usize,
    /// Number of distinct prepare signers.
    pub prepare: usize,
    /// Number of distinct commit signers.
    pub commit: usize,
}

impl PerasState {
    /// Build a fresh state.
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            prepare: 0,
            commit: 0,
        }
    }

    /// Quorum threshold `2 * n / 3 + 1`.
    #[must_use]
    pub fn quorum(&self) -> usize {
        (2 * self.n) / 3 + 1
    }

    /// Add a prepare vote (idempotent in this stand-in).
    pub fn add_prepare(&mut self) {
        if self.prepare < self.n {
            self.prepare += 1;
        }
    }

    /// Add a commit vote.
    pub fn add_commit(&mut self) {
        if self.commit < self.n {
            self.commit += 1;
        }
    }

    /// Current status.
    #[must_use]
    pub fn status(&self) -> Status {
        let q = self.quorum();
        if self.commit >= q {
            Status::Finalised
        } else if self.prepare >= q {
            Status::Prepared
        } else {
            Status::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending_until_quorum() {
        let mut s = PerasState::new(7);
        for _ in 0..4 {
            s.add_prepare();
        }
        assert_eq!(s.status(), Status::Pending);
    }

    #[test]
    fn prepared_at_two_thirds_plus_one() {
        let mut s = PerasState::new(7);
        for _ in 0..5 {
            s.add_prepare();
        }
        assert_eq!(s.status(), Status::Prepared);
    }

    #[test]
    fn finalised_after_commit_quorum() {
        let mut s = PerasState::new(7);
        for _ in 0..5 {
            s.add_prepare();
        }
        for _ in 0..5 {
            s.add_commit();
        }
        assert_eq!(s.status(), Status::Finalised);
    }
}
