// source: packages/tui/src/util/layout.ts (25 lines, v1.18.30)
// 1:1 port — Yoga pre-layout sibling margins become a plain resolver: the
// caller supplies the previous sibling for a frame (ratatui has no Yoga
// lifecycle pass; margins are computed during layout).

#![allow(dead_code)]

/// Per-frame sibling index cache (mirrors the WeakMap keyed by frame id).
#[derive(Debug, Default)]
pub struct SiblingMargins {
    frame_id: u64,
    cache: std::collections::HashMap<usize, Option<usize>>,
}

impl SiblingMargins {
    /// Mirrors `previousSiblings` — map child index → previous index.
    pub fn rebuild(&mut self, frame_id: u64, child_count: usize) {
        self.frame_id = frame_id;
        self.cache.clear();
        for index in 0..child_count {
            self.cache.insert(index, if index == 0 { None } else { Some(index - 1) });
        }
    }

    /// Mirrors the lifecycle pass: returns the margin for `index` given
    /// `margin(previous)`. Caches per frame id (rebuild on change).
    pub fn margin_for(&mut self, frame_id: u64, child_count: usize, index: usize, margin: impl Fn(Option<usize>) -> u16) -> u16 {
        if self.frame_id != frame_id || self.cache.len() != child_count {
            self.rebuild(frame_id, child_count);
        }
        margin(self.cache.get(&index).copied().flatten())
    }
}