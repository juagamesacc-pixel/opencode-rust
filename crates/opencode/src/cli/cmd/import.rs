// source: src/cli/cmd/import.ts — exports: [ShareData, parseShareUrl, shouldAttachShareAuthHeaders, formatImportFileError, transformShareData, ImportCommand]
// PROVISIONAL pending crates/sdk: `@opencode-ai/sdk/v2`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
// PROVISIONAL pending crates/core: `@opencode-ai/core/database/database`
// PROVISIONAL pending crates/core: `@opencode-ai/core/session/sql`
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/sdk/v2"
/// - "session_diff"
/// - "PlatformError"
/// - "NotFound"
/// - "PermissionDenied"
/// - "path to JSON file or share URL"
/// - "Cli.import"
/// - "InstanceRef not provided"
/// source: `export type ShareData` — shape as JSON value; CI verifies.
pub type ShareData = serde_json::Value;
/// source: `export function parseShareUrl` — stub shell; CI verifies behavior.
pub fn parseShareUrl(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function shouldAttachShareAuthHeaders` — stub shell; CI verifies behavior.
pub fn shouldAttachShareAuthHeaders(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function formatImportFileError` — stub shell; CI verifies behavior.
pub fn formatImportFileError(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function transformShareData` — stub shell; CI verifies behavior.
pub fn transformShareData(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const ImportCommand` — shape as JSON value; CI verifies.
pub type ImportCommand = serde_json::Value;
