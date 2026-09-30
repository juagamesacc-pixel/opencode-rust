//! Rust port of `packages/app/src/utils/worktree.ts` (opencode v1.18.30).
//!
//! Source 76 lines: `Worktree` (`get`/`pending`/`ready`/`failed`/`wait`)
//! keyed by `ScopedKey.from(scope, normalize(directory))` with trailing
//! `[\\/]+$` normalization. Waiters are explicit state-machine entries.
//! Original file: `packages/app/src/utils/worktree.ts`

#![allow(dead_code)]

use std::collections::HashMap;

/// Mirrors `State`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorktreeState {
    Pending,
    Ready,
    Failed { message: String },
}

/// Mirrors the trailing `[\\/]+$` directory normalization.
pub fn normalize_worktree_directory(directory: &str) -> String {
    directory.trim_end_matches(['/', '\\']).to_string()
}

/// Mirrors `key(scope, directory)`.
pub fn worktree_key(scope: &str, directory: &str) -> String {
    format!("{scope}\0{}", normalize_worktree_directory(directory))
}

/// Mirrors `Worktree` store + waiters.
#[derive(Debug, Default)]
pub struct Worktree {
    state: HashMap<String, WorktreeState>,
    waiters: HashMap<String, Vec<WorktreeState>>,
}

impl Worktree {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mirrors `get(scope, directory)`.
    pub fn get(&self, scope: &str, directory: &str) -> Option<&WorktreeState> {
        self.state.get(&worktree_key(scope, directory))
    }

    /// Mirrors `pending(scope, directory)` (ready/failed are sticky).
    pub fn update_pending(&mut self, scope: &str, directory: &str) {
        let id = worktree_key(scope, directory);
        if let Some(current) = self.state.get(&id) {
            if *current != WorktreeState::Pending {
                return;
            }
        }
        self.state.insert(id, WorktreeState::Pending);
    }

    /// Mirrors `ready(scope, directory)`.
    pub fn transition_ready(&mut self, scope: &str, directory: &str) {
        let id = worktree_key(scope, directory);
        self.state.insert(id.clone(), WorktreeState::Ready);
        if let Some(waiters) = self.waiters.remove(&id) {
            let _ = waiters;
        }
    }

    /// Mirrors `failed(scope, directory, message)`.
    pub fn transition_failed(&mut self, scope: &str, directory: &str, message: &str) {
        let id = worktree_key(scope, directory);
        self.state.insert(
            id.clone(),
            WorktreeState::Failed {
                message: message.to_string(),
            },
        );
        if let Some(waiters) = self.waiters.remove(&id) {
            let _ = waiters;
        }
    }

    /// Mirrors `wait(scope, directory)` immediate (non-pending resolves at once).
    pub fn transition_wait_immediate(&self, scope: &str, directory: &str) -> Option<WorktreeState> {
        match self.state.get(&worktree_key(scope, directory)) {
            Some(WorktreeState::Pending) | None => None,
            Some(state) => Some(state.clone()),
        }
    }
}
