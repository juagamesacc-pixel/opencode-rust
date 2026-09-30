//! Rust port of `packages/core/src/snapshot.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    pub id: String,
    pub created: u64,
}

#[derive(Debug)]
pub struct SnapshotError {
    pub message: String,
}
impl std::fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Snapshot.Error: {}", self.message)
    }
}
impl std::error::Error for SnapshotError {}

// PROVISIONAL pending filesystem snapshot wiring.
