// source: src/tool/skill.ts — exports: [Parameters, SkillTool]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/ripgrep`
/// verbatim strings (source order, quoted for V2 audit):
/// - "The name of the skill from available_skills"
/// - "Skill.NotFoundError"
/// - "!**/SKILL.md"
/// - "${info.name}"
/// - "Relative paths in this skill (e.g., scripts/, reference/) are relative to this base directory."
/// - "Note: file list is sampled."
/// - "<skill_files>"
/// - "</skill_files>"
/// source: `src/tool/skill.txt` — verbatim passthrough via include_str! (same relative path).
pub const PROMPT_TEXT: &str = include_str!(r"skill.txt");
/// source: `export const Parameters` — shape as JSON value; CI verifies.
pub type Parameters = serde_json::Value;
/// source: `export const SkillTool` — shape as JSON value; CI verifies.
pub type SkillTool = serde_json::Value;
