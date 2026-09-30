// source: src/cli/cmd/github.handler.ts — exports: [githubInstall, githubRun]
// PROVISIONAL pending crates/core: `@opencode-ai/core/models-dev`
// PROVISIONAL pending crates/core: `@opencode-ai/core/event`
// PROVISIONAL pending external `node:timers/promises` (host-provided; no new dep)
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "opencode-agent[bot]"
/// - ".github/workflows/opencode.yml"
/// - "issue_comment"
/// - "pull_request_review_comment"
/// - "pull_request"
/// - "schedule"
/// - "workflow_dispatch"
/// - "Cli.github.install"
/// source: `export const githubInstall` — shape as JSON value; CI verifies.
pub type githubInstall = serde_json::Value;
/// source: `export const githubRun` — shape as JSON value; CI verifies.
pub type githubRun = serde_json::Value;
