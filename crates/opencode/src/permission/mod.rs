// source: src/permission/index.ts — exports: Event, Interface, evaluate,
// Service, fromConfig, merge, disabled, visibleTools, node, Permission
// PROVISIONAL pending crates/core (layer-node, v1/config/permission,
// v1/permission, util/wildcard) + @/event-v2-bridge + @/effect/instance-state:
// evaluate (findLast + ask-fallback), expand, fromConfig, merge, disabled
// (edit/read groups), visibleTools verbatim; ask/reply/list as trait.

use serde::{Deserialize, Serialize};

/// source: Rule { permission, pattern, action } — verbatim shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub permission: String,
    pub pattern: String,
    pub action: String,
}

/// source: evaluate + ask-fallback live in ./evaluate (mirrors
/// `export { evaluate } from "."` re-export chain). Verbatim.
pub use evaluate::{ask_fallback, evaluate};

/// source: expand() — ~, ~/ and $HOME prefixes, verbatim.
pub fn expand(pattern: &str, home: &str) -> String {
    if let Some(_rest) = pattern.strip_prefix("~/") {
        return format!("{}{}", home, &pattern[1..]);
    }
    if pattern == "~" {
        return home.to_string();
    }
    if let Some(rest) = pattern.strip_prefix("$HOME/") {
        return format!("{}/{}", home, rest);
    }
    if let Some(rest) = pattern.strip_prefix("$HOME") {
        return format!("{}{}", home, rest);
    }
    pattern.to_string()
}

/// source: fromConfig() — string → {permission: key, action, pattern: "*"};
/// map → expanded patterns. Verbatim.
pub fn from_config(entries: &[(&str, Vec<(String, String)>)], home: &str) -> Vec<Rule> {
    let mut ruleset = Vec::new();
    for (key, value) in entries {
        if value.is_empty() {
            continue;
        }
        for (pattern, action) in value {
            ruleset.push(Rule {
                permission: key.to_string(),
                pattern: expand(pattern, home),
                action: action.clone(),
            });
        }
    }
    ruleset
}

/// source: string-shorthand fromConfig branch — verbatim helper.
pub fn from_config_string(key: &str, action: &str) -> Rule {
    Rule {
        permission: key.to_string(),
        action: action.to_string(),
        pattern: "*".to_string(),
    }
}

/// source: merge() — rulesets.flat(), verbatim.
pub fn merge(rulesets: Vec<Vec<Rule>>) -> Vec<Rule> {
    rulesets.into_iter().flatten().collect()
}

/// source: disabled() — edit group [edit, write, apply_patch], read group
/// [list_mcp_resources, list_mcp_resource_templates, read_mcp_resource],
/// else tool name; deny + pattern "*" via findLast (permission-only). Verbatim.
pub const EDIT_GROUP: &[&str] = &["edit", "write", "apply_patch"];
pub const READ_GROUP: &[&str] = &[
    "list_mcp_resources",
    "list_mcp_resource_templates",
    "read_mcp_resource",
];

pub fn permission_of_tool(tool: &str) -> &str {
    if EDIT_GROUP.contains(&tool) {
        return "edit";
    }
    if READ_GROUP.contains(&tool) {
        return "read";
    }
    tool
}

/// source: disabled() — verbatim.
pub fn disabled(tools: &[String], ruleset: &[Rule]) -> Vec<String> {
    tools
        .iter()
        .filter(|tool| {
            let permission = permission_of_tool(tool);
            let rule = ruleset
                .iter()
                .rev()
                .find(|r| crate::util::wildcard::match_(permission, &r.permission));
            matches!(rule, Some(r) if r.pattern == "*" && r.action == "deny")
        })
        .cloned()
        .collect()
}

/// source: visibleTools() — verbatim.
pub fn visible_tools(tools: &[String], ruleset: &[Rule]) -> Vec<String> {
    let hidden = disabled(tools, ruleset);
    tools
        .iter()
        .filter(|t| !hidden.contains(t))
        .cloned()
        .collect()
}

/// source: Interface — ask/reply/list, verbatim names.
pub trait Interface {
    fn list_pending(&self) -> Vec<String>;
}

/// source: Service "@opencode/Permission" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Permission";

/// source: node deps [EventV2Bridge.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &["@/event-v2-bridge.EventV2Bridge"];

pub mod arity;
pub mod evaluate;
