//! Module 0030: Disk Paxos -- a logical shared-disk store.

#![warn(missing_docs)]

use std::collections::BTreeMap;

use sim::NodeId;

/// A logical shared disk: a map from (`disk_id`, `process_id`) to
/// the latest written block.
#[derive(Clone, Debug, Default)]
pub struct SharedDisks {
    /// Per (disk, process) latest block.
    pub blocks: BTreeMap<(u32, NodeId), Block>,
}

/// A disk block: ballot + optional accepted (ballot, value).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Block {
    /// Promised ballot.
    pub ballot: u32,
    /// Accepted (ballot, value).
    pub accepted: Option<(u32, u32)>,
}

impl SharedDisks {
    /// Write `block` for `(disk, process)`.
    pub fn write(&mut self, disk: u32, process: NodeId, block: Block) {
        self.blocks.insert((disk, process), block);
    }

    /// Read every other process's block on `disk`.
    pub fn read_others(&self, disk: u32, my_id: NodeId) -> Vec<Block> {
        self.blocks
            .iter()
            .filter(|((d, p), _)| *d == disk && *p != my_id)
            .map(|(_, b)| *b)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_and_read_back() {
        let mut disks = SharedDisks::default();
        disks.write(
            0,
            NodeId(0),
            Block {
                ballot: 1,
                accepted: Some((1, 42)),
            },
        );
        disks.write(
            0,
            NodeId(1),
            Block {
                ballot: 1,
                accepted: None,
            },
        );
        let others = disks.read_others(0, NodeId(0));
        assert_eq!(others.len(), 1);
        assert_eq!(others[0].ballot, 1);
    }

    #[test]
    fn process_reads_majority_of_disks() {
        let mut disks = SharedDisks::default();
        // 3 disks, 1 process. Majority = 2 disks.
        for d in 0..3 {
            disks.write(
                d,
                NodeId(0),
                Block {
                    ballot: 5,
                    accepted: Some((5, 100)),
                },
            );
        }
        let mut majority_count = 0;
        for d in 0..3 {
            if disks
                .blocks
                .get(&(d, NodeId(0)))
                .is_some_and(|b| b.ballot == 5)
            {
                majority_count += 1;
            }
        }
        assert!(majority_count >= 2);
    }
}
