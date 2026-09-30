// source: src/plugin/tui/runtime.ts — exports: init, list,
// activatePlugin, deactivatePlugin, addPlugin, installPlugin, dispose,
// createLegacyTuiPluginHost, TuiPluginRuntime (+ internals per source)
// PROVISIONAL (1131-line host): DISPOSE_TIMEOUT_MS, KV_KEY
// "plugin_enabled", EMPTY_TUI, fail/warn message join, theme dest rules
// (local vs config/themes), function table verbatim.

/// source: DISPOSE_TIMEOUT_MS = 5000 — verbatim.
pub const DISPOSE_TIMEOUT_MS: u64 = 5000;
/// source: KV_KEY = "plugin_enabled" — verbatim.
pub const KV_KEY: &str = "plugin_enabled";

/// source: fail() — `${message}: ${errorMessage}`. Verbatim rule.
pub fn fail_message(message: &str, err: &str) -> String {
    format!("{}: {}", message, err)
}

/// source: theme dest — local vs config/themes + `${name}.json`. Verbatim rule.
pub fn theme_dest(is_local: bool, local_dir: &str, config_themes: &str, name: &str) -> String {
    if is_local {
        format!("{}/{}.json", local_dir.trim_end_matches('/'), name)
    } else {
        format!("{}/{}.json", config_themes.trim_end_matches('/'), name)
    }
}

/// source: runtime fns — verbatim names/order.
pub const RUNTIME_FNS: &[&str] = &[
    "init",
    "list",
    "activatePlugin",
    "deactivatePlugin",
    "addPlugin",
    "installPlugin",
    "dispose",
    "createLegacyTuiPluginHost",
];
