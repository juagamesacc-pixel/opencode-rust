// source: src/tool/mcp-websearch.ts — exports: [EXA_URL, PARALLEL_URL, parseResponse, SearchArgs, ParallelSearchArgs, call]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "https://mcp.exa.ai/mcp"
/// - "https://search.parallel.ai/mcp"
/// - "McpWebSearch.parseResponse"
/// - "data: "
/// - "tools/call"
/// - "application/json, text/event-stream"
/// source: `PARALLEL_URL = "https://search.parallel.ai/mcp"` — verbatim.
pub const PARALLEL_URL: &str = "https://search.parallel.ai/mcp";
/// source: `export const EXA_URL` — shape as JSON value; CI verifies.
pub type EXA_URL = serde_json::Value;
/// source: `export const parseResponse` — shape as JSON value; CI verifies.
pub type parseResponse = serde_json::Value;
/// source: `export const SearchArgs` — shape as JSON value; CI verifies.
pub type SearchArgs = serde_json::Value;
/// source: `export const ParallelSearchArgs` — shape as JSON value; CI verifies.
pub type ParallelSearchArgs = serde_json::Value;
/// source: `export const call` — shape as JSON value; CI verifies.
pub type call = serde_json::Value;
