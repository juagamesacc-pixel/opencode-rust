// source: src/session/message-error.ts — exports: [OutputLengthError, AuthError, Shared, SharedSchema]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/util/error`
/// verbatim strings (source order, quoted for V2 audit):
/// - "MessageOutputLengthError"
/// - "ProviderAuthError"
/// source: `export const OutputLengthError` — shape as JSON value; CI verifies.
pub type OutputLengthError = serde_json::Value;
/// source: `export const AuthError` — shape as JSON value; CI verifies.
pub type AuthError = serde_json::Value;
/// source: `export const Shared` — shape as JSON value; CI verifies.
pub type Shared = serde_json::Value;
/// source: `export const SharedSchema` — shape as JSON value; CI verifies.
pub type SharedSchema = serde_json::Value;
