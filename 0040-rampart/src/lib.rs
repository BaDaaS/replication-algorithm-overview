//! Module 0040: Rampart -- conceptual placeholder.
//!
//! Rampart's atomic broadcast is structurally similar to PBFT
//! (module 0042). This crate exposes only the layering type.

#![warn(missing_docs)]

/// Rampart's three-layer architecture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    /// Group communication (reliable, atomic, causal broadcast).
    GroupCommunication,
    /// Membership service.
    Membership,
    /// Replication service.
    Replication,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layers_distinct() {
        assert_ne!(Layer::GroupCommunication, Layer::Membership);
    }
}
