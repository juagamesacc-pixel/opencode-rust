// source: src/server/routes/instance/httpapi/middleware/schema-error.ts — exports: [SchemaErrorMiddleware, schemaErrorLayer]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode/HttpApiSchemaError"
/// - "/api/"
/// - "BadRequest"
/// - "schema rejection"
use serde::{Deserialize, Serialize};

/// source: `export class SchemaErrorMiddleware` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaErrorMiddleware {
    pub value: serde_json::Value,
}
/// source: `export const schemaErrorLayer` — shape as JSON value; CI verifies.
pub type schemaErrorLayer = serde_json::Value;
