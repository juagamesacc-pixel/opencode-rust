// source: src/cli/cmd/debug/agent.ts — exports: [AgentCommand]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "agent <name>"
/// - "show agent configuration details"
/// - "Agent name"
/// - "Tool id to execute"
/// - "Tool params as JSON or a JS object literal"
/// - "./agent.handler"
/// source: `export const AgentCommand` — shape as JSON value; CI verifies.
pub type AgentCommand = serde_json::Value;
