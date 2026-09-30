// source: src/cli/cmd/debug/index.ts — exports: [DebugCommand]
// PROVISIONAL pending crates/core: `@opencode-ai/core/global`
// PROVISIONAL pending crates/core: `@opencode-ai/core/installation/version`
// PROVISIONAL pending crates/core: `@opencode-ai/core/flag/flag`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/global"
/// - "debugging and troubleshooting tools"
/// - "wait indefinitely (for debugging)"
/// - "Cli.debug.wait"
/// - "show debug information"
/// - "Cli.debug.info"
/// - "@/config/config"
/// - "@/config/plugin"
/// source: `export const DebugCommand` — shape as JSON value; CI verifies.
pub type DebugCommand = serde_json::Value;
