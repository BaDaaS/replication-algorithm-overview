//! Module 0072: Narwhal-Tusk types.

#![warn(missing_docs)]

/// Narwhal layer: mempool / certificate of availability.
#[derive(Clone, Debug)]
pub struct Certificate {
    /// Batch hash.
    pub batch_hash: u64,
    /// Round in the Narwhal DAG.
    pub round: u32,
    /// Producer.
    pub producer: u32,
}

/// Tusk wave length: 3 Narwhal rounds.
pub const TUSK_WAVE: u32 = 3;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cert_construction() {
        let c = Certificate {
            batch_hash: 0xABC,
            round: 1,
            producer: 0,
        };
        assert_eq!(c.round, 1);
    }
}
