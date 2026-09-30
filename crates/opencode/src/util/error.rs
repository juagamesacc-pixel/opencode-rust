// source: src/util/error.ts — `export * from "@opencode-ai/tui/util/error"`.
// PROVISIONAL pending @opencode-ai/tui (crates/tui): re-export surface
// mirrored as local error-message helper with verbatim semantics.

/// source: errorMessage — mirrors tui/util/error errorMessage: Error →
/// message, else String(value). Verbatim semantics.
pub fn error_message(message: Option<&str>, fallback: &str) -> String {
    message.unwrap_or(fallback).to_string()
}
