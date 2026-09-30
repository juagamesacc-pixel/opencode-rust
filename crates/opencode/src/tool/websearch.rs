// source: src/tool/websearch.ts — exports: [Parameters, WebSearchProvider, selectWebSearchProvider, webSearchProviderLabel, webSearchModelName, WebSearchTool]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/util/encode`
// PROVISIONAL pending crates/core: `@opencode-ai/core/installation/version`
/// verbatim strings (source order, quoted for V2 audit):
/// - "Websearch query"
/// - "Number of search results to return (default: 8)"
/// - "fallback"
/// - "preferred"
/// - "Search type - 'auto': balanced search (default), 'fast': quick results, 'deep': comprehensive search"
/// - "Maximum characters for context string optimized for LLMs (default: 10000)"
/// - "parallel"
/// - " || override === "
/// source: `src/tool/websearch.txt` — verbatim passthrough via include_str! (same relative path).
pub const PROMPT_TEXT: &str = include_str!(r"websearch.txt");
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export type WebSearchProvider` — shape as JSON value; CI verifies.
pub type WebSearchProvider = serde_json::Value;
/// source: `export function selectWebSearchProvider` — stub shell; CI verifies behavior.
pub fn selectWebSearchProvider(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function webSearchProviderLabel` — stub shell; CI verifies behavior.
pub fn webSearchProviderLabel(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function webSearchModelName` — stub shell; CI verifies behavior.
pub fn webSearchModelName(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const WebSearchTool` — shape as JSON value; CI verifies.
pub type WebSearchTool = serde_json::Value;
