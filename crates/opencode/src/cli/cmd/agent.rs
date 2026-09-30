// source: src/cli/cmd/agent.ts — exports: [AgentCommand]
// PROVISIONAL pending crates/core: `@opencode-ai/core/global`
// PROVISIONAL pending external `yargs` (host-provided; no new dep)
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "./cmd"
/// - "subagent"
/// - "webfetch"
/// - "todowrite"
/// - "websearch"
/// - "create a new agent"
/// - "directory path to generate the agent file"
/// - "description"
/// source: `export const AgentCommand` — shape as JSON value; CI verifies.
pub type AgentCommand = serde_json::Value;
