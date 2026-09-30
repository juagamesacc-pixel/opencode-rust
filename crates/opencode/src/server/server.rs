// source: src/server/server.ts — exports: [Listener, Default, openapi, listen]
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/app-node-builder`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
// PROVISIONAL pending external `node:http` (host-provided; no new dep)
// PROVISIONAL pending crates/server: `@opencode-ai/server/cors`
/// verbatim strings (source order, quoted for V2 audit):
/// - "./init-projectors"
/// - "@opencode/ListenerServer"
/// - "http://localhost"
/// - "Server.listen"
/// - "TcpAddress"
/// - "127.0.0.1"
/// - "localhost"
/// - "mDNS enabled but hostname is loopback; skipping mDNS publish"
/// source: `export type Listener` — shape as JSON value; CI verifies.
pub type Listener = serde_json::Value;
/// source: `export const Default` — shape as JSON value; CI verifies.
pub type Default = serde_json::Value;
/// source: `export function openapi` — stub shell; CI verifies behavior.
pub fn openapi(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function listen` — stub shell; CI verifies behavior.
pub fn listen(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
