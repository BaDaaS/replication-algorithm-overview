//! Module 0064: Jolteon-Ditto mode discriminator.

#![warn(missing_docs)]

/// Network regime currently observed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Jolteon (synchronous fast path).
    Jolteon,
    /// Ditto (asynchronous fallback).
    Ditto,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn modes_distinct() {
        assert_ne!(Mode::Jolteon, Mode::Ditto);
    }
}
