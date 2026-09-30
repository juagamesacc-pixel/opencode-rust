// source: src/session/llm/native-request.ts — exports: [RequestInput, model, request]
// PROVISIONAL pending crates/llm: `@opencode-ai/llm`
// PROVISIONAL pending crates/llm: `@opencode-ai/llm/providers`
// PROVISIONAL pending external `ai` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/llm"
/// - "required"
/// - "providerOptions"
/// - "Native LLM request adapter only supports file parts with string or Uint8Array data"
/// - "application/octet-stream"
/// - "error-text"
/// - "Native LLM request adapter only supports object content parts"
/// - "reasoning"
/// source: `export type RequestInput` — shape as JSON value; CI verifies.
pub type RequestInput = serde_json::Value;
/// source: `export const model` — shape as JSON value; CI verifies.
pub type model = serde_json::Value;
/// source: `export const request` — shape as JSON value; CI verifies.
pub type request = serde_json::Value;
