//! Rust port of `packages/core/src/plugin/provider/openai-compatible.ts`.

pub const ID: &str = "openai-compatible";
pub const PACKAGE: &str = "@ai-sdk/openai-compatible";

pub fn with_include_usage(
    mut options: std::collections::HashMap<String, serde_json::Value>,
) -> std::collections::HashMap<String, serde_json::Value> {
    options
        .entry("includeUsage".to_string())
        .or_insert(serde_json::Value::Bool(true));
    options
}

pub fn build_request(
    base_url: &str,
    api_key: Option<&str>,
    model_id: &str,
    messages: serde_json::Value,
    stream: bool,
) -> Result<reqwest::Request, String> {
    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
    let body = serde_json::json!({"model": model_id, "messages": messages, "stream": stream});
    let mut req = reqwest::Client::new()
        .post(url)
        .json(&body)
        .header("Content-Type", "application/json");
    if let Some(k) = api_key {
        req = req.header("Authorization", format!("Bearer {k}"));
    }
    req.build().map_err(|e| e.to_string())
}
