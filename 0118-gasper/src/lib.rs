//! Module 0118: Gasper combined LMD-GHOST + FFG state.

#![warn(missing_docs)]

use std::collections::BTreeMap;

/// Block height.
pub type Height = u64;

/// Per-validator attestation: latest block they support and
/// FFG link they vote.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Attestation {
    /// Validator id.
    pub validator: u64,
    /// Latest block voted for (LMD).
    pub head: Height,
    /// FFG source checkpoint.
    pub ffg_source: Height,
    /// FFG target checkpoint.
    pub ffg_target: Height,
}

/// Gasper combined state.
#[derive(Clone, Debug, Default)]
pub struct Gasper {
    /// Total validator count.
    pub n: usize,
    /// Latest LMD attestation per validator.
    pub latest: BTreeMap<u64, Height>,
    /// FFG checkpoint heights -> voter set.
    pub ffg_votes: BTreeMap<(Height, Height), Vec<u64>>,
    /// Justified checkpoint heights.
    pub justified: Vec<Height>,
    /// Finalised checkpoint heights.
    pub finalised: Vec<Height>,
}

impl Gasper {
    /// Build state with `n` validators and genesis at 0
    /// justified.
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            latest: BTreeMap::new(),
            ffg_votes: BTreeMap::new(),
            justified: vec![0],
            finalised: vec![0],
        }
    }

    /// Record an attestation: updates LMD-latest and FFG votes.
    pub fn vote(&mut self, a: Attestation) {
        self.latest.insert(a.validator, a.head);
        let voters = self
            .ffg_votes
            .entry((a.ffg_source, a.ffg_target))
            .or_default();
        if !voters.contains(&a.validator) {
            voters.push(a.validator);
        }
        let stake = voters.len();
        if 3 * stake >= 2 * self.n
            && self.justified.contains(&a.ffg_source)
            && !self.justified.contains(&a.ffg_target)
        {
            self.justified.push(a.ffg_target);
            if a.ffg_target == a.ffg_source + 1
                && !self.finalised.contains(&a.ffg_source)
            {
                self.finalised.push(a.ffg_source);
            }
        }
    }

    /// LMD-GHOST head: block with the most recent attestations
    /// pointing at it.
    #[must_use]
    pub fn head(&self) -> Option<Height> {
        let mut counts: BTreeMap<Height, u64> = BTreeMap::new();
        for &h in self.latest.values() {
            *counts.entry(h).or_insert(0) += 1;
        }
        let mut best: Option<(Height, u64)> = None;
        for (&h, &c) in &counts {
            best = match best {
                None => Some((h, c)),
                Some((bh, bc)) if c > bc || (c == bc && h > bh) => Some((h, c)),
                Some(b) => Some(b),
            };
        }
        best.map(|(h, _)| h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn att(v: u64, head: Height, src: Height, tgt: Height) -> Attestation {
        Attestation {
            validator: v,
            head,
            ffg_source: src,
            ffg_target: tgt,
        }
    }

    #[test]
    fn lmd_head_follows_attestation_weight() {
        let mut g = Gasper::new(3);
        g.vote(att(1, 5, 0, 0));
        g.vote(att(2, 5, 0, 0));
        g.vote(att(3, 7, 0, 0));
        assert_eq!(g.head(), Some(5));
    }

    #[test]
    fn ffg_finalisation_via_consecutive_justification() {
        let mut g = Gasper::new(3);
        // Justify checkpoint 1.
        g.vote(att(1, 1, 0, 1));
        g.vote(att(2, 1, 0, 1));
        assert!(g.justified.contains(&1));
        // Justify checkpoint 2 directly after 1.
        g.vote(att(1, 2, 1, 2));
        g.vote(att(2, 2, 1, 2));
        assert!(g.finalised.contains(&1));
    }

    #[test]
    fn empty_state_has_no_head() {
        let g = Gasper::new(3);
        assert_eq!(g.head(), None);
    }
}
