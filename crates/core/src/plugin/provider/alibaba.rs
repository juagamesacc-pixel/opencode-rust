//! Rust port of `packages/core/src/plugin/provider/alibaba.ts`.

pub const ID: &str = "alibaba";
pub const PACKAGE: &str = "@ai-sdk/alibaba";
pub const BASE_URL: &str = "https://dashscope.aliyuncs.com/compatible-mode/v1";

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
        .json(&body)
        .build()
        .map_err(|e| e.to_string())
}
