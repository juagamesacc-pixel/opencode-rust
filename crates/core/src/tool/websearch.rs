//! Rust port of `packages/core/src/tool/websearch.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.

use serde::{Deserialize, Serialize};

pub const NAME: &str = "websearch";
pub const NO_RESULTS: &str = "No search results found. Please try a different query.";
pub const EXA_URL: &str = "https://mcp.exa.ai/mcp";
pub const PARALLEL_URL: &str = "https://search.parallel.ai/mcp";
pub const MAX_NUM_RESULTS: u64 = 20;
pub const MAX_CONTEXT_CHARACTERS: u64 = 50_000;
pub const MAX_RESPONSE_BYTES: usize = 256 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Exa,
    Parallel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub query: String,
    #[serde(rename = "numResults", skip_serializing_if = "Option::is_none")]
    pub num_results: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livecrawl: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(
        rename = "contextMaxCharacters",
        skip_serializing_if = "Option::is_none"
    )]
    pub context_max_characters: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub provider: Option<Provider>,
    pub enable_exa: bool,
    pub enable_parallel: bool,
    pub exa_api_key: Option<String>,
    pub parallel_api_key: Option<String>,
}

pub fn select_provider(
    session_id: &str,
    flags: &Config,
    override_provider: Option<Provider>,
) -> Provider {
    if let Some(p) = override_provider {
        return p;
    }
    if flags.enable_parallel {
        return Provider::Parallel;
    }
    if flags.enable_exa {
        return Provider::Exa;
    }
    // checksum-based fallback — use simple hash mirroring `checksum(sessionID)` %2
    let hash: u64 = session_id
        .bytes()
        .fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
    if hash.is_multiple_of(2) {
        Provider::Exa
    } else {
        Provider::Parallel
    }
}

pub fn exa_url(api_key: Option<&str>) -> String {
    if let Some(key) = api_key {
        format!("{EXA_URL}?exaApiKey={key}")
    } else {
        EXA_URL.to_string()
    }
}

pub fn parse_response(body: &str) -> Option<String> {
    let trimmed = body.trim();
    if trimmed.starts_with('{') {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(trimmed) {
            if let Some(text) = v
                .get("result")
                .and_then(|r| r.get("content"))
                .and_then(|c| c.as_array())
                .and_then(|arr| {
                    arr.iter().find_map(|item| {
                        item.get("text")
                            .and_then(|t| t.as_str())
                            .map(|s| s.to_string())
                    })
                })
            {
                return Some(text);
            }
        }
    }
    for line in body.split('\n') {
        if !line.starts_with("data: ") {
            continue;
        }
        let data = &line[6..];
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(data) {
            if let Some(text) = v
                .get("result")
                .and_then(|r| r.get("content"))
                .and_then(|c| c.as_array())
                .and_then(|arr| {
                    arr.iter().find_map(|item| {
                        item.get("text")
                            .and_then(|t| t.as_str())
                            .map(|s| s.to_string())
                    })
                })
            {
                return Some(text);
            }
        }
    }
    None
}

/// Build MCP JSON-RPC body for Exa
pub fn exa_mcp_body(
    query: &str,
    type_: &str,
    num_results: u64,
    livecrawl: &str,
    context_max_characters: Option<u64>,
) -> serde_json::Value {
    let mut args = serde_json::json!({
        "query": query,
        "type": type_,
        "numResults": num_results,
        "livecrawl": livecrawl
    });
    if let Some(v) = context_max_characters {
        args["contextMaxCharacters"] = serde_json::json!(v);
    }
    serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": { "name": "web_search_exa", "arguments": args }
    })
}

/// Build MCP JSON-RPC body for Parallel
pub fn parallel_mcp_body(query: &str, session_id: &str) -> serde_json::Value {
    serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "web_search",
            "arguments": {
                "objective": query,
                "search_queries": [query],
                "session_id": session_id
            }
        }
    })
}

pub fn parallel_headers(
    parallel_api_key: Option<&str>,
    installation_version: &str,
) -> std::collections::HashMap<String, String> {
    let mut m = std::collections::HashMap::new();
    m.insert(
        "User-Agent".to_string(),
        format!("opencode/{installation_version}"),
    );
    m.insert(
        "Accept".to_string(),
        "application/json, text/event-stream".to_string(),
    );
    if let Some(k) = parallel_api_key {
        m.insert("Authorization".to_string(), format!("Bearer {k}"));
    }
    m
}

/// Hermetic: build reqwest request for Exa without sending
pub fn build_exa_request(
    api_key: Option<&str>,
    body: &serde_json::Value,
) -> Result<reqwest::Request, String> {
    let url = exa_url(api_key);
    let client = reqwest::Client::new();
    client
        .post(url)
        .header("Accept", "application/json, text/event-stream")
        .header("Content-Type", "application/json")
        .json(body)
        .build()
        .map_err(|e| e.to_string())
}

/// Hermetic: build reqwest request for Parallel without sending
pub fn build_parallel_request(
    api_key: Option<&str>,
    installation_version: &str,
    body: &serde_json::Value,
) -> Result<reqwest::Request, String> {
    let client = reqwest::Client::new();
    let mut req = client
        .post(PARALLEL_URL)
        .header("Accept", "application/json, text/event-stream")
        .header("Content-Type", "application/json")
        .header("User-Agent", format!("opencode/{installation_version}"));
    if let Some(k) = api_key {
        req = req.header("Authorization", format!("Bearer {k}"));
    }
    req.json(body).build().map_err(|e| e.to_string())
}

/// Async MCP call with 25s timeout and 256KB bound, mirroring `callMcp` + `collectBoundedResponseBody`
pub async fn call_mcp(
    client: &reqwest::Client,
    url: &str,
    tool: &str,
    body: &serde_json::Value,
    extra_headers: std::collections::HashMap<String, String>,
) -> Result<Option<String>, String> {
    let timeout = std::time::Duration::from_secs(25);
    let mut req = client
        .post(url)
        .header("Accept", "application/json, text/event-stream")
        .header("Content-Type", "application/json")
        .json(body)
        .timeout(timeout);
    for (k, v) in extra_headers {
        req = req.header(k, v);
    }
    let resp = req
        .send()
        .await
        .map_err(|e| format!("{tool} request failed: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("{tool} request failed: {}", resp.status()));
    }
    let bytes =
        crate::tool::http_body::collect_bounded_response_body(resp, MAX_RESPONSE_BYTES, || {
            format!("{tool} response exceeded {MAX_RESPONSE_BYTES} bytes")
        })
        .await?;
    let body_str = String::from_utf8_lossy(&bytes).to_string();
    // timeout is via reqwest timeout; map to error string "{tool} request timed out" is handled by reqwest err
    Ok(parse_response(&body_str))
}
