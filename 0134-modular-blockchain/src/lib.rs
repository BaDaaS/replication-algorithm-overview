//! Module 0134: Modular blockchain stack composition.

#![warn(missing_docs)]

/// One of the four modular layers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    /// Execution: smart contracts, state transitions.
    Execution,
    /// Settlement: dispute resolution, finality anchor.
    Settlement,
    /// Consensus / ordering.
    Consensus,
    /// Data availability.
    Da,
}

/// A modular blockchain stack: which chain/layer implements
/// each function. Stand-in fields are provider names.
#[derive(Clone, Debug, Default)]
pub struct Stack {
    /// Execution provider.
    pub execution: String,
    /// Settlement provider.
    pub settlement: String,
    /// Consensus provider.
    pub consensus: String,
    /// `DA` provider.
    pub da: String,
}

impl Stack {
    /// True iff all four layers are populated.
    #[must_use]
    pub fn complete(&self) -> bool {
        !self.execution.is_empty()
            && !self.settlement.is_empty()
            && !self.consensus.is_empty()
            && !self.da.is_empty()
    }

    /// Returns the provider for the given layer.
    #[must_use]
    pub fn provider(&self, l: Layer) -> &str {
        match l {
            Layer::Execution => &self.execution,
            Layer::Settlement => &self.settlement,
            Layer::Consensus => &self.consensus,
            Layer::Da => &self.da,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ethereum_arbitrum_stack() -> Stack {
        Stack {
            execution: "Arbitrum".to_string(),
            settlement: "Ethereum".to_string(),
            consensus: "Ethereum".to_string(),
            da: "Ethereum (blobs)".to_string(),
        }
    }

    #[test]
    fn complete_stack_has_all_layers() {
        assert!(ethereum_arbitrum_stack().complete());
    }

    #[test]
    fn empty_layers_break_completeness() {
        let mut s = ethereum_arbitrum_stack();
        s.da = String::new();
        assert!(!s.complete());
    }

    #[test]
    fn provider_returns_layer_name() {
        let s = ethereum_arbitrum_stack();
        assert_eq!(s.provider(Layer::Execution), "Arbitrum");
        assert_eq!(s.provider(Layer::Settlement), "Ethereum");
    }
}
