//! Rust port of `packages/core/src/github-copilot/copilot-provider.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

use std::collections::HashMap;

pub const VERSION: &str = "0.1.0";
pub const DEFAULT_BASE_URL: &str = "https://api.openai.com/v1";

pub fn without_trailing_slash(url: &str) -> &str {
    url.trim_end_matches('/')
}

pub fn with_user_agent_suffix(mut headers: HashMap<String, String>) -> HashMap<String, String> {
    let suffix = format!("ai-sdk/openai-compatible/{VERSION}");
    let entry = headers
        .entry("User-Agent".to_string())
        .or_insert_with(|| "opencode".to_string());
    if !entry.contains(&suffix) {
        *entry = format!("{entry} {suffix}");
    }
    headers
}

#[derive(Debug, Clone, Default)]
pub struct OpenaiCompatibleProviderSettings {
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub name: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

pub fn build_headers(settings: &OpenaiCompatibleProviderSettings) -> HashMap<String, String> {
    let mut h = HashMap::new();
    if let Some(k) = &settings.api_key {
        h.insert("Authorization".to_string(), format!("Bearer {k}"));
    }
    if let Some(extra) = &settings.headers {
        for (k, v) in extra {
            h.insert(k.clone(), v.clone());
        }
    }
    with_user_agent_suffix(h)
}

pub fn base_url(settings: &OpenaiCompatibleProviderSettings) -> String {
    without_trailing_slash(settings.base_url.as_deref().unwrap_or(DEFAULT_BASE_URL)).to_string()
}

pub fn chat_url(base: &str) -> String {
    format!("{}/chat/completions", base.trim_end_matches('/'))
}

pub fn responses_url(base: &str) -> String {
    format!("{}/responses", base.trim_end_matches('/'))
}

pub fn build_chat_request(
    settings: &OpenaiCompatibleProviderSettings,
    _model_id: &str,
    body: serde_json::Value,
    extra_headers: HashMap<String, String>,
) -> Result<reqwest::Request, String> {
    let base = base_url(settings);
    let url = chat_url(&base);
    let mut headers = build_headers(settings);
    for (k, v) in extra_headers {
        headers.insert(k, v);
    }
    let client = reqwest::Client::new();
    let mut req = client
        .post(url)
        .json(&body)
        .header("Content-Type", "application/json");
    for (k, v) in headers {
        req = req.header(k, v);
    }
    req.build().map_err(|e| e.to_string())
}

pub fn build_responses_request(
    settings: &OpenaiCompatibleProviderSettings,
    _model_id: &str,
    body: serde_json::Value,
    extra_headers: HashMap<String, String>,
) -> Result<reqwest::Request, String> {
    let base = base_url(settings);
    let url = responses_url(&base);
    let mut headers = build_headers(settings);
    for (k, v) in extra_headers {
        headers.insert(k, v);
    }
    let client = reqwest::Client::new();
    let mut req = client
        .post(url)
        .json(&body)
        .header("Content-Type", "application/json");
    for (k, v) in headers {
        req = req.header(k, v);
    }
    req.build().map_err(|e| e.to_string())
}

pub fn parse_chat_sse(body: &str) -> Vec<serde_json::Value> {
    body.lines()
        .filter_map(|l| l.strip_prefix("data: "))
        .filter(|d| *d != "[DONE]")
        .filter_map(|d| serde_json::from_str(d).ok())
        .collect()
}
