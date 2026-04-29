//! Module 0135: Capstone open problems summary.

#![warn(missing_docs)]

/// Number of modules in this course.
pub const COURSE_SIZE: usize = 136;

/// The four pillars of the course.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pillar {
    /// Theoretical foundations.
    Theory,
    /// Practical engineering.
    Practice,
    /// Formal verification.
    Formalisation,
    /// Verifiability and SNARK encoding.
    Verifiability,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn course_size_is_consistent() {
        assert_eq!(COURSE_SIZE, 136);
    }

    #[test]
    fn pillars_are_distinct() {
        assert_ne!(Pillar::Theory, Pillar::Practice);
        assert_ne!(Pillar::Formalisation, Pillar::Verifiability);
    }
}
