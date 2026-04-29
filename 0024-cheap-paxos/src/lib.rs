//! Module 0024: Cheap Paxos -- a thin wrapper marking acceptors
//! as main vs auxiliary. The actual protocol is Multi-Paxos
//! (module 0023); this crate exposes only the role distinction.

#![warn(missing_docs)]

use sim::NodeId;

/// Acceptor role.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// Always-on with stable storage.
    Main,
    /// Online only during reconfiguration; no stable storage.
    Auxiliary,
}

/// A configuration: which acceptors are main and which are
/// auxiliary.
#[derive(Clone, Debug)]
pub struct Config {
    /// Main acceptors.
    pub main: Vec<NodeId>,
    /// Auxiliary acceptors.
    pub auxiliary: Vec<NodeId>,
}

impl Config {
    /// Total acceptors `n = main + auxiliary`.
    #[must_use]
    pub fn n(&self) -> usize {
        self.main.len() + self.auxiliary.len()
    }

    /// Quorum threshold (`f + 1`).
    #[must_use]
    pub fn quorum(&self) -> usize {
        self.n() / 2 + 1
    }

    /// Is this configuration valid (main count >= quorum)?
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.main.len() >= self.quorum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cheap_paxos_config_3_2() {
        // n = 5, f = 2. Main = 3 (= f + 1), aux = 2.
        let cfg = Config {
            main: vec![NodeId(0), NodeId(1), NodeId(2)],
            auxiliary: vec![NodeId(3), NodeId(4)],
        };
        assert_eq!(cfg.n(), 5);
        assert_eq!(cfg.quorum(), 3);
        assert!(cfg.is_valid());
    }

    #[test]
    fn invalid_when_main_under_quorum() {
        // Only 2 main in n = 5: under quorum.
        let cfg = Config {
            main: vec![NodeId(0), NodeId(1)],
            auxiliary: vec![NodeId(2), NodeId(3), NodeId(4)],
        };
        assert!(!cfg.is_valid());
    }
}
