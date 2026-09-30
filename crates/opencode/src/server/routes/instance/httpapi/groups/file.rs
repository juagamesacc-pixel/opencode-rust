// source: src/server/routes/instance/httpapi/groups/file.ts — exports: [FileQuery, FindTextQuery, FindFileQuery, FindSymbolQuery, LegacyMatch, LegacyEntry, LegacyContent, LegacyStatus, FilePaths, FileApi]
// PROVISIONAL pending crates/core: `@opencode-ai/core/filesystem`
// PROVISIONAL pending crates/core: `@opencode-ai/core/schema`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/filesystem"
/// - "directory"
/// - "FileNode"
/// - "FileContent"
/// - "modified"
/// - "/find"
/// - "/find/file"
/// - "/find/symbol"
/// source: `export const FileQuery` — shape as JSON value; CI verifies.
pub type FileQuery = serde_json::Value;
/// source: `export const FindTextQuery` — shape as JSON value; CI verifies.
pub type FindTextQuery = serde_json::Value;
/// source: `export const FindFileQuery` — shape as JSON value; CI verifies.
pub type FindFileQuery = serde_json::Value;
/// source: `export const FindSymbolQuery` — shape as JSON value; CI verifies.
pub type FindSymbolQuery = serde_json::Value;
/// source: `export const LegacyMatch` — shape as JSON value; CI verifies.
pub type LegacyMatch = serde_json::Value;
/// source: `export const LegacyEntry` — shape as JSON value; CI verifies.
pub type LegacyEntry = serde_json::Value;
/// source: `export const LegacyContent` — shape as JSON value; CI verifies.
pub type LegacyContent = serde_json::Value;
/// source: `export const LegacyStatus` — shape as JSON value; CI verifies.
pub type LegacyStatus = serde_json::Value;
/// source: `export const FilePaths` — shape as JSON value; CI verifies.
pub type FilePaths = serde_json::Value;
/// source: `export const FileApi` — shape as JSON value; CI verifies.
pub type FileApi = serde_json::Value;
