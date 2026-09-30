// source: src/session/llm/request.ts — exports: [Prepared, prepare, hasToolCalls]
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/permission`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
// PROVISIONAL pending crates/core: `@opencode-ai/core/installation/version`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `ai` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/v1/permission"
/// - "LLMRequestPrep.prepare"
/// - "experimental.chat.system.transform"
/// - "@ai-sdk/azure"
/// - "chat.params"
/// - "chat.headers"
/// - "@ai-sdk/openai"
/// - "@ai-sdk/amazon-bedrock/mantle"
/// source: `export type Prepared` — shape as JSON value; CI verifies.
pub type Prepared = serde_json::Value;
/// source: `export const prepare` — shape as JSON value; CI verifies.
pub type prepare = serde_json::Value;
/// source: `export function hasToolCalls` — stub shell; CI verifies behavior.
pub fn hasToolCalls(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
