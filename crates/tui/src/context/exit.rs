// source: packages/tui/src/context/exit.tsx (8 lines, v1.18.30)
// 1:1 port — the context value is the exit function itself.

#![allow(dead_code)]

use std::sync::Arc;

/// Mirrors `Exit` (`(reason?: unknown) => void`).
pub type ExitFn = Arc<dyn Fn(Option<String>) + Send + Sync>;

/// Mirrors the Exit context value.
#[derive(Clone)]
pub struct Exit {
    exit: ExitFn,
}

impl Exit {
    pub fn new(exit: ExitFn) -> Self {
        Self { exit }
    }

    pub fn call(&self, reason: Option<String>) {
        (self.exit)(reason);
    }
}
