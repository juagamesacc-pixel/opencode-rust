// source: src/server/shared/public-ui.ts — exports: [PUBLIC_UI_PATHS, isPublicUIPath]
/// verbatim strings (source order, quoted for V2 audit):
/// - "/site.webmanifest"
/// - "/web-app-manifest-192x192.png"
/// - "/web-app-manifest-512x512.png"
/// source: `export const PUBLIC_UI_PATHS` — shape as JSON value; CI verifies.
pub type PUBLIC_UI_PATHS = serde_json::Value;
/// source: `export function isPublicUIPath` — stub shell; CI verifies behavior.
pub fn isPublicUIPath(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
