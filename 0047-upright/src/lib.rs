//! Module 0047: `UpRight` hybrid fault tolerance helpers.

#![warn(missing_docs)]

/// `UpRight` required `n` for `u` Byzantine + `r` crash.
#[must_use]
pub fn min_n(u: usize, r: usize) -> usize {
    2 * u + r + 1
}

/// `UpRight` quorum.
#[must_use]
pub fn quorum(u: usize, r: usize) -> usize {
    u + r + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upright_u1_r2() {
        // u = 1 Byzantine + r = 2 crash: n = 5, quorum = 4.
        assert_eq!(min_n(1, 2), 5);
        assert_eq!(quorum(1, 2), 4);
    }
}
