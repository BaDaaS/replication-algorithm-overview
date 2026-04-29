//! Module 0054: BFT-SMaRt -- conceptual placeholder.
//!
//! BFT-SMaRt's protocol is structurally PBFT (module 0042)
//! plus engineering. This crate exposes the modular component
//! types only.

#![warn(missing_docs)]

/// BFT-SMaRt's pluggable component types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Component {
    /// Authentication module (MAC vs signature).
    Auth,
    /// Network module (TCP, UDP, etc.).
    Network,
    /// State-transfer module.
    StateTransfer,
    /// Recovery module.
    Recovery,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn components_distinct() {
        assert_ne!(Component::Auth, Component::Network);
    }
}
