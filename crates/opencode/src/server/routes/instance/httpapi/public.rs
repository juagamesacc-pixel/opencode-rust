// source: src/server/routes/instance/httpapi/public.ts — exports: [PublicApi]
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "effect/unstable/httpapi"
/// - "GET /experimental/session start"
/// - "GET /experimental/session roots"
/// - "GET /experimental/session archived"
/// - "GET /find/file limit"
/// - "GET /experimental/session cursor"
/// - "GET /experimental/session limit"
/// - "GET /session start"
/// source: `export const PublicApi` — shape as JSON value; CI verifies.
pub type PublicApi = serde_json::Value;
