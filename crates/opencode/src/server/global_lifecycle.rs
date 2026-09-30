// source: src/server/global-lifecycle.ts — exports: [emitGlobalDisposed, disposeAllInstancesAndEmitGlobalDisposed]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/bus/global"
/// - "Server.disposeAllInstancesAndEmitGlobalDisposed"
/// - "global disposal failed"
/// source: `export const emitGlobalDisposed` — shape as JSON value; CI verifies.
pub type emitGlobalDisposed = serde_json::Value;
/// source: `export const disposeAllInstancesAndEmitGlobalDisposed` — shape as JSON value; CI verifies.
pub type disposeAllInstancesAndEmitGlobalDisposed = serde_json::Value;
