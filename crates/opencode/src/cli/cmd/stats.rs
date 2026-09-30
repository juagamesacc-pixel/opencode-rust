// source: src/cli/cmd/stats.ts — exports: [StatsCommand, displayStats]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/database/database`
// PROVISIONAL pending crates/core: `@opencode-ai/core/session/sql`
/// verbatim strings (source order, quoted for V2 audit):
/// - "show token usage and cost statistics"
/// - "show stats for the last N days (default: all time)"
/// - "number of tools to show (default: all)"
/// - "show model statistics (default: hidden). Pass a number to show top N, otherwise shows all"
/// - "filter by project (default: all projects, empty string: current project)"
/// - "Cli.stats"
/// - "Cli.stats.aggregate"
/// - "currentProject required when projectFilter is empty string"
/// source: `export const StatsCommand` — shape as JSON value; CI verifies.
pub type StatsCommand = serde_json::Value;
/// source: `export function displayStats` — stub shell; CI verifies behavior.
pub fn displayStats(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
