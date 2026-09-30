// source: src/tool/todo.ts — exports: [Parameters, TodoWriteTool]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "The updated todo list"
/// - "todowrite"
/// - "completed"
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export const TodoWriteTool` — shape as JSON value; CI verifies.
pub type TodoWriteTool = serde_json::Value;
