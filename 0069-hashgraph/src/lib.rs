//! Module 0069: `HashGraph` event types.

#![warn(missing_docs)]

/// `HashGraph` event: gossip-about-gossip.
#[derive(Clone, Debug)]
pub struct Event {
    /// Self-parent hash.
    pub self_parent: u64,
    /// Other-parent hash.
    pub other_parent: u64,
    /// Creator id.
    pub creator: u32,
    /// Optional payload (transactions).
    pub payload: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_construction() {
        let e = Event {
            self_parent: 0xABC,
            other_parent: 0xDEF,
            creator: 1,
            payload: vec![1, 2, 3],
        };
        assert_eq!(e.creator, 1);
    }
}
