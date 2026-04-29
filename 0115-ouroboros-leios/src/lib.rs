//! Module 0115: Ouroboros Leios three-layer block model.

#![warn(missing_docs)]

/// Input block: bandwidth-saturating tx batch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InputBlock {
    /// Block id.
    pub id: u64,
    /// Slot.
    pub slot: u64,
}

/// Endorser block: endorses input blocks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EndorserBlock {
    /// Block id.
    pub id: u64,
    /// Endorsed input block ids.
    pub endorses: Vec<u64>,
    /// Slot.
    pub slot: u64,
}

/// Ranking block: orders endorser blocks (Praos-style).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RankingBlock {
    /// Block id.
    pub id: u64,
    /// Endorser-block references.
    pub endorser_refs: Vec<u64>,
    /// Slot.
    pub slot: u64,
}

/// Leios ledger: three independent layers.
#[derive(Clone, Debug, Default)]
pub struct LeiosLedger {
    /// Input layer.
    pub inputs: Vec<InputBlock>,
    /// Endorser layer.
    pub endorsers: Vec<EndorserBlock>,
    /// Ranking layer.
    pub ranking: Vec<RankingBlock>,
}

impl LeiosLedger {
    /// Empty ledger.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Counts of each layer.
    #[must_use]
    pub fn counts(&self) -> (usize, usize, usize) {
        (self.inputs.len(), self.endorsers.len(), self.ranking.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_layers_grow_independently() {
        let mut l = LeiosLedger::new();
        l.inputs.push(InputBlock { id: 1, slot: 0 });
        l.inputs.push(InputBlock { id: 2, slot: 1 });
        l.endorsers.push(EndorserBlock {
            id: 10,
            endorses: vec![1, 2],
            slot: 1,
        });
        l.ranking.push(RankingBlock {
            id: 100,
            endorser_refs: vec![10],
            slot: 2,
        });
        assert_eq!(l.counts(), (2, 1, 1));
    }
}
