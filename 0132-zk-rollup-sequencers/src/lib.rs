//! Module 0132: zk-rollup sequencer architectures.

#![warn(missing_docs)]

/// Different sequencer architectures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sequencer {
    /// Single trusted sequencer.
    Centralised,
    /// Sequencer set running BFT among themselves.
    RollupBft,
    /// Shared sequencer chain (Astria/Espresso style).
    Shared,
    /// L1-driven sequencing (based rollup).
    L1Based,
}

/// Latency characteristic of each sequencer architecture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Latency {
    /// Seconds (fast path).
    Fast,
    /// Roughly L1 block time (slow but maximally secure).
    Slow,
}

/// Map sequencer architecture to expected latency.
#[must_use]
pub fn latency_class(s: Sequencer) -> Latency {
    match s {
        Sequencer::Centralised | Sequencer::RollupBft | Sequencer::Shared => {
            Latency::Fast
        }
        Sequencer::L1Based => Latency::Slow,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centralised_is_fast() {
        assert_eq!(latency_class(Sequencer::Centralised), Latency::Fast);
    }

    #[test]
    fn l1_based_is_slow() {
        assert_eq!(latency_class(Sequencer::L1Based), Latency::Slow);
    }
}
