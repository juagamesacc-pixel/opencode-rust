//! Rust port of `packages/app/src/pages/session/message-id-from-hash.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `session/message-id-from-hash.ts` -> `session/message_id_from_hash.rs` (kebab -> snake_case).

pub fn message_id_from_hash(hash: &str) -> Option<String> {
    let value = hash.strip_prefix('#').unwrap_or(hash);
    Some(value.strip_prefix("message-")?.to_string())
}
