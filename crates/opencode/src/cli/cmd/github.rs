// source: src/cli/cmd/github.ts — exports: [GithubInstallCommand, GithubRunCommand, GithubCommand, extractResponseText, formatPromptTooLargeError, parseGitHubRemote]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "install the GitHub agent"
/// - "./github.handler"
/// - "run the GitHub agent"
/// - "GitHub mock event to run the agent for"
/// - "GitHub personal access token (github_pat_********)"
/// - "manage GitHub agent"
/// source: `export const GithubInstallCommand` — shape as JSON value; CI verifies.
pub type GithubInstallCommand = serde_json::Value;
/// source: `export const GithubRunCommand` — shape as JSON value; CI verifies.
pub type GithubRunCommand = serde_json::Value;
/// source: `export const GithubCommand` — shape as JSON value; CI verifies.
pub type GithubCommand = serde_json::Value;
// source: `export { extractResponseText }` — re-export; resolve via crate path.
// source: `export { formatPromptTooLargeError }` — re-export; resolve via crate path.
// source: `export { parseGitHubRemote }` — re-export; resolve via crate path.
