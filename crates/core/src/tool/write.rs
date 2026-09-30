//! Rust port of `packages/core/src/tool/write.ts`.

use serde::{Deserialize, Serialize};

pub const NAME: &str = "write";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub operation: String,
    pub target: String,
    pub resource: String,
    pub existed: bool,
}

pub fn to_model_output(output: &Output) -> String {
    if output.existed {
        format!("Wrote file successfully: {}", output.resource)
    } else {
        format!("Created file successfully: {}", output.resource)
    }
}

// PROVISIONAL pending effect/runtime — Layer wiring requires LocationMutation + FileMutation + PermissionV2.
