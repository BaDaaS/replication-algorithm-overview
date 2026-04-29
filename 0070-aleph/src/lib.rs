//! Module 0070: Aleph round-DAG types.

#![warn(missing_docs)]

/// Aleph "unit": a per-round block referencing n - f parents.
#[derive(Clone, Debug)]
pub struct Unit {
    /// Round number.
    pub round: u32,
    /// Creator id.
    pub creator: u32,
    /// Parent unit hashes (n - f from the previous round).
    pub parents: Vec<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_construction() {
        let u = Unit {
            round: 1,
            creator: 0,
            parents: vec![1, 2, 3],
        };
        assert_eq!(u.round, 1);
        assert_eq!(u.parents.len(), 3);
    }
}
