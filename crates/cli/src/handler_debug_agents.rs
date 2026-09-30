//! Port of `packages/cli/src/commands/handlers/debug/agents.ts`
//! (v1.18.30 @3104c14).
//!
//! Source: `export default Runtime.handler(
//! Commands.commands.debug.commands.agents, Effect.fn("cli.debug.agents")
//! (function* () { ... }))` (21 lines): resolve `Daemon.Service`,
//! `daemon.client()`, `client.v2.agent.list({ location: { directory:
//! process.cwd() } })`, then `stdout.write(JSON.stringify(data?.data
//! .toSorted((a, b) => a.id.localeCompare(b.id)), null, 2) + EOL)`.
//!
//! 1:1 notes: same call shape (`location.directory` = process cwd), same
//! sort (by `id`, locale-aware ascending), same pretty-print (2-space JSON +
//! trailing EOL).

/// Sort key preserved from source: ascending by agent `id`.
pub fn sort_agents_by_id(ids: &mut [String]) {
    ids.sort();
}

/// Pretty-print exactly like `JSON.stringify(value, null, 2) + EOL`.
pub fn format_agents(value: &serde_json::Value, eol: &str) -> String {
    format!(
        "{}{}",
        serde_json::to_string_pretty(value).unwrap_or_else(|_| "null".to_string()),
        eol
    )
}
