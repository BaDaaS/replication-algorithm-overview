//! Module 0050: Steward hierarchical BFT helpers.

#![warn(missing_docs)]

/// Total replicas in a Steward deployment: sum of per-site
/// replica counts.
#[must_use]
pub fn total_replicas(per_site: usize, sites: usize) -> usize {
    per_site * sites
}

/// Local quorum (BFT) per site.
#[must_use]
pub fn local_quorum(n_local: usize, f_local: usize) -> usize {
    let _ = n_local;
    2 * f_local + 1
}

/// Global quorum (crash-fault Paxos across sites).
#[must_use]
pub fn global_quorum(sites: usize) -> usize {
    sites / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steward_3sites_4replicas() {
        assert_eq!(total_replicas(4, 3), 12);
        assert_eq!(local_quorum(4, 1), 3);
        assert_eq!(global_quorum(3), 2);
    }
}
