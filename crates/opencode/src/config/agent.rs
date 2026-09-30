// source: src/config/agent.ts — exports: load, loadMode, ConfigAgent
// PROVISIONAL pending crates/core (util/glob, v1/config/agent) + ./entry-name
// + ./markdown + ./parse: glob patterns + prompt trim + mode="primary"
// override verbatim.

/// source: agent glob "{agent,agents}/**/*.md" — verbatim.
pub const AGENT_GLOB: &str = "{agent,agents}/**/*.md";
/// source: agent prefixes ["agent/", "agents/"] — verbatim.
pub const AGENT_PREFIXES: &[&str] = &["agent/", "agents/"];
/// source: mode glob "{mode,modes}/*.md" — verbatim.
pub const MODE_GLOB: &str = "{mode,modes}/*.md";
/// source: mode prefixes ["mode/", "modes/"] — verbatim.
pub const MODE_PREFIXES: &[&str] = &["mode/", "modes/"];
/// source: loadMode forces mode "primary" — verbatim.
pub const MODE_FORCED: &str = "primary";
/// source: prompt = md.content.trim() — verbatim rule.
pub fn prompt_of(content: &str) -> String {
    content.trim().to_string()
}
/// source: entry name from scanned path — verbatim composition.
pub fn entry_name(dir: &str, item: &str, prefixes: &[&str]) -> String {
    let rel = item.strip_prefix(dir.trim_end_matches('/')).unwrap_or(item);
    let rel = rel.trim_start_matches('/');
    super::entry_name::config_entry_name_from_path(rel, prefixes)
}
