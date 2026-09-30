// source: src/server/routes/instance/httpapi/groups/mcp.ts — exports: [AddPayload, StatusMap, AuthStartResponse, AuthCallbackPayload, AuthRemoveResponse, UnsupportedOAuthError, McpPaths, McpApi]
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/config/mcp`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/mcp"
/// - "McpUnsupportedOAuthError"
/// - "/mcp"
/// - "/mcp/:name/auth"
/// - "/mcp/:name/auth/callback"
/// - "/mcp/:name/auth/authenticate"
/// - "/mcp/:name/connect"
/// - "/mcp/:name/disconnect"
use serde::{Deserialize, Serialize};

/// source: `export const AddPayload` — shape as JSON value; CI verifies.
pub type AddPayload = serde_json::Value;
/// source: `export const StatusMap` — shape as JSON value; CI verifies.
pub type StatusMap = serde_json::Value;
/// source: `export const AuthStartResponse` — shape as JSON value; CI verifies.
pub type AuthStartResponse = serde_json::Value;
/// source: `export const AuthCallbackPayload` — shape as JSON value; CI verifies.
pub type AuthCallbackPayload = serde_json::Value;
/// source: `export const AuthRemoveResponse` — shape as JSON value; CI verifies.
pub type AuthRemoveResponse = serde_json::Value;
/// source: `export class UnsupportedOAuthError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsupportedOAuthError {
    pub value: serde_json::Value,
}
/// source: `export const McpPaths` — shape as JSON value; CI verifies.
pub type McpPaths = serde_json::Value;
/// source: `export const McpApi` — shape as JSON value; CI verifies.
pub type McpApi = serde_json::Value;
