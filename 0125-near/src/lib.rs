//! Module 0125: NEAR Nightshade per-shard chunk aggregation.

#![warn(missing_docs)]

/// Shard identifier.
pub type ShardId = u32;

/// Per-shard chunk root.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chunk {
    /// Shard the chunk belongs to.
    pub shard: ShardId,
    /// Chunk root (placeholder hash).
    pub root: [u8; 32],
}

/// Shard configuration.
#[derive(Clone, Debug, Default)]
pub struct Shard {
    /// Shard id.
    pub id: ShardId,
    /// Validator ids assigned to the shard.
    pub validators: Vec<u64>,
}

/// `NEAR` block: aggregates chunks from all shards.
#[derive(Clone, Debug, Default)]
pub struct Block {
    /// Shards present in this block (ordered by id).
    pub chunks: Vec<Chunk>,
}

impl Block {
    /// Build with given chunk list.
    #[must_use]
    pub fn new(chunks: Vec<Chunk>) -> Self {
        Self { chunks }
    }

    /// True iff every shard id `0..n` has exactly one chunk.
    #[must_use]
    pub fn complete(&self, n: ShardId) -> bool {
        if self.chunks.len() != n as usize {
            return false;
        }
        for (i, c) in self.chunks.iter().enumerate() {
            let Ok(expected) = ShardId::try_from(i) else {
                return false;
            };
            if c.shard != expected {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ch(shard: ShardId, byte: u8) -> Chunk {
        Chunk {
            shard,
            root: [byte; 32],
        }
    }

    #[test]
    fn complete_block_has_chunks_per_shard() {
        let b = Block::new(vec![ch(0, 1), ch(1, 2), ch(2, 3), ch(3, 4)]);
        assert!(b.complete(4));
    }

    #[test]
    fn missing_chunk_fails_completeness() {
        let b = Block::new(vec![ch(0, 1), ch(1, 2)]);
        assert!(!b.complete(4));
    }

    #[test]
    fn shard_records_validators() {
        let s = Shard {
            id: 7,
            validators: vec![1, 2, 3],
        };
        assert_eq!(s.id, 7);
        assert_eq!(s.validators.len(), 3);
    }
}
