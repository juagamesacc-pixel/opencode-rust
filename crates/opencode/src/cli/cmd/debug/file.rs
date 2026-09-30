// source: src/cli/cmd/debug/file.ts — exports: [FileCommand]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/filesystem`
// PROVISIONAL pending crates/core: `@opencode-ai/core/location-services`
// PROVISIONAL pending crates/core: `@opencode-ai/core/location`
// PROVISIONAL pending crates/core: `@opencode-ai/core/schema`
/// verbatim strings (source order, quoted for V2 audit):
/// - "search <query>"
/// - "search files by query"
/// - "Search query"
/// - "Cli.debug.file.search"
/// - "read <path>"
/// - "read file contents as JSON"
/// - "File path to read"
/// - "Cli.debug.file.read"
/// source: `export const FileCommand` — shape as JSON value; CI verifies.
pub type FileCommand = serde_json::Value;
