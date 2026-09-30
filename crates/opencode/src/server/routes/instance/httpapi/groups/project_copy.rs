// source: src/server/routes/instance/httpapi/groups/project-copy.ts — exports: [GenerateNamePayload, ProjectCopyApi]
// PROVISIONAL pending crates/core: `@opencode-ai/core/project`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/project"
/// - "projectCopyName"
/// - "generateName"
/// - "/experimental/project/:projectID/copy/generate-name"
/// - "experimental.projectCopy.generateName"
/// - "Generate project copy name"
/// - "Generate a short name for a project copy from task context."
/// - "projectCopy"
/// source: `export const GenerateNamePayload` — shape as JSON value; CI verifies.
pub type GenerateNamePayload = serde_json::Value;
/// source: `export const ProjectCopyApi` — shape as JSON value; CI verifies.
pub type ProjectCopyApi = serde_json::Value;
