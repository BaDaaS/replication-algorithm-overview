//! Module 0123: Sui Lutris + Mysticeti routing skeleton.

#![warn(missing_docs)]

/// Path Sui can use for a transaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Path {
    /// Owner-only transactions; signed but not totally ordered.
    SingleOwner,
    /// Shared-object transactions; routed through Mysticeti.
    SharedObject,
}

/// Transaction type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tx {
    /// Touches only owner-private objects.
    SingleOwner {
        /// Owner public key.
        owner: u64,
        /// Object ids referenced.
        objects: Vec<u64>,
    },
    /// Touches one or more shared objects.
    SharedObject {
        /// Object ids referenced.
        objects: Vec<u64>,
    },
}

/// Route a transaction to the appropriate consensus path.
#[must_use]
pub fn route(tx: &Tx) -> Path {
    match tx {
        Tx::SingleOwner { .. } => Path::SingleOwner,
        Tx::SharedObject { .. } => Path::SharedObject,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_owner_uses_fast_path() {
        let t = Tx::SingleOwner {
            owner: 1,
            objects: vec![10, 11],
        };
        assert_eq!(route(&t), Path::SingleOwner);
    }

    #[test]
    fn shared_object_uses_mysticeti() {
        let t = Tx::SharedObject { objects: vec![100] };
        assert_eq!(route(&t), Path::SharedObject);
    }
}
