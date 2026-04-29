//! Module 0038: `CASPaxos` -- per-key compare-and-swap Paxos.

#![warn(missing_docs)]

/// CAS function: takes the current value, returns the new.
pub type CasFn = fn(u32) -> u32;

/// Per-key `CASPaxos` acceptor state.
#[derive(Clone, Copy, Debug, Default)]
pub struct CasPaxosAcceptor {
    /// Promised ballot.
    pub promised: u32,
    /// Latest accepted (ballot, value).
    pub accepted: Option<(u32, u32)>,
}

/// Result of a Phase-1 prepare: rejected, or accepted with the
/// latest known accepted (ballot, value).
#[derive(Clone, Copy, Debug)]
pub enum PrepareReply {
    /// Promised; carries the latest accepted (ballot, value) if
    /// any.
    Promised(Option<(u32, u32)>),
    /// Rejected (`b <= promised`).
    Rejected,
}

impl CasPaxosAcceptor {
    /// Phase 1: prepare for ballot `b`. Returns the current
    /// state if `b > promised`.
    pub fn prepare(&mut self, b: u32) -> PrepareReply {
        if b > self.promised {
            self.promised = b;
            PrepareReply::Promised(self.accepted)
        } else {
            PrepareReply::Rejected
        }
    }

    /// Phase 2: accept (ballot, value).
    pub fn accept(&mut self, b: u32, v: u32) -> bool {
        if b >= self.promised {
            self.promised = b;
            self.accepted = Some((b, v));
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cas_increment() {
        // Three acceptors, one proposer.
        let mut acceptors = vec![CasPaxosAcceptor::default(); 3];
        // Initial value: 0 (no accepteds).

        // Phase 1: ballot 1.
        let mut promises = 0;
        for a in &mut acceptors {
            if matches!(a.prepare(1), PrepareReply::Promised(_)) {
                promises += 1;
            }
        }
        assert!(promises >= 2);

        // Compute new value: f(0) = 1.
        let f: CasFn = |v| v + 1;
        let new_value = f(0);

        // Phase 2: accept (ballot 1, new_value).
        let mut accepteds = 0;
        for a in &mut acceptors {
            if a.accept(1, new_value) {
                accepteds += 1;
            }
        }
        assert!(accepteds >= 2);

        // Verify state.
        for a in &acceptors {
            assert_eq!(a.accepted, Some((1, 1)));
        }
    }
}
