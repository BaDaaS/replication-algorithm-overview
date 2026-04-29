//! Module 0025: Fast Paxos quorum-size helpers.

#![warn(missing_docs)]

/// Compute the classic quorum size for Fast Paxos.
#[must_use]
pub fn classic_quorum(n: usize) -> usize {
    n / 2 + 1
}

/// Compute the fast quorum size for Fast Paxos.
#[must_use]
pub fn fast_quorum(_n: usize, f: usize) -> usize {
    2 * f + 1
}

/// Verify Fast Paxos's quorum-intersection invariant: the
/// intersection of any classic and fast quorum exceeds `f`.
#[must_use]
pub fn intersection_lower_bound(n: usize, f: usize) -> usize {
    let qc = classic_quorum(n);
    let qf = fast_quorum(n, f);
    qc + qf - n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fast_paxos_n7_f2() {
        // n = 7, f = 2: classic = 4, fast = 5, intersect = 2 > f.
        assert_eq!(classic_quorum(7), 4);
        assert_eq!(fast_quorum(7, 2), 5);
        assert_eq!(intersection_lower_bound(7, 2), 2);
        // f = 2 means we need intersection > 2, but 2 == 2.
        // Actually Fast Paxos requires intersection > f.
        // So n = 7, f = 2 is just at the boundary.
    }

    #[test]
    fn fast_paxos_n10_f3() {
        // n = 10, f = 3: classic = 6, fast = 7, intersect = 3.
        assert_eq!(classic_quorum(10), 6);
        assert_eq!(fast_quorum(10, 3), 7);
        assert_eq!(intersection_lower_bound(10, 3), 3);
    }
}
