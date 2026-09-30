// source: src/cli/cmd/run/footer.ts — exports: [RunFooter]
// PROVISIONAL pending crates/tui: `@opencode-ai/tui/keymap`
/// verbatim strings (source order, quoted for V2 audit):
/// - "turn.send"
/// - "sending prompt"
/// - "turn.wait"
/// - "waiting for assistant"
/// - "turn.idle"
/// - "stream.patch"
/// - "composer"
/// - "sessionID"
use serde::{Deserialize, Serialize};

/// source: `export class RunFooter` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunFooter {
    pub value: serde_json::Value,
}
