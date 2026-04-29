//! Module 0036: CRAQ helpers (clean/dirty per-key state).

#![warn(missing_docs)]

/// Per-key state in CRAQ: clean version + list of dirty
/// (in-flight) versions.
#[derive(Clone, Debug, Default)]
pub struct KeyState {
    /// The latest known-committed version.
    pub clean: Option<u32>,
    /// In-flight dirty versions (version, value).
    pub dirty: Vec<(u32, u32)>,
}

impl KeyState {
    /// Local read: returns clean if no dirty pending.
    /// Otherwise, returns None (caller must query tail).
    #[must_use]
    pub fn local_read(&self) -> Option<u32> {
        if self.dirty.is_empty() {
            self.clean
        } else {
            None
        }
    }

    /// Apply a new dirty version (during write propagation).
    pub fn add_dirty(&mut self, version: u32, value: u32) {
        self.dirty.push((version, value));
    }

    /// Promote dirty version `version` to clean (on tail-ack
    /// flowing back).
    pub fn promote(&mut self, version: u32) {
        if let Some(idx) = self.dirty.iter().position(|(v, _)| *v == version) {
            let (_, value) = self.dirty.remove(idx);
            self.clean = Some(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_read_returns_clean() {
        let s = KeyState {
            clean: Some(100),
            dirty: vec![],
        };
        assert_eq!(s.local_read(), Some(100));
    }

    #[test]
    fn dirty_blocks_local_read() {
        let s = KeyState {
            clean: Some(100),
            dirty: vec![(2, 200)],
        };
        assert_eq!(s.local_read(), None);
    }

    #[test]
    fn promote_clears_dirty() {
        let mut s = KeyState {
            clean: Some(100),
            dirty: vec![(2, 200)],
        };
        s.promote(2);
        assert_eq!(s.clean, Some(200));
        assert!(s.dirty.is_empty());
    }
}
