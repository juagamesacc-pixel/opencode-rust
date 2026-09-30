//! Rust port of `packages/core/src/config/plugin/reference.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.
//! Plugin id "core/config-reference" verbatim.
pub const PLUGIN_ID: &str = "core/config-reference";
pub fn valid_alias(name: &str) -> bool {
    !name.is_empty()
        && !name.contains('/')
        && !name.contains(' ')
        && !name.contains('`')
        && !name.contains(',')
}
pub fn is_local(entry: &str) -> bool {
    entry.starts_with('.') || entry.starts_with('/') || entry.starts_with('~')
}
pub fn local_path(directory: &str, home: &str, value: &str) -> String {
    if let Some(stripped) = value.strip_prefix("~/") {
        return format!("{}/{}", home, stripped);
    }
    if value.starts_with('/') {
        return value.to_string();
    }
    format!("{}/{}", directory, value)
}
