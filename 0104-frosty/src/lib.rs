//! Module 0104: Frosty Avalanche security-bound calculator.
//!
//! Estimates the disagreement probability via Frosty's
//! exponential bound:
//! `n * exp(-c * beta * (alpha/k - 1/2)^2)`.

#![warn(missing_docs)]

/// Compute the Frosty security bound (an upper bound on the
/// probability of disagreement).
#[must_use]
pub fn disagreement_bound(n: u64, k: u32, alpha: u32, beta: u32) -> f64 {
    if k == 0 {
        return 1.0;
    }
    let alpha_frac = f64::from(alpha) / f64::from(k);
    if alpha_frac <= 0.5 {
        return 1.0;
    }
    let gap = alpha_frac - 0.5;
    let exponent = -f64::from(beta) * gap * gap;
    let exponent = exponent.max(-700.0);
    #[allow(clippy::cast_precision_loss)]
    let n_f = n as f64;
    n_f * exponent.exp()
}

/// True if the bound is below `threshold`.
#[must_use]
pub fn meets_threshold(
    n: u64,
    k: u32,
    alpha: u32,
    beta: u32,
    threshold: f64,
) -> bool {
    disagreement_bound(n, k, alpha, beta) <= threshold
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_below_half_gives_no_security() {
        assert!((disagreement_bound(1000, 20, 10, 20) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn larger_beta_decreases_bound_monotonically() {
        let b1 = disagreement_bound(1000, 20, 12, 5);
        let b2 = disagreement_bound(1000, 20, 12, 50);
        assert!(b2 < b1);
    }

    #[test]
    fn larger_alpha_decreases_bound() {
        let b1 = disagreement_bound(1000, 20, 12, 20);
        let b2 = disagreement_bound(1000, 20, 16, 20);
        assert!(b2 < b1);
    }

    #[test]
    fn zero_k_yields_trivial_bound() {
        assert!((disagreement_bound(1, 0, 0, 0) - 1.0).abs() < 1e-12);
    }
}
