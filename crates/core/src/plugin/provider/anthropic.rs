//! Rust port of `packages/core/src/plugin/provider/anthropic.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

use std::collections::HashMap;

pub const ID: &str = "anthropic";
pub const PACKAGE: &str = "@ai-sdk/anthropic";
pub const ANTHROPIC_BETA: &str =
    "interleaved-thinking-2025-05-14,fine-grained-tool-streaming-2025-05-14";

pub fn headers_with_beta(mut headers: HashMap<String, String>) -> HashMap<String, String> {
    headers.insert("anthropic-beta".to_string(), ANTHROPIC_BETA.to_string());
    headers
}

pub fn build_request(
    base_url: &str,
    api_key: &str,
    model_id: &str,
    messages: serde_json::Value,
    stream: bool,
) -> Result<reqwest::Request, String> {
    let url = format!("{}/messages", base_url.trim_end_matches('/'));
    let body = serde_json::json!({
        "model": model_id,
        "messages": messages,
        "stream": stream,
        "max_tokens": 4096
    });
    let client = reqwest::Client::new();
    client
        .post(url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("anthropic-beta", ANTHROPIC_BETA)
        .header("Content-Type", "application/json")
        .json(&body)
        .build()
        .map_err(|e| e.to_string())
}

pub fn parse_sse_chunk(line: &str) -> Option<serde_json::Value> {
    let data = line.strip_prefix("data: ")?;
    if data == "[DONE]" {
        return None;
    }
    serde_json::from_str(data).ok()
}
