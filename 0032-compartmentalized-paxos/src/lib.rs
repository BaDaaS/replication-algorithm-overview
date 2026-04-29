//! Module 0032: Compartmentalized Paxos -- role-type
//! definitions for the decomposition.

#![warn(missing_docs)]

/// Role of a process in Compartmentalized Paxos.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// Client.
    Client,
    /// Proposer (leader-equivalent).
    Proposer,
    /// Acceptor.
    Acceptor,
    /// Batcher: aggregates client requests.
    Batcher,
    /// Unbatcher: splits committed batches.
    Unbatcher,
    /// Replica: executes operations.
    Replica,
    /// Read-only replica.
    ReadReplica,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_eq() {
        assert_eq!(Role::Proposer, Role::Proposer);
        assert_ne!(Role::Proposer, Role::Acceptor);
    }
}
