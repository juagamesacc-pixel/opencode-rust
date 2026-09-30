// source: src/config/tui.ts — exports: Info, Resolved, HostMetadata,
// Interface, Service, node, waitForDependencies, get, pluginOrigins, TuiConfig
// PROVISIONAL pending tui config + core (app-node-builder, layer-node, flag,
// global, fs-util, npm, installation/version, runtime) + @/*: normalize()
// (tui-key flatten, non-record drop), dropUnknownKeybinds, pluginScope
// (commented worktree line preserved as note), 4-step merge order, resolve
// options { terminalSuspend: platform !== win32 }, plugin package pin,
// log strings verbatim.

/// source: Service "@opencode/TuiConfig" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/TuiConfig";

/// source: node deps [Npm.node, FSUtil.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[
    "@opencode-ai/core/npm.Npm",
    "@opencode-ai/core/fs-util.FSUtil",
];

/// source: pluginScope() — contains(directory, file) → "local" else "global".
/// (worktree line is commented out in source — preserved as note, not logic.)
pub fn plugin_scope(directory: &str, file: &str) -> &'static str {
    if crate::util::filesystem::contains(directory, file) {
        return "local";
    }
    "global"
}

/// source: normalize() — no "tui" key → as-is; non-record tui → delete;
/// else { ...tui, ...data } (data wins). Verbatim.
pub fn normalize_has_tui(data_has_tui: bool, tui_is_record: bool) -> &'static str {
    if !data_has_tui {
        return "as-is";
    }
    if !tui_is_record {
        return "drop";
    }
    "flatten"
}

/// source: merge steps — 1 global, 2 OPENCODE_TUI_CONFIG override, 3 project
/// root-first, 4 .opencode dirs + CONFIG_DIR. Verbatim order.
pub const MERGE_STEPS: &[&str] = &["global", "override", "project", "dirs"];

/// source: OPENCODE_TUI_CONFIG flag — verbatim key.
pub const TUI_CONFIG_ENV: &str = "OPENCODE_TUI_CONFIG";

/// source: resolve option terminalSuspend = platform !== "win32" — verbatim.
pub fn terminal_suspend(platform: &str) -> bool {
    platform != "win32"
}

/// source: plugin pin "@opencode-ai/plugin" — verbatim.
pub const PLUGIN_PACKAGE: &str = "@opencode-ai/plugin";

/// source: log strings — verbatim.
pub const LOG_SKIPPING_INVALID: &str = "skipping invalid tui config";
pub const LOG_READ_FAILED: &str = "failed to read tui config";
pub const LOG_LOADING: &str = "loading tui config";
pub const LOG_APPLYING: &str = "applying tui config";
pub const LOG_CUSTOM: &str = "loaded custom tui config";

/// source: Interface — get/pluginOrigins/waitForDependencies, verbatim.
pub trait Interface {
    fn plugin_origins(&self) -> Vec<super::plugin::Origin>;
}
