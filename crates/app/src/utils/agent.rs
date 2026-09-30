//! Rust port of `packages/app/src/utils/agent.ts` (opencode v1.18.30).
//!
//! Source 44 lines: `agentColor`, `messageAgentColor` (+ private
//! `defaults`/`palette`/`tone`). Pure logic ported verbatim.
//! Original file: `packages/app/src/utils/agent.ts`

#![allow(dead_code)]

const DEFAULTS: &[(&str, &str)] = &[
    ("ask", "var(--icon-agent-ask-base)"),
    ("build", "var(--icon-agent-build-base)"),
    ("docs", "var(--icon-agent-docs-base)"),
    ("plan", "var(--icon-agent-plan-base)"),
];

const PALETTE: &[&str] = &[
    "var(--icon-agent-ask-base)",
    "var(--icon-agent-build-base)",
    "var(--icon-agent-docs-base)",
    "var(--icon-agent-plan-base)",
    "var(--syntax-info)",
    "var(--syntax-success)",
    "var(--syntax-warning)",
    "var(--syntax-property)",
    "var(--syntax-constant)",
    "var(--text-diff-add-base)",
    "var(--text-diff-delete-base)",
    "var(--icon-warning-base)",
];

fn tone(name: &str) -> &'static str {
    let mut hash: u32 = 0;
    for ch in name.chars() {
        hash = hash.wrapping_mul(31).wrapping_add(ch as u32);
    }
    PALETTE[(hash as usize) % PALETTE.len()]
}

fn default_for(name: &str) -> Option<&'static str> {
    DEFAULTS.iter().find(|(k, _)| *k == name).map(|(_, v)| *v)
}

/// Mirrors `agentColor(name, custom?)`.
pub fn agent_color(name: &str, custom: Option<&str>) -> String {
    if let Some(custom) = custom {
        return custom.to_string();
    }
    if let Some(value) = default_for(name) {
        return value.to_string();
    }
    default_for(&name.to_lowercase())
        .map(str::to_string)
        .unwrap_or_else(|| tone(&name.to_lowercase()).to_string())
}

/// Mirrors the `{ role; agent? }` list item shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentListItem {
    pub role: String,
    pub agent: Option<String>,
}

/// Mirrors the `{ name; color? }` agent shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentEntry {
    pub name: String,
    pub color: Option<String>,
}

/// Mirrors `messageAgentColor(list, agents)`.
pub fn message_agent_color(
    list: Option<&[AgentListItem]>,
    agents: &[AgentEntry],
) -> Option<String> {
    let list = list?;
    for item in list.iter().rev() {
        if item.role != "user" {
            continue;
        }
        let agent = item.agent.as_deref()?;
        let custom = agents
            .iter()
            .find(|entry| entry.name == agent)
            .and_then(|entry| entry.color.as_deref());
        return Some(agent_color(agent, custom));
    }
    None
}
