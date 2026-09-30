//! Rust port of `packages/core/src/plugin/provider/openrouter.ts`.

use std::collections::HashMap;

pub const ID: &str = "openrouter";
pub const PACKAGE: &str = "@openrouter/ai-sdk-provider";
pub const BASE_URL: &str = "https://openrouter.ai/api/v1";

pub fn headers(mut h: HashMap<String, String>) -> HashMap<String, String> {
    h.insert(
        "HTTP-Referer".to_string(),
        "https://opencode.ai/".to_string(),
    );
    h.insert("X-Title".to_string(), "opencode".to_string());
    h
}

pub fn is_disabled_model(model_id: &str) -> bool {
    matches!(model_id, "gpt-5-chat-latest" | "openai/gpt-5-chat")
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
        .header("HTTP-Referer", "https://opencode.ai/")
        .header("X-Title", "opencode")
        .json(&body)
        .build()
        .map_err(|e| e.to_string())
}
