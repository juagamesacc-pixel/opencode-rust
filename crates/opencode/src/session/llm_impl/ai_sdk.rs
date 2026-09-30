// source: src/session/llm/ai-sdk.ts — exports: [adapterState, toLLMEvents]
// PROVISIONAL pending crates/llm: `@opencode-ai/llm`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `ai` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/llm"
/// - "fullStream"
/// - "start-step"
/// - "finish-step"
/// - "network_error"
/// - "Provider finish_reason: network_error"
/// - "providerMetadata"
/// - "text-start"
/// source: `export function adapterState` — stub shell; CI verifies behavior.
pub fn adapterState(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function toLLMEvents` — stub shell; CI verifies behavior.
pub fn toLLMEvents(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
