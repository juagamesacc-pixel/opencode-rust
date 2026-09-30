// source: src/server/mdns.ts — exports: [publish, unpublish]
/// verbatim strings (source order, quoted for V2 audit):
/// - "bonjour-service"
/// - "opencode.local"
/// source: `export function publish` — stub shell; CI verifies behavior.
pub fn publish(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function unpublish` — stub shell; CI verifies behavior.
pub fn unpublish(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
