// source: src/server/routes/instance/httpapi/handlers/file.ts — exports: [fileHandlers]
// PROVISIONAL pending crates/core: `@opencode-ai/core/filesystem`
// PROVISIONAL pending crates/core: `@opencode-ai/core/location-services`
// PROVISIONAL pending crates/core: `@opencode-ai/core/ripgrep`
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
// PROVISIONAL pending crates/core: `@opencode-ai/core/location`
// PROVISIONAL pending crates/core: `@opencode-ai/core/schema`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/effect/instance-state"
/// - "FileHttpApi.findText"
/// - "FileHttpApi.findFile"
/// - "directory"
/// - "find file"
/// - "FileHttpApi.findSymbol"
/// - "FileHttpApi.list"
/// - ".gitignore"
/// source: `export const fileHandlers` — shape as JSON value; CI verifies.
pub type fileHandlers = serde_json::Value;
