// source: src/tool/plan.ts — exports: [Parameters, PlanExitTool]
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "plan_exit"
/// - "Build Agent"
/// - ", description: "
/// - "Switching to build agent"
/// - "User approved switching to build agent. Wait for further instructions."
/// source: `src/tool/plan-enter.txt` — verbatim passthrough via include_str! (same relative path).
pub const PROMPT_PLAN_ENTER: &str = include_str!(r"plan-enter.txt");
/// source: `src/tool/plan-exit.txt` — verbatim passthrough via include_str! (same relative path).
pub const PROMPT_PLAN_EXIT: &str = include_str!(r"plan-exit.txt");
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export const PlanExitTool` — shape as JSON value; CI verifies.
pub type PlanExitTool = serde_json::Value;
