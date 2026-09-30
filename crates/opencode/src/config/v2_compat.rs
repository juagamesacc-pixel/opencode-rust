// source: src/config/v2-compat.ts — exports: Diagnostic, Result, lower, ConfigV2Compat
// PROVISIONAL pending effect Schema + core v1/config/*: diagnostic kinds +
// messages, key tables, lowerSelection/lowerTimeout/lowerServer/lowerAgent/
/// lowerCommand pure rules verbatim; decode-dependent branches as data rules.
use serde::{Deserialize, Serialize};

/// source: Diagnostic { kind, path, message } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub kind: String,
    pub path: Vec<String>,
    pub message: String,
}

/// source: kinds — verbatim.
pub const KIND_INVALID: &str = "invalid";
pub const KIND_UNSUPPORTED: &str = "unsupported";
pub const KIND_CONFLICT: &str = "conflict";

/// source: default source "configuration" — verbatim.
pub const DEFAULT_SOURCE: &str = "configuration";

/// source: V2 permissions error — verbatim message.
pub const V2_PERMISSIONS_MESSAGE: &str =
    "V2 permissions are not supported by OpenCode V1. Use V1 \"permission\" rules or run opencode2.";

/// source: unsupported() message — verbatim.
pub const UNSUPPORTED_MESSAGE: &str = "Omitted native setting that cannot be represented in V1";
/// source: conflict() message — verbatim.
pub const CONFLICT_MESSAGE: &str = "Retained legacy value over native value";
/// source: decodeValue() invalid message — verbatim.
pub const INVALID_MESSAGE: &str = "Native setting could not be lowered because it is malformed";

/// source: top-level unsupported keys — verbatim.
pub const UNSUPPORTED_TOP: &[&str] = &["plugins", "providers", "websearch", "warming"];

/// source: lowerSelection() — string "model#variant" split; struct
/// providerID/model (+variant). Verbatim.
pub fn lower_selection(model: &str, variant: Option<&str>) -> (String, Option<String>) {
    match model.find('#') {
        None => (model.to_string(), variant.map(|s| s.to_string())),
        Some(i) => (model[..i].to_string(), Some(model[i + 1..].to_string())),
    }
}

/// source: lowerSelection struct branch — `${providerID}/${model}`. Verbatim.
pub fn lower_selection_struct(
    provider_id: &str,
    model: &str,
    variant: Option<&str>,
) -> (String, Option<String>) {
    (
        format!("{}/{}", provider_id, model),
        variant.map(|s| s.to_string()),
    )
}

/// source: model.variant unsupported path — verbatim.
pub const MODEL_VARIANT_PATH: &[&str] = &["model", "variant"];

/// source: lowerTimeout() — startup defined → undefined; catalog+execution
/// both defined and equal → catalog; else undefined. Verbatim.
pub fn lower_timeout(
    startup: Option<i64>,
    catalog: Option<i64>,
    execution: Option<i64>,
) -> Option<i64> {
    if startup.is_some() {
        return None;
    }
    match (catalog, execution) {
        (Some(c), Some(e)) if c == e => Some(c),
        _ => None,
    }
}

/// source: lowerServer() — enabled = disabled !== true; delete disabled/
/// codemode/timeout; oauth snake→camel renames. Verbatim.
pub const OAUTH_RENAMES: &[(&str, &str)] = &[
    ("client_id", "clientId"),
    ("client_secret", "clientSecret"),
    ("scope", "scope"),
    ("callback_port", "callbackPort"),
    ("redirect_uri", "redirectUri"),
];

/// source: lowerAgent() kept keys — verbatim.
pub const AGENT_KEPT: &[&str] = &["description", "mode", "hidden", "color", "steps"];
/// source: lowerAgent() renames — system→prompt, disabled→disable, model→spread,
/// request.body→options. Verbatim.
pub const AGENT_SYSTEM_TO: &str = "prompt";
pub const AGENT_DISABLED_TO: &str = "disable";
pub const AGENT_BODY_TO: &str = "options";

/// source: normalizeSkills() — http(s) → urls else paths. Verbatim.
pub fn split_skill_paths(skills: &[String]) -> (Vec<String>, Vec<String>) {
    let mut paths = Vec::new();
    let mut urls = Vec::new();
    for s in skills {
        let lower = s.to_lowercase();
        if lower.starts_with("http://") || lower.starts_with("https://") {
            urls.push(s.clone());
        } else {
            paths.push(s.clone());
        }
    }
    (paths, urls)
}

/// source: compaction renames — keep.tokens→preserve_recent_tokens,
/// buffer→reserved. Verbatim.
pub const COMPACTION_TOKENS_TO: &str = "preserve_recent_tokens";
pub const COMPACTION_BUFFER_TO: &str = "reserved";

/// source: experimental subagent_depth passthrough name — verbatim.
pub const SUBAGENT_DEPTH: &str = "subagent_depth";
/// source: experimental mcp_timeout name — verbatim.
pub const MCP_TIMEOUT: &str = "mcp_timeout";

/// source: selection patterns — verbatim.
pub const SELECTION_STRING_PATTERN: &str = "^[^/#]+/[^#]+(?:#[^#]+)?$";
pub const PROVIDER_PATTERN: &str = "^[^/#]+$";
pub const MODEL_PATTERN: &str = "^[^#]+$";
/// source: color pattern — verbatim.
pub const COLOR_PATTERN: &str = "^#[0-9a-fA-F]{6}$";
/// source: callback port range 1..=65535 — verbatim.
pub const CALLBACK_PORT_MIN: i64 = 1;
pub const CALLBACK_PORT_MAX: i64 = 65535;
