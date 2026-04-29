//! Module 0089: Sleepy-model snapshot.
//!
//! Tracks which registered nodes are online and which are
//! honest, and checks the Pass-Shi sleepy security predicate
//! `online_honest > online_corrupt`.

#![warn(missing_docs)]

use std::collections::BTreeMap;

/// Node identifier.
pub type NodeId = u64;

/// Pass-Shi sleepy snapshot.
#[derive(Clone, Debug, Default)]
pub struct SleepyState {
    /// Online status: id -> online?
    pub online: BTreeMap<NodeId, bool>,
    /// Honesty: id -> honest?
    pub honest: BTreeMap<NodeId, bool>,
}

impl SleepyState {
    /// Empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a node with given honesty and online status.
    pub fn register(&mut self, id: NodeId, honest: bool, online: bool) {
        self.honest.insert(id, honest);
        self.online.insert(id, online);
    }

    /// Update online status of a node. Returns false if unknown.
    pub fn set_online(&mut self, id: NodeId, online: bool) -> bool {
        self.online.insert(id, online).is_some()
    }

    /// Count of currently-online honest nodes.
    #[must_use]
    pub fn online_honest(&self) -> usize {
        self.online
            .iter()
            .filter(|&(id, &on)| on && self.honest.get(id) == Some(&true))
            .count()
    }

    /// Count of currently-online corrupt nodes.
    #[must_use]
    pub fn online_corrupt(&self) -> usize {
        self.online
            .iter()
            .filter(|&(id, &on)| on && self.honest.get(id) == Some(&false))
            .count()
    }

    /// Sleepy security predicate: online honest > online
    /// corrupt.
    #[must_use]
    pub fn sleepy_secure(&self) -> bool {
        self.online_honest() > self.online_corrupt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fully_online_with_honest_majority_is_secure() {
        let mut s = SleepyState::new();
        for i in 0..3 {
            s.register(i, true, true);
        }
        for i in 3..5 {
            s.register(i, false, true);
        }
        assert!(s.sleepy_secure());
    }

    #[test]
    fn sleeping_honest_majority_breaks_security() {
        let mut s = SleepyState::new();
        for i in 0..3 {
            s.register(i, true, true);
        }
        for i in 3..5 {
            s.register(i, false, true);
        }
        s.set_online(0, false);
        s.set_online(1, false);
        assert!(!s.sleepy_secure());
    }

    #[test]
    fn awake_minority_corrupt_remains_secure() {
        let mut s = SleepyState::new();
        s.register(0, true, true);
        s.register(1, true, true);
        s.register(2, false, false);
        assert!(s.sleepy_secure());
    }
}
