// source: src/cli/cmd/debug/agent.handler.ts — exports: [debugAgent]
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/permission`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/v1/permission"
/// - "Cli.debug.agent"
/// - "Cli.debug.agent.body"
/// - "${basename(process.execPath)} agent list"
/// - "Cli.debug.agent.getAvailableTools"
/// - "No providers found"
/// - "Tool params must be an object."
/// - "Cli.debug.agent.createToolContext"
/// source: `export const debugAgent` — shape as JSON value; CI verifies.
pub type debugAgent = serde_json::Value;
