//! Rust port of `packages/core/src/config/plugin/skill.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.
//! Plugin id "config-skill" verbatim.
pub const PLUGIN_ID: &str = "config-skill";
pub fn is_url(item: &str) -> bool {
    item.starts_with("http://") || item.starts_with("https://")
}
