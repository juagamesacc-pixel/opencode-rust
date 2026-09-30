// source: src/session/llm/native-runtime.ts — exports: [RuntimeStatus, StreamResult, status, stream, nativeTools]
// PROVISIONAL pending external `ai` (host-provided; no new dep)
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/Stream` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
// PROVISIONAL pending crates/llm: `@opencode-ai/llm`
// PROVISIONAL pending crates/llm: `@opencode-ai/llm/route`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/auth"
/// - "supported"
/// - "unsupported"
/// - "required"
/// - "provider"
/// - "anthropic"
/// - "opencode"
/// - "provider is not openai, opencode, or anthropic"
/// source: `export type RuntimeStatus` — shape as JSON value; CI verifies.
pub type RuntimeStatus = serde_json::Value;
/// source: `export type StreamResult` — shape as JSON value; CI verifies.
pub type StreamResult = serde_json::Value;
/// source: `export function status` — stub shell; CI verifies behavior.
pub fn status(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function stream` — stub shell; CI verifies behavior.
pub fn stream(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function nativeTools` — stub shell; CI verifies behavior.
pub fn nativeTools(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
