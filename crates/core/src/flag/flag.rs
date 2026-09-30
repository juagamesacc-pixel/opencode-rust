//! Rust port of `packages/core/src/flag/flag.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

pub fn truthy(key: &str) -> bool {
    match std::env::var(key).ok().map(|v| v.to_lowercase()) {
        Some(v) => v == "true" || v == "1",
        None => false,
    }
}

fn env_opt(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

fn enabled_by_experimental(key: &str) -> bool {
    match std::env::var(key).ok() {
        Some(_) => truthy(key),
        None => truthy("OPENCODE_EXPERIMENTAL"),
    }
}

pub fn otel_exporter_otlp_endpoint() -> Option<String> {
    env_opt("OTEL_EXPORTER_OTLP_ENDPOINT")
}
pub fn otel_exporter_otlp_headers() -> Option<String> {
    env_opt("OTEL_EXPORTER_OTLP_HEADERS")
}
pub fn opencode_auto_heap_snapshot() -> bool {
    truthy("OPENCODE_AUTO_HEAP_SNAPSHOT")
}
pub fn opencode_git_bash_path() -> Option<String> {
    env_opt("OPENCODE_GIT_BASH_PATH")
}
pub fn opencode_config() -> Option<String> {
    env_opt("OPENCODE_CONFIG")
}
pub fn opencode_config_content() -> Option<String> {
    env_opt("OPENCODE_CONFIG_CONTENT")
}
pub fn opencode_disable_autoupdate() -> bool {
    truthy("OPENCODE_DISABLE_AUTOUPDATE")
}
pub fn opencode_always_notify_update() -> bool {
    truthy("OPENCODE_ALWAYS_NOTIFY_UPDATE")
}
pub fn opencode_disable_prune() -> bool {
    truthy("OPENCODE_DISABLE_PRUNE")
}
pub fn opencode_disable_terminal_title() -> bool {
    truthy("OPENCODE_DISABLE_TERMINAL_TITLE")
}
pub fn opencode_show_ttfd() -> bool {
    truthy("OPENCODE_SHOW_TTFD")
}
pub fn opencode_disable_autocompact() -> bool {
    truthy("OPENCODE_DISABLE_AUTOCOMPACT")
}
pub fn opencode_disable_models_fetch() -> bool {
    truthy("OPENCODE_DISABLE_MODELS_FETCH")
}
pub fn opencode_disable_mouse() -> bool {
    truthy("OPENCODE_DISABLE_MOUSE")
}
pub fn opencode_fake_vcs() -> Option<String> {
    env_opt("OPENCODE_FAKE_VCS")
}
pub fn opencode_server_password() -> Option<String> {
    env_opt("OPENCODE_SERVER_PASSWORD")
}
pub fn opencode_server_username() -> Option<String> {
    env_opt("OPENCODE_SERVER_USERNAME")
}
pub fn opencode_disable_fff() -> bool {
    match env_opt("OPENCODE_DISABLE_FFF") {
        Some(_) => truthy("OPENCODE_DISABLE_FFF"),
        None => cfg!(windows),
    }
}
pub fn opencode_disable_copy_on_select() -> bool {
    match env_opt("OPENCODE_EXPERIMENTAL_DISABLE_COPY_ON_SELECT") {
        Some(_) => truthy("OPENCODE_EXPERIMENTAL_DISABLE_COPY_ON_SELECT"),
        None => cfg!(windows),
    }
}
pub fn opencode_models_url() -> Option<String> {
    env_opt("OPENCODE_MODELS_URL")
}
pub fn opencode_models_path() -> Option<String> {
    env_opt("OPENCODE_MODELS_PATH")
}
pub fn opencode_db() -> Option<String> {
    env_opt("OPENCODE_DB")
}
pub fn opencode_workspace_id() -> Option<String> {
    env_opt("OPENCODE_WORKSPACE_ID")
}
pub fn opencode_experimental_workspaces() -> bool {
    enabled_by_experimental("OPENCODE_EXPERIMENTAL_WORKSPACES")
}
pub fn opencode_disable_project_config() -> bool {
    truthy("OPENCODE_DISABLE_PROJECT_CONFIG")
}
pub fn opencode_experimental_references() -> bool {
    enabled_by_experimental("OPENCODE_EXPERIMENTAL_REFERENCES")
}
pub fn opencode_tui_config() -> Option<String> {
    env_opt("OPENCODE_TUI_CONFIG")
}
pub fn opencode_config_dir() -> Option<String> {
    env_opt("OPENCODE_CONFIG_DIR")
}
pub fn opencode_pure() -> bool {
    truthy("OPENCODE_PURE")
}
pub fn opencode_permission() -> Option<String> {
    env_opt("OPENCODE_PERMISSION")
}
pub fn opencode_plugin_meta_file() -> Option<String> {
    env_opt("OPENCODE_PLUGIN_META_FILE")
}
pub fn opencode_client() -> String {
    env_opt("OPENCODE_CLIENT").unwrap_or_else(|| "cli".to_string())
}

/// Source Flag object shape preserved via accessor functions above.
/// Additional getters for experimental flags that source defines as Config.boolean with defaults.
pub fn opencode_experimental_filewatcher() -> bool {
    // Config.boolean withDefault false -> env var boolean default false
    truthy("OPENCODE_EXPERIMENTAL_FILEWATCHER")
}
pub fn opencode_experimental_disable_filewatcher() -> bool {
    truthy("OPENCODE_EXPERIMENTAL_DISABLE_FILEWATCHER")
}
