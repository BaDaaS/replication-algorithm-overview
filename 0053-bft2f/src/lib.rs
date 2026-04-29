//! Module 0053: BFT2F fault-classification helper.

#![warn(missing_docs)]

/// BFT2F's three regimes of fault-tolerance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Regime {
    /// `f' < f + 1`: full linearisability (PBFT regime).
    Linearisable,
    /// `f + 1 <= f' < 2f + 1`: fork consistency (BFT2F).
    ForkConsistent,
    /// `f' >= 2f + 1`: no useful consistency.
    None,
}

/// Classify the regime given the number of actual Byzantine
/// faults `f_prime` and the design parameter `f`.
#[must_use]
pub fn classify(f_prime: usize, f: usize) -> Regime {
    if f_prime <= f {
        Regime::Linearisable
    } else if f_prime < 2 * f + 1 {
        Regime::ForkConsistent
    } else {
        Regime::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_regimes() {
        // f = 1, n = 4. f_prime = 1: linearisable; 2: fork; 3: none.
        assert_eq!(classify(1, 1), Regime::Linearisable);
        assert_eq!(classify(2, 1), Regime::ForkConsistent);
        assert_eq!(classify(3, 1), Regime::None);
    }
}
