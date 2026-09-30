//! Rust port of `packages/core/src/plugin/provider/cloudflare-workers-ai.ts`.

pub const ID: &str = "cloudflare-workers-ai";
pub const PACKAGE: &str = "@ai-sdk/openai-compatible";

pub fn resolve_account_id(options_account: Option<&str>) -> Option<String> {
    std::env::var("CLOUDFLARE_ACCOUNT_ID")
        .ok()
        .or_else(|| options_account.map(|s| s.to_string()))
}

pub fn workers_endpoint(account_id: &str) -> String {
    format!("https://api.cloudflare.com/client/v4/accounts/{account_id}/ai/v1")
}

pub fn expand_account_id(base_url: Option<&str>) -> Option<String> {
    base_url.map(|s| {
        s.replace(
            "${CLOUDFLARE_ACCOUNT_ID}",
            &std::env::var("CLOUDFLARE_ACCOUNT_ID")
                .unwrap_or("${CLOUDFLARE_ACCOUNT_ID}".to_string()),
        )
    })
}

pub fn build_request(
    account_id: &str,
    api_key: Option<&str>,
    model_id: &str,
    messages: serde_json::Value,
) -> Result<reqwest::Request, String> {
    let base = workers_endpoint(account_id);
    let url = format!("{}/chat/completions", base.trim_end_matches('/'));
    let body = serde_json::json!({"model": model_id, "messages": messages, "stream": true});
    let mut req = reqwest::Client::new()
        .post(url)
        .json(&body)
        .header("Content-Type", "application/json");
    if let Some(k) = api_key {
        req = req.header("Authorization", format!("Bearer {k}"));
    }
    req.header("User-Agent", "opencode/1.18.30 cloudflare-workers-ai")
        .build()
        .map_err(|e| e.to_string())
}
