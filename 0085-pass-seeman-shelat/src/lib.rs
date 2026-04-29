//! Module 0085: Pass-Seeman-Shelat 2017 bounded-delay analysis.
//!
//! Models the bounded-delay network parameter `Delta` and
//! provides numerical helpers for the PSS security bound.

#![warn(missing_docs)]

/// PSS bounded-delay parameter packet.
#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// Honest hashing fraction `alpha` in `[0, 1]`.
    pub alpha: f64,
    /// Per-unit-time mining probability `f`.
    pub f: f64,
    /// Network-delay bound `Delta`.
    pub delta_bound: f64,
}

impl Params {
    /// Compute the PSS LHS:
    /// `alpha * (1 - 2 * f * Delta)`.
    #[must_use]
    pub fn lhs(self) -> f64 {
        self.alpha * (1.0 - 2.0 * self.f * self.delta_bound)
    }

    /// Compute the PSS RHS scale: `(1 - alpha) = beta`.
    #[must_use]
    pub fn beta(self) -> f64 {
        1.0 - self.alpha
    }

    /// Returns true if the PSS security condition
    /// `alpha * (1 - 2 * f * Delta) > beta` holds.
    #[must_use]
    pub fn is_secure(self) -> bool {
        self.lhs() > self.beta()
    }

    /// Excess margin `delta` such that `lhs >= (1 + delta) * beta`.
    /// Returns `None` if not secure.
    #[must_use]
    pub fn margin(self) -> Option<f64> {
        let beta = self.beta();
        if beta <= 0.0 {
            return Some(f64::INFINITY);
        }
        let m = self.lhs() / beta - 1.0;
        if m > 0.0 { Some(m) } else { None }
    }
}

/// Predicted natural-fork rate (PSS 2017): scales as `f * Delta`.
#[must_use]
pub fn natural_fork_rate(p: Params) -> f64 {
    p.f * p.delta_bound
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    #[test]
    fn bitcoin_mainnet_is_secure_with_55_pct_honest() {
        let p = Params {
            alpha: 0.55,
            f: 1.0 / 600.0,
            delta_bound: 10.0,
        };
        assert!(p.is_secure());
        let m = p.margin().expect("secure parameters yield positive margin");
        assert!(m > 0.0 && m < 0.5);
    }

    #[test]
    fn fifty_percent_split_is_not_secure() {
        let p = Params {
            alpha: 0.50,
            f: 1.0 / 600.0,
            delta_bound: 10.0,
        };
        assert!(!p.is_secure());
        assert!(p.margin().is_none());
    }

    #[test]
    fn natural_fork_rate_matches_f_times_delta() {
        let p = Params {
            alpha: 0.55,
            f: 1.0 / 60.0,
            delta_bound: 6.0,
        };
        assert!(approx_eq(natural_fork_rate(p), 0.1, 1e-9));
    }
}
