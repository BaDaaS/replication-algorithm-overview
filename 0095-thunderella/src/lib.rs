//! Module 0095: Thunderella optimistic-path selector.
//!
//! Models the optimistic/fallback path split. Optimistic path
//! requires a 3/4 super-quorum and a live accelerator;
//! otherwise fall back to the slow chain layer.

#![warn(missing_docs)]

/// Which path Thunderella is currently on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Path {
    /// Optimistic single-round-trip finality.
    Optimistic,
    /// Fallback: chain-rate finality.
    Fallback,
}

/// Pick the path:
/// - `Optimistic` if accelerator online AND online honest
///   `>= ceil(3 * n / 4)`.
/// - `Fallback` otherwise.
#[must_use]
pub fn pick_path(
    n: usize,
    online_honest: usize,
    accelerator_online: bool,
) -> Path {
    let super_quorum = (3 * n).div_ceil(4);
    if accelerator_online && online_honest >= super_quorum {
        Path::Optimistic
    } else {
        Path::Fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimistic_when_super_quorum_and_accelerator_live() {
        // n = 8, 3*n/4 = 6. honest = 6, accelerator on.
        assert_eq!(pick_path(8, 6, true), Path::Optimistic);
    }

    #[test]
    fn fallback_when_accelerator_offline() {
        assert_eq!(pick_path(8, 8, false), Path::Fallback);
    }

    #[test]
    fn fallback_when_below_super_quorum() {
        // 5 honest of 8 < 6 super-quorum.
        assert_eq!(pick_path(8, 5, true), Path::Fallback);
    }

    #[test]
    fn ceil_three_quarters_with_n_eq_seven_requires_six_honest() {
        // ceil(21/4) = 6.
        assert_eq!(pick_path(7, 5, true), Path::Fallback);
        assert_eq!(pick_path(7, 6, true), Path::Optimistic);
    }
}
