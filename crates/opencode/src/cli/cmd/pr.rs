// source: src/cli/cmd/pr.ts — exports: [PrCommand]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "pr <number>"
/// - "fetch and checkout a GitHub PR branch, then run opencode"
/// - "PR number to checkout"
/// - "Cli.pr"
/// - "Could not load instance context"
/// - "Could not find git repository. Please run this command from a git repository."
/// - "checkout"
/// - "--branch"
/// source: `export const PrCommand` — shape as JSON value; CI verifies.
pub type PrCommand = serde_json::Value;
