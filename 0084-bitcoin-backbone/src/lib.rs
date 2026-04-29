//! Module 0084: Bitcoin Backbone Protocol property checkers
//! (Garay-Kiayias-Leonardos 2015).
//!
//! Implements simple checkers for the three core properties:
//! common prefix, chain quality, chain growth, operating on
//! abstract block sequences (each block annotated as honest or
//! adversarial).

#![warn(missing_docs)]

/// A block in the abstract backbone chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AbstractBlock {
    /// Block id (monotonically increasing).
    pub id: u64,
    /// True if mined by an honest party.
    pub honest: bool,
    /// Round in which the block was produced.
    pub round: u64,
}

/// Common-prefix check: drop the last `k` blocks of `c1` and
/// require it to be a prefix of `c2`.
#[must_use]
pub fn common_prefix(
    c1: &[AbstractBlock],
    c2: &[AbstractBlock],
    k: usize,
) -> bool {
    if c1.len() < k {
        return true;
    }
    let trimmed = &c1[..c1.len() - k];
    if trimmed.len() > c2.len() {
        return false;
    }
    trimmed
        .iter()
        .zip(c2.iter())
        .all(|(a, b)| a.id == b.id && a.honest == b.honest)
}

/// Chain-quality check: in any window of `l` consecutive blocks,
/// at least `mu_num / mu_den` fraction are honest.
#[must_use]
pub fn chain_quality(
    chain: &[AbstractBlock],
    l: usize,
    mu_num: u64,
    mu_den: u64,
) -> bool {
    if chain.len() < l || l == 0 {
        return true;
    }
    let threshold = (l as u64 * mu_num).div_ceil(mu_den);
    chain.windows(l).all(|w| {
        let honest_count = w.iter().filter(|b| b.honest).count() as u64;
        honest_count >= threshold
    })
}

/// Chain-growth check: across rounds `r_start..=r_end`, the chain
/// grows by at least `tau_num / tau_den * (r_end - r_start + 1)`
/// blocks.
#[must_use]
pub fn chain_growth(
    chain: &[AbstractBlock],
    r_start: u64,
    r_end: u64,
    tau_num: u64,
    tau_den: u64,
) -> bool {
    if r_end < r_start {
        return true;
    }
    let span = r_end - r_start + 1;
    let grew = chain
        .iter()
        .filter(|b| b.round >= r_start && b.round <= r_end)
        .count() as u64;
    let need = (span * tau_num).div_ceil(tau_den);
    grew >= need
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(id: u64, honest: bool, round: u64) -> AbstractBlock {
        AbstractBlock { id, honest, round }
    }

    #[test]
    fn common_prefix_holds_for_aligned_chains() {
        let c1 = vec![b(1, true, 1), b(2, true, 2), b(3, true, 3)];
        let c2 =
            vec![b(1, true, 1), b(2, true, 2), b(3, true, 3), b(4, true, 4)];
        assert!(common_prefix(&c1, &c2, 1));
    }

    #[test]
    fn common_prefix_fails_on_fork() {
        let c1 = vec![b(1, true, 1), b(2, true, 2), b(3, false, 3)];
        let c2 = vec![b(1, true, 1), b(2, true, 2), b(99, true, 3)];
        assert!(!common_prefix(&c1, &c2, 0));
    }

    #[test]
    fn chain_quality_passes_when_majority_honest() {
        let chain = vec![
            b(1, true, 1),
            b(2, true, 2),
            b(3, false, 3),
            b(4, true, 4),
            b(5, true, 5),
        ];
        assert!(chain_quality(&chain, 3, 1, 2));
    }

    #[test]
    fn chain_growth_passes_when_blocks_per_round_sufficient() {
        let chain = vec![b(1, true, 1), b(2, true, 2), b(3, true, 3)];
        assert!(chain_growth(&chain, 1, 3, 1, 1));
        assert!(!chain_growth(&chain, 1, 3, 2, 1));
    }
}
