// source: src/server/routes/instance/httpapi/middleware/instance-context.ts — exports: [InstanceContextMiddleware, instanceContextLayer]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/effect/instance-ref"
/// - "@opencode/ExperimentalHttpApiInstanceContext"
use serde::{Deserialize, Serialize};

/// source: `export class InstanceContextMiddleware` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceContextMiddleware {
    pub value: serde_json::Value,
}
/// source: `export const instanceContextLayer` — shape as JSON value; CI verifies.
pub type instanceContextLayer = serde_json::Value;
