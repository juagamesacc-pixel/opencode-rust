//! Rust port of `packages/core/src/config/plugin/external.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.
//! Plugin id "config-plugin" verbatim.
pub const PLUGIN_ID: &str = "config-plugin";
pub fn resolve_package(package: &str, directory: &str) -> String {
    if package.starts_with("file://") {
        return package.trim_start_matches("file://").to_string();
    }
    if package.starts_with("./") || package.starts_with("../") {
        return format!("{}/{}", directory, package);
    }
    package.to_string()
}
