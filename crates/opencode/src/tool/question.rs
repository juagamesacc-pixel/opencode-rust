// source: src/tool/question.ts — exports: [Parameters, QuestionTool]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "Questions to ask"
/// - "question"
/// - "${q.question}"
/// - "${answers[i]?.length ? answers[i].join("
/// - ") : "
/// source: `src/tool/question.txt` — verbatim passthrough via include_str! (same relative path).
pub const PROMPT_TEXT: &str = include_str!(r"question.txt");
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export const QuestionTool` — shape as JSON value; CI verifies.
pub type QuestionTool = serde_json::Value;
