// source: src/cli/cmd/export.ts — exports: [ExportCommand]
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/session/session"
/// - "file-text"
/// - "file-path"
/// - "file-symbol"
/// - "resource"
/// - "file-client"
/// - "file-uri"
/// - "file-url"
/// source: `export const ExportCommand` — shape as JSON value; CI verifies.
pub type ExportCommand = serde_json::Value;
