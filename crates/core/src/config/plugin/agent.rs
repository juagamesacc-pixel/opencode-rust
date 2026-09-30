//! Rust port of `packages/core/src/config/plugin/agent.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.
//! Plugin id "config-agent" verbatim.

pub const PLUGIN_ID: &str = "config-agent";
pub const LEGACY_SOURCES: &[(&str, bool)] = &[
    ("{agent,agents}/**/*.md", false),
    ("{mode,modes}/*.md", true),
];
pub const AGENT_KEYS: &[&str] = &[
    "model",
    "variant",
    "request",
    "system",
    "description",
    "mode",
    "hidden",
    "color",
    "steps",
    "disabled",
    "permissions",
];
pub fn expand_home(resource: &str, home: &str) -> String {
    if resource.starts_with("~/") {
        return format!("{}{}", home, &resource[1..]);
    }
    if resource == "~" || resource == "$HOME" {
        return home.to_string();
    }
    if resource.starts_with("$HOME/") {
        return format!("{}{}", home, &resource[5..]);
    }
    if resource.starts_with("$HOME\\") {
        return format!("{}{}", home, &resource[5..]);
    }
    resource.to_string()
}
pub fn is_path_action(action: &str) -> bool {
    matches!(action, "external_directory" | "read" | "edit")
}
