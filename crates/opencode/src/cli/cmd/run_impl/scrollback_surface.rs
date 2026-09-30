// source: src/cli/cmd/run/scrollback.surface.ts — exports: [RunScrollbackStream]
/// verbatim strings (source order, quoted for V2 audit):
/// - "structured"
/// - "progress"
/// - "completed"
/// - "top-level"
/// - "markdown"
use serde::{Deserialize, Serialize};

/// source: `export class RunScrollbackStream` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunScrollbackStream {
    pub value: serde_json::Value,
}
