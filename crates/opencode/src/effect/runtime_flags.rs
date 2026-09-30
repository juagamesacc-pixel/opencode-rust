// source: src/effect/runtime-flags.ts — exports: Service, Info, layer,
// node, RuntimeFlags ("@opencode/RuntimeFlags"; env table verbatim: bool
// defaults false, positiveInteger rule, experimental fallback, broad||direct
// pairs, experimental||enabled||legacy triples, client default "cli").
// PROVISIONAL: Effect Config provider modelled as env-map reads.

/// source: Service "@opencode/RuntimeFlags" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/RuntimeFlags";

/// source: bool flags (default false) — verbatim env names.
pub const BOOL_FLAGS: &[&str] = &[
    "OPENCODE_AUTO_SHARE",
    "OPENCODE_PURE",
    "OPENCODE_DISABLE_DEFAULT_PLUGINS",
    "OPENCODE_DISABLE_EMBEDDED_WEB_UI",
    "OPENCODE_DISABLE_EXTERNAL_SKILLS",
    "OPENCODE_DISABLE_LSP_DOWNLOAD",
    "OPENCODE_ENABLE_EXPERIMENTAL_MODELS",
    "OPENCODE_ENABLE_QUESTION_TOOL",
    "OPENCODE_EXPERIMENTAL_LSP_TY",
    "OPENCODE_EXPERIMENTAL_NATIVE_LLM",
    "OPENCODE_EXPERIMENTAL_WEBSOCKETS",
];

/// source: broad||direct pairs — verbatim.
pub const PAIRED_FLAGS: &[(&str, &str)] = &[
    (
        "OPENCODE_DISABLE_CLAUDE_CODE",
        "OPENCODE_DISABLE_CLAUDE_CODE_PROMPT",
    ),
    (
        "OPENCODE_DISABLE_CLAUDE_CODE",
        "OPENCODE_DISABLE_CLAUDE_CODE_SKILLS",
    ),
];

/// source: experimental||enabled||legacy triple — verbatim.
pub const EXA_FLAGS: &[&str] = &[
    "OPENCODE_EXPERIMENTAL",
    "OPENCODE_ENABLE_EXA",
    "OPENCODE_EXPERIMENTAL_EXA",
];
/// source: enabled||legacy pair — verbatim.
pub const PARALLEL_FLAGS: &[&str] = &["OPENCODE_ENABLE_PARALLEL", "OPENCODE_EXPERIMENTAL_PARALLEL"];

/// source: enabledByExperimental names — verbatim.
pub const EXPERIMENTAL_FALLBACK_FLAGS: &[&str] = &[
    "OPENCODE_EXPERIMENTAL",
    "OPENCODE_EXPERIMENTAL_REFERENCES",
    "OPENCODE_EXPERIMENTAL_BACKGROUND_SUBAGENTS",
    "OPENCODE_EXPERIMENTAL_LSP_TOOL",
    "OPENCODE_EXPERIMENTAL_OXFMT",
    "OPENCODE_EXPERIMENTAL_PLAN_MODE",
    "OPENCODE_EXPERIMENTAL_CODE_MODE",
    "OPENCODE_EXPERIMENTAL_EVENT_SYSTEM",
    "OPENCODE_EXPERIMENTAL_WORKSPACES",
    "OPENCODE_EXPERIMENTAL_ICON_DISCOVERY",
];

/// source: positiveInteger names — verbatim.
pub const POSITIVE_INT_FLAGS: &[&str] = &[
    "OPENCODE_EXPERIMENTAL_OUTPUT_TOKEN_MAX",
    "OPENCODE_EXPERIMENTAL_BASH_DEFAULT_TIMEOUT_MS",
];

/// source: client default "cli" — verbatim.
pub const CLIENT_ENV: &str = "OPENCODE_CLIENT";
pub const CLIENT_DEFAULT: &str = "cli";

/// source: bool() — Config.boolean default false. Verbatim rule.
pub fn read_bool(env: &std::collections::HashMap<String, String>, name: &str) -> bool {
    matches!(env.get(name).map(|v| v.as_str()), Some("true") | Some("1"))
}

/// source: positiveInteger() — integer && > 0 else undefined. Verbatim rule.
pub fn read_positive_int(
    env: &std::collections::HashMap<String, String>,
    name: &str,
) -> Option<i64> {
    env.get(name)?.parse::<i64>().ok().filter(|v| *v > 0)
}

/// source: node deps [] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[];
