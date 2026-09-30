//! Rust port of `packages/core/src/plugin/provider/cerebras.ts`.

use std::collections::HashMap;

pub const ID: &str = "cerebras";
pub const PACKAGE: &str = "@ai-sdk/cerebras";
pub const HEADER: &str = "X-Cerebras-3rd-Party-Integration";
pub const HEADER_VALUE: &str = "opencode";

pub fn headers(mut h: HashMap<String, String>) -> HashMap<String, String> {
    h.insert(HEADER.to_string(), HEADER_VALUE.to_string());
    h
}

pub fn build_request(
    base_url: &str,
    api_key: &str,
    model_id: &str,
    messages: serde_json::Value,
    stream: bool,
) -> Result<reqwest::Request, String> {
    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
    let body = serde_json::json!({"model": model_id, "messages": messages, "stream": stream});
    reqwest::Client::new()
        .post(url)
        .header("Authorization", format!("Bearer {api_key}"))
        .header(HEADER, HEADER_VALUE)
        .json(&body)
        .build()
        .map_err(|e| e.to_string())
}
