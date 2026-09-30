// source: src/session/schema.ts — exports: [SessionID, MessageID, PartID]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/session`
// PROVISIONAL pending crates/core: `@opencode-ai/core/schema`
/// verbatim strings (source order, quoted for V2 audit):
/// - "MessageID"
/// source: `export const SessionID` — shape as JSON value; CI verifies.
pub type SessionID = serde_json::Value;
/// source: `export const MessageID` — shape as JSON value; CI verifies.
pub type MessageID = serde_json::Value;
/// source: `export const PartID` — shape as JSON value; CI verifies.
pub type PartID = serde_json::Value;
