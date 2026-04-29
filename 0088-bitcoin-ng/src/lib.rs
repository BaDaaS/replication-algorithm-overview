//! Module 0088: Bitcoin-NG block model.
//!
//! Two block types: key blocks (`PoW`, leader-electing) and
//! microblocks (signed by leader, no `PoW`). Microblocks attach
//! to the most recent key block.

#![warn(missing_docs)]

/// 32-byte hash digest.
pub type Hash = [u8; 32];

/// Public-key placeholder for the leader.
pub type LeaderKey = u64;

/// Key block: `PoW` witness, elects the leader for the next epoch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyBlock {
    /// Epoch index (monotonic).
    pub epoch: u64,
    /// Hash of the previous key block.
    pub parent: Hash,
    /// Public key of the elected leader.
    pub leader: LeaderKey,
    /// Hash of this key block.
    pub hash: Hash,
}

/// Microblock: leader-signed batch of transactions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Microblock {
    /// Epoch this microblock belongs to.
    pub epoch: u64,
    /// Sequence number within the epoch.
    pub seq: u64,
    /// Hash of the parent (previous microblock or key block).
    pub parent: Hash,
    /// Public key of the signer (must match the epoch leader).
    pub leader: LeaderKey,
    /// Opaque transactions.
    pub txs: Vec<Vec<u8>>,
    /// Hash of this microblock.
    pub hash: Hash,
}

/// Bitcoin-NG ledger: a sequence of key blocks plus an ordered
/// list of microblocks per epoch.
#[derive(Clone, Debug, Default)]
pub struct Ledger {
    /// Key blocks in order.
    pub keys: Vec<KeyBlock>,
    /// Microblocks per epoch (`epoch -> microblocks`).
    pub micros: Vec<Vec<Microblock>>,
}

impl Ledger {
    /// Empty ledger.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a key block. Returns false if the epoch is not
    /// `keys.len()` or the parent does not match.
    pub fn append_key(&mut self, k: KeyBlock) -> bool {
        let expected_parent = self.keys.last().map_or([0u8; 32], |kb| kb.hash);
        if k.epoch != self.keys.len() as u64 || k.parent != expected_parent {
            return false;
        }
        self.keys.push(k);
        self.micros.push(Vec::new());
        true
    }

    /// Append a microblock to the current epoch. Returns false if
    /// no key block exists, the leader does not match, or the
    /// sequence/parent are inconsistent.
    pub fn append_micro(&mut self, m: Microblock) -> bool {
        let Some(current) = self.keys.last() else {
            return false;
        };
        if m.epoch != current.epoch || m.leader != current.leader {
            return false;
        }
        let Ok(idx) = usize::try_from(m.epoch) else {
            return false;
        };
        let micros = &mut self.micros[idx];
        let expected_seq = micros.len() as u64;
        let expected_parent = micros.last().map_or(current.hash, |mb| mb.hash);
        if m.seq != expected_seq || m.parent != expected_parent {
            return false;
        }
        micros.push(m);
        true
    }

    /// Microblocks for the latest epoch.
    #[must_use]
    pub fn current_micros(&self) -> &[Microblock] {
        self.micros.last().map_or(&[], |v| v.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(epoch: u64, parent: Hash, leader: LeaderKey, hb: u8) -> KeyBlock {
        KeyBlock {
            epoch,
            parent,
            leader,
            hash: [hb; 32],
        }
    }

    fn micro(
        epoch: u64,
        seq: u64,
        parent: Hash,
        leader: LeaderKey,
        hb: u8,
    ) -> Microblock {
        Microblock {
            epoch,
            seq,
            parent,
            leader,
            txs: vec![],
            hash: [hb; 32],
        }
    }

    #[test]
    fn key_chain_extends() {
        let mut l = Ledger::new();
        assert!(l.append_key(key(0, [0u8; 32], 100, 1)));
        assert!(l.append_key(key(1, [1u8; 32], 200, 2)));
        assert_eq!(l.keys.len(), 2);
    }

    #[test]
    fn micros_attach_to_current_epoch() {
        let mut l = Ledger::new();
        l.append_key(key(0, [0u8; 32], 100, 1));
        assert!(l.append_micro(micro(0, 0, [1u8; 32], 100, 11)));
        assert!(l.append_micro(micro(0, 1, [11u8; 32], 100, 12)));
        assert_eq!(l.current_micros().len(), 2);
    }

    #[test]
    fn micro_with_wrong_leader_is_rejected() {
        let mut l = Ledger::new();
        l.append_key(key(0, [0u8; 32], 100, 1));
        assert!(!l.append_micro(micro(0, 0, [1u8; 32], 999, 11)));
    }

    #[test]
    fn micro_with_stale_epoch_is_rejected() {
        let mut l = Ledger::new();
        l.append_key(key(0, [0u8; 32], 100, 1));
        l.append_micro(micro(0, 0, [1u8; 32], 100, 11));
        l.append_key(key(1, [1u8; 32], 200, 2));
        // Old leader keeps trying to extend old epoch in new state.
        assert!(!l.append_micro(micro(0, 1, [11u8; 32], 100, 99)));
    }
}
