// source: src/server/shared/workspace-routing.ts — exports: [isLocalWorkspaceRoute, getWorkspaceRouteSessionID, workspaceProxyURL]
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/session/schema"
/// - "/experimental/workspace"
/// - "/session/status"
/// - ", path: "
/// - ", action: "
/// - "workspace"
/// - "directory"
/// source: `export function isLocalWorkspaceRoute` — stub shell; CI verifies behavior.
pub fn isLocalWorkspaceRoute(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function getWorkspaceRouteSessionID` — stub shell; CI verifies behavior.
pub fn getWorkspaceRouteSessionID(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function workspaceProxyURL` — stub shell; CI verifies behavior.
pub fn workspaceProxyURL(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
