// source: src/cli/effect/prompt.ts — exports: [intro, outro, log, select, autocomplete, text, password, spinner]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@clack/prompts"
/// source: `export const intro` — shape as JSON value; CI verifies.
pub type intro = serde_json::Value;
/// source: `export const outro` — shape as JSON value; CI verifies.
pub type outro = serde_json::Value;
/// source: `export const log` — shape as JSON value; CI verifies.
pub type log = serde_json::Value;
/// source: `export const select` — shape as JSON value; CI verifies.
pub type select = serde_json::Value;
/// source: `export const autocomplete` — shape as JSON value; CI verifies.
pub type autocomplete = serde_json::Value;
/// source: `export const text` — shape as JSON value; CI verifies.
pub type text = serde_json::Value;
/// source: `export const password` — shape as JSON value; CI verifies.
pub type password = serde_json::Value;
/// source: `export const spinner` — shape as JSON value; CI verifies.
pub type spinner = serde_json::Value;
