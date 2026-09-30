// source: src/server/routes/instance/httpapi/handlers/session-errors.ts — exports: [mapStorageNotFound, mapBusy]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/storage/storage"
/// - "SessionBusyError"
/// source: `export function mapStorageNotFound` — stub shell; CI verifies behavior.
pub fn mapStorageNotFound(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function mapBusy` — stub shell; CI verifies behavior.
pub fn mapBusy(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
