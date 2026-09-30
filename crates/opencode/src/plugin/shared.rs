// source: src/plugin/shared.ts — exports: DEPRECATED_PLUGIN_PACKAGES,
// isDeprecatedPlugin, parsePluginSpecifier, PluginSource, PluginKind,
// PluginPackage, PluginEntry, pluginSource, isPathPluginSpec,
// resolvePathPluginTarget, checkPluginCompatibility, resolvePluginTarget,
// readPluginPackage, createPluginEntry, readPackageThemes, readPluginId,
// readV1Plugin, resolvePluginId (+ internals)
// PROVISIONAL pending npm-package-arg/semver/core npm/@/util: deprecated
// list, INDEX_FILES, export keys, drive-letter rule, containment error
// verbatim; semver/npm resolution as trait.

/// source: DEPRECATED_PLUGIN_PACKAGES — verbatim.
pub const DEPRECATED_PLUGIN_PACKAGES: &[&str] =
    &["opencode-openai-codex-auth", "opencode-copilot-auth"];

/// source: isDeprecatedPlugin() — spec.includes(pkg). Verbatim.
pub fn is_deprecated_plugin(spec: &str) -> bool {
    DEPRECATED_PLUGIN_PACKAGES
        .iter()
        .any(|pkg| spec.contains(pkg))
}

/// source: INDEX_FILES — verbatim order.
pub const INDEX_FILES: &[&str] = &[
    "index.ts",
    "index.tsx",
    "index.js",
    "index.mjs",
    "index.cjs",
];

/// source: export keys ["import", "default"] — verbatim order.
pub const EXPORT_KEYS: &[&str] = &["import", "default"];

/// source: PluginSource/PluginKind literals — verbatim.
pub const SOURCE_FILE: &str = "file";
pub const SOURCE_NPM: &str = "npm";
pub const KIND_SERVER: &str = "server";
pub const KIND_TUI: &str = "tui";

/// source: `Plugin ${spec} resolved ${kind} entry outside plugin directory` — verbatim.
pub fn outside_dir_message(spec: &str, kind: &str) -> String {
    format!(
        "Plugin {} resolved {} entry outside plugin directory",
        spec, kind
    )
}

/// source: parsePluginSpecifier() — alias-without-name, no-name, bare-name,
// rawSpec rules. Verbatim (npm-specific parse injected by host).
pub fn spec_version(raw: Option<&str>, name: Option<&str>) -> (Option<String>, String) {
    match (raw, name) {
        (None, _) => (None, "latest".to_string()),
        (Some(r), Some(n)) if r == n => (Some(n.to_string()), "latest".to_string()),
        (Some(r), Some(n)) => (Some(n.to_string()), r.to_string()),
        (Some(r), None) => (None, r.to_string()),
    }
}
