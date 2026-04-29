//! Module 0087: Selfish-mining revenue analysis (Eyal-Sirer 2014).
//!
//! Closed-form revenue share for the selfish-mining strategy.
//! Equation reproduced from Eyal-Sirer FC 2014.

#![warn(missing_docs)]

/// Selfish miner's long-run revenue share given hash fraction
/// `alpha` and propagation parameter `gamma`.
///
/// Returns `f(alpha, gamma)` from Eyal-Sirer 2014:
///
/// ```text
/// R = ( alpha * (1 - alpha)^2 * (4*alpha + gamma * (1 - 2*alpha))
///       - alpha^3 )
///   / ( 1 - alpha * (1 + (2 - alpha) * alpha) )
/// ```
#[must_use]
pub fn selfish_share(alpha: f64, gamma: f64) -> f64 {
    let one_minus_alpha = 1.0 - alpha;
    let numerator = alpha
        * one_minus_alpha
        * one_minus_alpha
        * (4.0 * alpha + gamma * (1.0 - 2.0 * alpha))
        - alpha * alpha * alpha;
    let denominator = 1.0 - alpha * (1.0 + (2.0 - alpha) * alpha);
    numerator / denominator
}

/// Returns true if selfish mining strictly outperforms honest
/// mining (i.e., `selfish_share(alpha, gamma) > alpha`).
#[must_use]
pub fn is_profitable(alpha: f64, gamma: f64) -> bool {
    selfish_share(alpha, gamma) > alpha
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_alpha_with_gamma_zero_is_unprofitable() {
        assert!(!is_profitable(0.20, 0.0));
        assert!(!is_profitable(0.30, 0.0));
    }

    #[test]
    fn alpha_above_one_third_is_profitable_with_gamma_zero() {
        assert!(is_profitable(0.34, 0.0));
        assert!(is_profitable(0.40, 0.0));
    }

    #[test]
    fn high_gamma_lowers_threshold() {
        assert!(is_profitable(0.20, 1.0));
        let r_low = selfish_share(0.30, 0.0);
        let r_high = selfish_share(0.30, 1.0);
        assert!(r_high > r_low);
    }
}
