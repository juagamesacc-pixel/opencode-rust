// source: src/cli/cmd/mcp.ts — exports: [McpCommand, McpListCommand, McpAuthCommand, McpAuthListCommand, McpLogoutCommand, McpAddCommand, McpDebugCommand]
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/config/config`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/config/mcp`
// PROVISIONAL pending crates/core: `@opencode-ai/core/installation/version`
// PROVISIONAL pending crates/core: `@opencode-ai/core/global`
/// verbatim strings (source order, quoted for V2 audit):
/// - "./cmd"
/// - "authenticated"
/// - "not_authenticated"
/// - "not authenticated"
/// - "unbounded"
/// - "manage MCP (Model Context Protocol) servers"
/// - "list MCP servers and their status"
/// - "Cli.mcp.list"
/// source: `export const McpCommand` — shape as JSON value; CI verifies.
pub type McpCommand = serde_json::Value;
/// source: `export const McpListCommand` — shape as JSON value; CI verifies.
pub type McpListCommand = serde_json::Value;
/// source: `export const McpAuthCommand` — shape as JSON value; CI verifies.
pub type McpAuthCommand = serde_json::Value;
/// source: `export const McpAuthListCommand` — shape as JSON value; CI verifies.
pub type McpAuthListCommand = serde_json::Value;
/// source: `export const McpLogoutCommand` — shape as JSON value; CI verifies.
pub type McpLogoutCommand = serde_json::Value;
/// source: `export const McpAddCommand` — shape as JSON value; CI verifies.
pub type McpAddCommand = serde_json::Value;
/// source: `export const McpDebugCommand` — shape as JSON value; CI verifies.
pub type McpDebugCommand = serde_json::Value;
