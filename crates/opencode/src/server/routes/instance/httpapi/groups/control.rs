// source: src/server/routes/instance/httpapi/groups/control.ts — exports: [LogInput, ControlPaths, ControlApi]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/provider`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/auth"
/// - "Service name for the log entry"
/// - "Log level"
/// - "Log message"
/// - "Additional metadata for the log entry"
/// - "/auth/:providerID"
/// - "/log"
/// - "Successfully set authentication credentials"
/// source: `export const LogInput` — shape as JSON value; CI verifies.
pub type LogInput = serde_json::Value;
/// source: `export const ControlPaths` — shape as JSON value; CI verifies.
pub type ControlPaths = serde_json::Value;
/// source: `export const ControlApi` — shape as JSON value; CI verifies.
pub type ControlApi = serde_json::Value;
