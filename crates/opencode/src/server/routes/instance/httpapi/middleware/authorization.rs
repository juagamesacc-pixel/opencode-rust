// source: src/server/routes/instance/httpapi/middleware/authorization.ts — exports: [Authorization, PtyConnectAuthorization, authorizationRouterMiddleware, authorizationLayer, ptyConnectAuthorizationLayer, ServerAuthorization, serverAuthorizationLayer]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
// PROVISIONAL pending crates/server: `@opencode-ai/server/middleware/authorization`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/server/auth"
/// - "auth_token"
/// - "Basic realm=\"Secure Area\""
/// - "@opencode/ExperimentalHttpApiAuthorization"
/// - "@opencode/ExperimentalHttpApiPtyConnectAuthorization"
/// - "www-authenticate"
/// - "http://localhost"
use serde::{Deserialize, Serialize};

/// source: `export class Authorization` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authorization {
    pub value: serde_json::Value,
}
/// source: `export class PtyConnectAuthorization` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtyConnectAuthorization {
    pub value: serde_json::Value,
}
/// source: `export const authorizationRouterMiddleware` — shape as JSON value; CI verifies.
pub type authorizationRouterMiddleware = serde_json::Value;
/// source: `export const authorizationLayer` — shape as JSON value; CI verifies.
pub type authorizationLayer = serde_json::Value;
/// source: `export const ptyConnectAuthorizationLayer` — shape as JSON value; CI verifies.
pub type ptyConnectAuthorizationLayer = serde_json::Value;
// source: `export { ServerAuthorization }` — re-export; resolve via crate path.
// source: `export { serverAuthorizationLayer }` — re-export; resolve via crate path.
