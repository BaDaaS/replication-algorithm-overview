//! Module 0026: Generalized Paxos -- a `Commutes` trait.
//!
//! The full protocol is in `EPaxos` (module 0027).

#![warn(missing_docs)]

/// Trait for state-machine commands that have a commutativity
/// relation.
pub trait Commutes {
    /// Returns true iff the two commands commute (i.e. their
    /// order does not affect the final state).
    fn commutes_with(&self, other: &Self) -> bool;
}

/// Example: per-key writes commute iff they target different
/// keys.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KvWrite {
    /// Key.
    pub key: u32,
    /// Value.
    pub value: u32,
}

impl Commutes for KvWrite {
    fn commutes_with(&self, other: &Self) -> bool {
        self.key != other.key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_keys_commute() {
        let a = KvWrite { key: 1, value: 100 };
        let b = KvWrite { key: 2, value: 200 };
        assert!(a.commutes_with(&b));
    }

    #[test]
    fn same_key_writes_conflict() {
        let a = KvWrite { key: 1, value: 100 };
        let b = KvWrite { key: 1, value: 200 };
        assert!(!a.commutes_with(&b));
    }
}
