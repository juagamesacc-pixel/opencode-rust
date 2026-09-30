// source: src/server/routes/instance/httpapi/groups/project.ts — exports: [ProjectApi]
// PROVISIONAL pending crates/core: `@opencode-ai/core/project`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/project/project"
/// - "/project"
/// - "List of projects"
/// - "project.list"
/// - "List all projects"
/// - "Get a list of projects that have been opened with OpenCode."
/// - "Current project information"
/// - "project.current"
/// source: `export const ProjectApi` — shape as JSON value; CI verifies.
pub type ProjectApi = serde_json::Value;
