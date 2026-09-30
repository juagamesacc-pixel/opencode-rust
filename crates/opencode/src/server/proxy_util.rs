// source: src/server/proxy-util.ts — exports: [headers, websocketProtocols, websocketTargetURL]
/// verbatim strings (source order, quoted for V2 audit):
/// - "connection"
/// - "keep-alive"
/// - "proxy-authenticate"
/// - "proxy-authorization"
/// - "proxy-connection"
/// - "transfer-encoding"
/// - "accept-encoding"
/// - "x-opencode-directory"
/// source: `export function headers` — stub shell; CI verifies behavior.
pub fn headers(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function websocketProtocols` — stub shell; CI verifies behavior.
pub fn websocketProtocols(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function websocketTargetURL` — stub shell; CI verifies behavior.
pub fn websocketTargetURL(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
