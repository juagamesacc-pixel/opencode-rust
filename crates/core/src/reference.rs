//! Rust port of `packages/core/src/reference.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub path: String,
    pub content: String,
}

// PROVISIONAL pending reference resolution wiring.
