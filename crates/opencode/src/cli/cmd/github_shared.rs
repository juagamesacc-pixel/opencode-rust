// source: src/cli/cmd/github.shared.ts — exports: [extractResponseText, formatPromptTooLargeError, parseGitHubRemote]
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/v1/session"
/// - "Failed to parse response: no parts returned"
/// source: `export function extractResponseText` — stub shell; CI verifies behavior.
pub fn extractResponseText(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function formatPromptTooLargeError` — stub shell; CI verifies behavior.
pub fn formatPromptTooLargeError(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
// source: `export { parseGitHubRemote }` — re-export; resolve via crate path.
