// source: src/server/routes/instance/httpapi/lifecycle.ts — exports: [markInstanceForDisposal, markInstanceForReload, disposeMiddleware]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/effect/bridge"
/// - "instance disposal failed"
/// source: `export const markInstanceForDisposal` — shape as JSON value; CI verifies.
pub type markInstanceForDisposal = serde_json::Value;
/// source: `export const markInstanceForReload` — shape as JSON value; CI verifies.
pub type markInstanceForReload = serde_json::Value;
/// source: `export const disposeMiddleware` — shape as JSON value; CI verifies.
pub type disposeMiddleware = serde_json::Value;
