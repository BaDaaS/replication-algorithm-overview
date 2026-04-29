//! Module 0027: `EPaxos` quorum-size helpers.

#![warn(missing_docs)]

/// `EPaxos` fast quorum size.
#[must_use]
pub fn fast_quorum(n: usize, f: usize) -> usize {
    let _ = n;
    f + f / 2 + 1
}

/// `EPaxos` slow quorum size (majority).
#[must_use]
pub fn slow_quorum(n: usize) -> usize {
    n / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epaxos_n5_f2() {
        // n = 5, f = 2: fast = 4, slow = 3.
        assert_eq!(fast_quorum(5, 2), 4);
        assert_eq!(slow_quorum(5), 3);
    }

    #[test]
    fn epaxos_n7_f3() {
        // n = 7, f = 3: fast = 5, slow = 4.
        assert_eq!(fast_quorum(7, 3), 5);
        assert_eq!(slow_quorum(7), 4);
    }
}
