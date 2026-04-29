//! Module 0059: `HotStuff-2` two-phase types.

#![warn(missing_docs)]

/// `HotStuff-2` phase count: 2.
pub const PHASES: usize = 2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phases_two() {
        assert_eq!(PHASES, 2);
    }
}
