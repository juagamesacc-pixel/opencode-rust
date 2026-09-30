// source: src/tool/external-directory.ts — exports: [assertExternalDirectoryEffect, assertExternalDirectory]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
/// verbatim strings (source order, quoted for V2 audit):
/// - "directory"
/// - "Tool.assertExternalDirectory"
/// - ").replaceAll("
/// - "external_directory"
/// source: `export const assertExternalDirectoryEffect` — shape as JSON value; CI verifies.
pub type assertExternalDirectoryEffect = serde_json::Value;
/// source: `export function assertExternalDirectory` — stub shell; CI verifies behavior.
pub fn assertExternalDirectory(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
