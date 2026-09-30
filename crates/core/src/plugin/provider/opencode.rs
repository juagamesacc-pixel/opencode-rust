//! Rust port of `packages/core/src/plugin/provider/opencode.ts`.

pub const ID: &str = "opencode";
pub const DEFAULT_SERVER: &str = "https://opencode.ai/console";
pub const CLIENT_ID: &str = "opencode-cli";

pub fn without_credentials(
    body: &serde_json::Map<String, serde_json::Value>,
) -> serde_json::Map<String, serde_json::Value> {
    let mut out = serde_json::Map::new();
    for (k, v) in body {
        if k == "apiKey" || k == "headers" {
            continue;
        }
        out.insert(k.clone(), v.clone());
    }
    out
}

pub fn fetch_url(server: &str) -> String {
    format!("{}/api/config", server.trim_end_matches('/'))
}

pub fn build_fetch_request(
    server: &str,
    token: &str,
    org_id: Option<&str>,
) -> Result<reqwest::Request, String> {
    let url = fetch_url(server);
    let mut req = reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {token}"));
    if let Some(id) = org_id {
        req = req.header("x-org-id", id);
    }
    req.build().map_err(|e| e.to_string())
}

pub fn device_verification_url(complete: &str, server: &str) -> Result<String, String> {
    let raw = if complete.starts_with("http://") || complete.starts_with("https://") {
        complete.to_string()
    } else {
        format!(
            "{}/{}",
            server.trim_end_matches('/'),
            complete.trim_start_matches('/')
        )
    };
    if !(raw.starts_with("http://") || raw.starts_with("https://")) {
        return Err("Invalid device verification URL: expected HTTP(S)".to_string());
    }
    Ok(raw)
}
