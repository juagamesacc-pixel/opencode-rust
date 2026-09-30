// source: src/config/config.ts — exports: Interface, Service, use, node, Config
// PROVISIONAL pending crates/core (layer-node, app-node-platform,
// service-use, global, flag, fs-util, npm, installation/version,
// util/effect-flock, v1/config/*) + @/* + jsonc-parser + remeda: merge rules
// (instructions concat-dedupe), normalizeLoadedConfig (theme/keybinds/tui
// strip), $schema URL + injection regex, global candidates order, gitignore
// body, config file orders, tools→permission map (write/edit/patch → edit),
// autoshare, disable flags, env keys, messages verbatim.

/// source: "$schema": "https://opencode.ai/config.json" — verbatim.
pub const SCHEMA_URL: &str = "https://opencode.ai/config.json";

/// source: $schema injection — text.replace(/^\s*\{/, '{\n  "$schema": ...'). Verbatim.
pub const SCHEMA_INJECT: &str = "{\n  \"$schema\": \"https://opencode.ai/config.json\",";

/// source: global candidates — ["opencode.jsonc", "opencode.json", "config.json"], verbatim.
pub const GLOBAL_CANDIDATES: &[&str] = &["opencode.jsonc", "opencode.json", "config.json"];

/// source: loadGlobal file order — config.json, opencode.json, opencode.jsonc. Verbatim.
pub const GLOBAL_LOAD_ORDER: &[&str] = &["config.json", "opencode.json", "opencode.jsonc"];

/// source: legacy "config" file name — verbatim.
pub const LEGACY_CONFIG: &str = "config";

/// source: managed dir files — ["opencode.json", "opencode.jsonc"]. Verbatim.
pub const MANAGED_FILES: &[&str] = &["opencode.json", "opencode.jsonc"];

/// source: instance dir files — ["opencode.json", "opencode.jsonc"]. Verbatim.
pub const INSTANCE_FILES: &[&str] = &["opencode.json", "opencode.jsonc"];

/// source: project files name "opencode" — verbatim.
pub const PROJECT_NAME: &str = "opencode";

/// source: ensureGitignore body — verbatim lines.
pub const GITIGNORE_BODY: &str =
    "node_modules\npackage.json\npackage-lock.json\nbun.lock\n.gitignore";

/// source: OPENCODE_CONFIG_CONTENT source label — verbatim.
pub const CONFIG_CONTENT_SOURCE: &str = "OPENCODE_CONFIG_CONTENT";

/// source: OPENCODE_CONSOLE_TOKEN env — verbatim key.
pub const CONSOLE_TOKEN_ENV: &str = "OPENCODE_CONSOLE_TOKEN";

/// source: tools→permission — write/edit/patch → edit; else same name. Verbatim.
pub fn tool_permission(tool: &str) -> &str {
    match tool {
        "write" | "edit" | "patch" => "edit",
        other => other,
    }
}

/// source: autoshare — autoshare === true && !share → share = "auto". Verbatim.
pub const SHARE_AUTO: &str = "auto";

/// source: username fallback "user" — verbatim.
pub const USERNAME_FALLBACK: &str = "user";

/// source: well-known path "/.well-known/opencode" — verbatim.
pub const WELLKNOWN_PATH: &str = "/.well-known/opencode";

/// source: console config source `${url}/api/config` — verbatim suffix.
pub const CONSOLE_CONFIG_SUFFIX: &str = "/api/config";

/// source: fetch messages — verbatim templates.
pub fn fetch_failed_message(url: &str, err: &str) -> String {
    format!("failed to fetch remote config from {}: {}", url, err)
}
pub fn read_failed_message(url: &str, err: &str) -> String {
    format!("failed to read remote config from {}: {}", url, err)
}
pub fn decode_failed_message(url: &str, err: &str) -> String {
    format!("failed to decode remote config from {}: {}", url, err)
}
pub const DECODE_EXPECTED_OBJECT: &str = "expected object";

/// source: log strings — verbatim.
pub const LOG_LOADING: &str = "loading";
pub const LOG_FETCHING_REMOTE: &str = "fetching remote config";
pub const LOG_LOADED_WELLKNOWN: &str = "loaded remote config from well-known";
pub const LOG_LOADED_CUSTOM: &str = "loaded custom config";
pub const LOG_LOADED_CONTENT: &str = "loaded custom config from OPENCODE_CONFIG_CONTENT";
pub const LOG_GLOBAL_FAILED: &str = "failed to load global config, using defaults";
pub const LOG_COMPAT: &str = "configuration compatibility diagnostic";
pub const LOG_BG_DEPS_FAILED: &str = "background dependency install failed";
pub const LOG_ACCOUNT_CONFIG_FAILED: &str = "failed to fetch remote account config";
pub const LOG_PERMISSION_JSON: &str = "OPENCODE_PERMISSION contains invalid JSON, skipping";
pub const LOG_USERNAME_FAILED: &str = "failed to read system username, using fallback";

/// source: mergeConfigConcatArrays — instructions concat-dedupe. Verbatim.
pub fn merge_instructions(target: &[String], source: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for item in target.iter().chain(source.iter()) {
        if !out.contains(item) {
            out.push(item.clone());
        }
    }
    out
}

/// source: normalizeLoadedConfig — strip theme/keybinds/tui when any present. Verbatim.
pub const LEGACY_TOP_KEYS: &[&str] = &["theme", "keybinds", "tui"];

/// source: plugin pin "@opencode-ai/plugin" — verbatim.
pub const PLUGIN_PACKAGE: &str = "@opencode-ai/plugin";

/// source: Interface — get/getGlobal/getConsoleState/update/updateGlobal/
/// invalidate/directories/waitForDependencies, verbatim.
pub trait Interface {
    fn directories(&self) -> Vec<String>;
}

/// source: Service "@opencode/Config" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Config";

/// source: node deps — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[
    "@opencode-ai/core/fs-util.FSUtil",
    "@/auth.Auth",
    "@/account/account.Account",
    "@/env.Env",
    "@opencode-ai/core/npm.Npm",
    "@opencode-ai/core/effect/app-node-platform.httpClient",
];
