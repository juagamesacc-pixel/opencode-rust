// source: src/tool/task.ts — exports: [TaskPromptOps, Parameters, TaskTool]
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/database/database`
/// verbatim strings (source order, quoted for V2 audit):
/// - "./tool"
/// - "Background mode: background=true launches the subagent asynchronously and returns immediately."
/// - "Foreground is the default; use it when you need the result before continuing."
/// - "Use background only for independent work that can run while you continue elsewhere."
/// - "You will be notified automatically when it finishes."
/// - "The task is working in the background. You will be notified automatically when it finishes."
/// - "Work on non-overlapping tasks, or briefly tell the user what you launched and end your response."
/// - "Additional context sent to the running background task."
use serde::{Deserialize, Serialize};

/// source: `src/tool/task.txt` — verbatim passthrough via include_str! (same relative path).
pub const PROMPT_TEXT: &str = include_str!(r"task.txt");
/// source: `export interface TaskPromptOps` — shape as JSON value; CI verifies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPromptOps {
    pub value: serde_json::Value,
}
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export const TaskTool` — shape as JSON value; CI verifies.
pub type TaskTool = serde_json::Value;
