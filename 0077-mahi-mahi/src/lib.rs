//! Module 0077: Mahi-Mahi multi-leader stub.

#![warn(missing_docs)]

/// Number of leaders per round in Mahi-Mahi.
pub const LEADERS_PER_ROUND: u32 = 4;

#[cfg(test)]
mod tests {
    #[test]
    fn leaders() {
        assert!(super::LEADERS_PER_ROUND > 1);
    }
}
