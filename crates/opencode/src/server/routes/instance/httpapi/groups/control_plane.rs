// source: src/server/routes/instance/httpapi/groups/control-plane.ts — exports: [MoveSessionPayload, ApiMoveSessionError, ControlPlaneApi]
// PROVISIONAL pending crates/core: `@opencode-ai/core/control-plane/move-session`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/control-plane/move-session"
/// - "/experimental/control-plane"
/// - "MoveSessionError"
/// - "controlPlane"
/// - "moveSession"
/// - "Session moved"
/// - "experimental.controlPlane.moveSession"
/// - "Move session"
use serde::{Deserialize, Serialize};

/// source: `export const MoveSessionPayload` — shape as JSON value; CI verifies.
pub type MoveSessionPayload = serde_json::Value;
/// source: `export class ApiMoveSessionError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMoveSessionError {
    pub value: serde_json::Value,
}
/// source: `export const ControlPlaneApi` — shape as JSON value; CI verifies.
pub type ControlPlaneApi = serde_json::Value;
