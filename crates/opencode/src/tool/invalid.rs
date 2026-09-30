// source: src/tool/invalid.ts — exports: [Parameters, InvalidTool]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "Do not use"
/// - "Invalid Tool"
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export const InvalidTool` — shape as JSON value; CI verifies.
pub type InvalidTool = serde_json::Value;
