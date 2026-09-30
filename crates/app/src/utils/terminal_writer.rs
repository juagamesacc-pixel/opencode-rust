//! Rust port of `packages/app/src/utils/terminal-writer.ts` (opencode v1.18.30).
//!
//! Source 65 lines: `terminalWriter(write, schedule?)` coalescing queue
//! (`push`/`flush`, `scheduled`/`writing` guards). Ported verbatim as an
//! explicit state machine.
//! Original file: `packages/app/src/utils/terminal-writer.ts`

#![allow(dead_code)]

/// Mirrors the `terminalWriter` queue state.
#[derive(Debug, Default)]
pub struct TerminalWriter {
    pub chunks: Vec<String>,
    pub waits: Vec<usize>,
    pub scheduled: bool,
    pub writing: bool,
    pub flushed: Vec<String>,
    next_wait_id: usize,
}

impl TerminalWriter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mirrors `push(data)` (empty writes ignored).
    pub fn update_push(&mut self, data: &str) {
        if data.is_empty() {
            return;
        }
        self.chunks.push(data.to_string());
        if self.scheduled || self.writing {
            return;
        }
        self.scheduled = true;
    }

    /// Mirrors one `run()` drain step — returns the joined payload when a
    /// write starts.
    pub fn transition_run(&mut self) -> Option<String> {
        if self.writing {
            return None;
        }
        self.scheduled = false;
        if self.chunks.is_empty() {
            return None;
        }
        let payload = self.chunks.join("");
        self.chunks.clear();
        self.writing = true;
        self.flushed.push(payload.clone());
        Some(payload)
    }

    /// Mirrors the write-completion callback.
    pub fn update_write_done(&mut self) {
        self.writing = false;
        if !self.chunks.is_empty() && !self.scheduled {
            self.scheduled = true;
        }
    }

    /// Mirrors `flush(done?)` — returns `true` when already drained.
    pub fn transition_flush(&mut self, with_callback: bool) -> bool {
        if !self.scheduled && !self.writing && self.chunks.is_empty() {
            return true;
        }
        if with_callback {
            let id = self.next_wait_id;
            self.next_wait_id += 1;
            self.waits.push(id);
        }
        self.transition_run();
        false
    }
}
