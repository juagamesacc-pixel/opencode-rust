// source: src/tool/webfetch.ts — exports: [Parameters, WebFetchTool]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "The URL to fetch content from"
/// - "markdown"
/// - "The format to return the content in (text, markdown, or html). Defaults to markdown."
/// - "Optional timeout in seconds (max 120)"
/// - "webfetch"
/// - "http://"
/// - "https://"
/// - "URL must start with http:// or https://"
/// source: `src/tool/webfetch.txt` — verbatim passthrough via include_str! (same relative path).
pub const PROMPT_TEXT: &str = include_str!(r"webfetch.txt");
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export const WebFetchTool` — shape as JSON value; CI verifies.
pub type WebFetchTool = serde_json::Value;
