// source: src/plugin/github-copilot/copilot.ts — exports: CopilotAuthPlugin
// (+ consts per source; PROVISIONAL: device-flow/polling as descriptors).
// CLIENT_ID, API_VERSION, UTILITY_MODELS, polling margin, npm verbatim.

/// source: CLIENT_ID — verbatim.
pub const CLIENT_ID: &str = "Ov23li8tweQw6odWQebz";
/// source: API_VERSION — verbatim.
pub const API_VERSION: &str = "2026-06-01";
/// source: UTILITY_MODELS — verbatim order.
pub const UTILITY_MODELS: &[&str] = &["gpt-5.4-nano", "gpt-4.1", "gpt-4o", "gpt-4o-mini"];
/// source: OAUTH_POLLING_SAFETY_MARGIN_MS = 3000 — verbatim.
pub const OAUTH_POLLING_SAFETY_MARGIN_MS: u64 = 3000;
/// source: base URLs — verbatim.
pub const COPILOT_API: &str = "https://api.githubcopilot.com";
/// source: fix() npm "@ai-sdk/github-copilot" — verbatim.
pub const COPILOT_NPM: &str = "@ai-sdk/github-copilot";

/// source: normalizeDomain() — strip scheme + trailing slash. Verbatim.
pub fn normalize_domain(url: &str) -> String {
    let s = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);
    s.strip_suffix('/').unwrap_or(s).to_string()
}
