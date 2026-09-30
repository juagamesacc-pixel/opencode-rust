// source: src/server/routes/instance/httpapi/middleware/error.ts — exports: [errorLayer]
// PROVISIONAL pending crates/core: `@opencode-ai/core/util/error`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/config/error`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/util/error"
/// - "Unexpected server error. Check server logs for details."
/// source: `export const errorLayer` — shape as JSON value; CI verifies.
pub type errorLayer = serde_json::Value;
