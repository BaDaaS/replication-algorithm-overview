//! Module 0080: Autobahn placeholder.

#![warn(missing_docs)]

/// Autobahn consensus topology.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Topology {
    /// Linear (single-leader, like HotStuff).
    Linear,
    /// DAG (wave-anchor, like Bullshark).
    Dag,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn topology_distinct() {
        assert_ne!(Topology::Linear, Topology::Dag);
    }
}
