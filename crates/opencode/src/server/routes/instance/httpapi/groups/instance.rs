// source: src/server/routes/instance/httpapi/groups/instance.ts — exports: [VcsDiffQuery, ApiVcsApplyError, InstancePaths, InstanceApi]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/agent/agent"
/// - "VcsApplyError"
/// - "non-git"
/// - "not-clean"
/// - "/instance/dispose"
/// - "/path"
/// - "/vcs"
/// - "/vcs/status"
use serde::{Deserialize, Serialize};

/// source: `export const VcsDiffQuery` — shape as JSON value; CI verifies.
pub type VcsDiffQuery = serde_json::Value;
/// source: `export class ApiVcsApplyError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiVcsApplyError {
    pub value: serde_json::Value,
}
/// source: `export const InstancePaths` — shape as JSON value; CI verifies.
pub type InstancePaths = serde_json::Value;
/// source: `export const InstanceApi` — shape as JSON value; CI verifies.
pub type InstanceApi = serde_json::Value;
