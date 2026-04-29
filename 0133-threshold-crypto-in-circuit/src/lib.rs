//! Module 0133: Threshold cryptography circuit-cost
//! estimates.

#![warn(missing_docs)]

/// Signature primitive type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Primitive {
    /// `BLS` aggregate over `BLS12-381`.
    BlsAggregate,
    /// Schnorr over Pasta cycle.
    SchnorrPasta,
    /// Ed25519.
    Ed25519,
    /// `ECDSA` over `secp256k1`.
    EcdsaSecp256k1,
    /// BLS-based VRF.
    BlsVrf,
}

/// Approximate circuit cost (constraint count) for verifying
/// one signature of the given primitive.
#[must_use]
pub fn circuit_cost(p: Primitive) -> u64 {
    match p {
        Primitive::BlsAggregate => 100_000,
        Primitive::SchnorrPasta => 3_000,
        Primitive::Ed25519 | Primitive::EcdsaSecp256k1 => 1_000_000,
        Primitive::BlsVrf => 50_000,
    }
}

/// True if `a` is cheaper than `b` in a SNARK circuit.
#[must_use]
pub fn cheaper(a: Primitive, b: Primitive) -> bool {
    circuit_cost(a) < circuit_cost(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schnorr_pasta_is_cheapest() {
        assert!(cheaper(Primitive::SchnorrPasta, Primitive::BlsAggregate));
        assert!(cheaper(Primitive::SchnorrPasta, Primitive::Ed25519));
        assert!(cheaper(Primitive::SchnorrPasta, Primitive::EcdsaSecp256k1));
    }

    #[test]
    fn bls_aggregate_cheaper_than_ed25519() {
        assert!(cheaper(Primitive::BlsAggregate, Primitive::Ed25519));
    }

    #[test]
    fn ed25519_and_ecdsa_are_expensive() {
        assert!(circuit_cost(Primitive::Ed25519) >= 1_000_000);
        assert!(circuit_cost(Primitive::EcdsaSecp256k1) >= 1_000_000);
    }
}
