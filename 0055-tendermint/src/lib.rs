//! Module 0055: Tendermint state-machine types.

#![warn(missing_docs)]

/// Tendermint round-step state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// Awaiting / generating proposal.
    Propose,
    /// Prevote phase.
    Prevote,
    /// Precommit phase.
    Precommit,
}

/// Tendermint per-height state.
#[derive(Clone, Debug, Default)]
pub struct TendermintState {
    /// Current block height.
    pub height: u64,
    /// Current round within the height.
    pub round: u32,
    /// Current step.
    pub step: Option<Step>,
    /// Value the validator is locked on (set on first
    /// precommit at the value).
    pub locked_value: Option<u32>,
    /// Round in which the lock was set.
    pub locked_round: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_distinct() {
        assert_ne!(Step::Propose, Step::Prevote);
        assert_ne!(Step::Prevote, Step::Precommit);
    }
}
