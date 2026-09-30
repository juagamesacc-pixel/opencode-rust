// source: src/server/routes/instance/httpapi/middleware/proxy.ts — exports: [websocket, http]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/socket/Socket` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/server/proxy-util"
/// - " || request.method === "
/// - "content-length"
/// - "content-type"
/// - "1 second"
/// - "SocketError"
/// - "SocketCloseError"
/// - "unbounded"
/// source: `export function websocket` — stub shell; CI verifies behavior.
pub fn websocket(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function http` — stub shell; CI verifies behavior.
pub fn http(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
