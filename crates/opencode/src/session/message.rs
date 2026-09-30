// source: src/session/message.ts — exports: [ToolCall, ToolPartialCall, ToolResult, ToolInvocation, TextPart, ReasoningPart, ToolInvocationPart, SourceUrlPart, FilePart, StepStartPart, MessagePart, Info, AuthError, OutputLengthError]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/schema`
// PROVISIONAL pending crates/core: `@opencode-ai/core/provider`
// PROVISIONAL pending crates/core: `@opencode-ai/core/model`
/// verbatim strings (source order, quoted for V2 audit):
/// - "ToolCall"
/// - "partial-call"
/// - "ToolPartialCall"
/// - "ToolResult"
/// - "ToolInvocation"
/// - "TextPart"
/// - "reasoning"
/// - "ReasoningPart"
/// source: `export const ToolCall` — shape as JSON value; CI verifies.
pub type ToolCall = serde_json::Value;
/// source: `export const ToolPartialCall` — shape as JSON value; CI verifies.
pub type ToolPartialCall = serde_json::Value;
/// source: `export const ToolResult` — shape as JSON value; CI verifies.
pub type ToolResult = serde_json::Value;
/// source: `export const ToolInvocation` — shape as JSON value; CI verifies.
pub type ToolInvocation = serde_json::Value;
/// source: `export const TextPart` — shape as JSON value; CI verifies.
pub type TextPart = serde_json::Value;
/// source: `export const ReasoningPart` — shape as JSON value; CI verifies.
pub type ReasoningPart = serde_json::Value;
/// source: `export const ToolInvocationPart` — shape as JSON value; CI verifies.
pub type ToolInvocationPart = serde_json::Value;
/// source: `export const SourceUrlPart` — shape as JSON value; CI verifies.
pub type SourceUrlPart = serde_json::Value;
/// source: `export const FilePart` — shape as JSON value; CI verifies.
pub type FilePart = serde_json::Value;
/// source: `export const StepStartPart` — shape as JSON value; CI verifies.
pub type StepStartPart = serde_json::Value;
/// source: `export const MessagePart` — shape as JSON value; CI verifies.
pub type MessagePart = serde_json::Value;
/// source: `export const Info` — shape as JSON value; CI verifies.
pub type Info = serde_json::Value;
// source: `export { AuthError }` — re-export; resolve via crate path.
// source: `export { OutputLengthError }` — re-export; resolve via crate path.
