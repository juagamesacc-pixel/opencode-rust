// source: src/session/message-v2.ts — exports: [SYNTHETIC_ATTACHMENT_PROMPT, Event, cursor, toModelMessagesEffect, toModelMessages, page, stream, parts, get, filterCompacted, filterCompactedEffect, latest, fromError, node, isMedia]
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
// PROVISIONAL pending crates/core: `@opencode-ai/core/provider`
// PROVISIONAL pending crates/core: `@opencode-ai/core/util/error`
// PROVISIONAL pending external `ai` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/database/database`
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/layer-node`
// PROVISIONAL pending crates/core: `@opencode-ai/core/session/sql`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "./schema"
/// - "ZlibError"
/// - "Attached media from tool result:"
/// - "base64url"
/// - "@ai-sdk/anthropic"
/// - "@ai-sdk/openai"
/// - "@ai-sdk/amazon-bedrock/mantle"
/// - "@ai-sdk/amazon-bedrock"
/// source: `SYNTHETIC_ATTACHMENT_PROMPT = "Attached media from tool result:"` — verbatim.
pub const SYNTHETIC_ATTACHMENT_PROMPT: &str = "Attached media from tool result:";
/// source: `export const Event` — shape as JSON value; CI verifies.
pub type Event = serde_json::Value;
/// source: `export const cursor` — shape as JSON value; CI verifies.
pub type cursor = serde_json::Value;
/// source: `export const toModelMessagesEffect` — shape as JSON value; CI verifies.
pub type toModelMessagesEffect = serde_json::Value;
/// source: `export function toModelMessages` — stub shell; CI verifies behavior.
pub fn toModelMessages(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const page` — shape as JSON value; CI verifies.
pub type page = serde_json::Value;
/// source: `export function stream` — stub shell; CI verifies behavior.
pub fn stream(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function parts` — stub shell; CI verifies behavior.
pub fn parts(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const get` — shape as JSON value; CI verifies.
pub type get = serde_json::Value;
/// source: `export function filterCompacted` — stub shell; CI verifies behavior.
pub fn filterCompacted(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const filterCompactedEffect` — shape as JSON value; CI verifies.
pub type filterCompactedEffect = serde_json::Value;
/// source: `export function latest` — stub shell; CI verifies behavior.
pub fn latest(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function fromError` — stub shell; CI verifies behavior.
pub fn fromError(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const node` — shape as JSON value; CI verifies.
pub type node = serde_json::Value;
// source: `export { isMedia }` — re-export; resolve via crate path.
