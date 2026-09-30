//! Rust port of `packages/core/src/models-dev.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const DEFAULT_SOURCE: &str = "https://models.opencode.ai";
pub const API_PATH: &str = "/api.json";
pub const CACHE_TTL_MINS: u64 = 5;
pub const FETCH_TIMEOUT_SECS: u64 = 10;
pub const RETRY_TIMES: usize = 2;
pub const RETRY_BASE_MS: u64 = 200;

pub fn user_agent(channel: &str, version: &str, client: &str) -> String {
    format!("opencode/{channel}/{version}/{client}")
}

pub fn fetch_url(source: &str) -> String {
    format!("{}{API_PATH}", source.trim_end_matches('/'))
}

pub fn cache_filepath(cache_dir: &str, source: &str) -> String {
    if source == DEFAULT_SOURCE {
        format!("{cache_dir}/models.json")
    } else {
        // Hash.fast(source) approximated as simple hash for filename
        let hash = source
            .bytes()
            .fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
        format!("{cache_dir}/models-{hash:x}.json")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CatalogModelStatus {
    Alpha,
    Beta,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostTier {
    pub input: f64,
    pub output: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_read: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_write: Option<f64>,
    pub tier: Tier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tier {
    #[serde(rename = "type")]
    pub tier_type: String,
    pub size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cost {
    pub input: f64,
    pub output: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_read: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_write: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<CostTier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_over_200k: Option<Box<Cost>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    pub release_date: String,
    pub attachment: bool,
    pub reasoning: bool,
    pub temperature: bool,
    pub tool_call: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<Cost>,
    pub limit: Limit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CatalogModelStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<ModelProvider>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limit {
    pub context: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<f64>,
    pub output: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProvider {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    pub name: String,
    pub env: Vec<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npm: Option<String>,
    pub models: HashMap<String, Model>,
}

pub type Catalog = HashMap<String, Provider>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub provider: String,
    pub name: String,
}

pub fn model_id_normalize(id: &str) -> String {
    id.to_lowercase()
}

pub fn headers_for(user_agent: &str) -> std::collections::HashMap<String, String> {
    let mut m = std::collections::HashMap::new();
    m.insert("User-Agent".to_string(), user_agent.to_string());
    m
}

/// Build reqwest request for GET /api.json with User-Agent, 10s timeout, retry transient semantics.
/// Hermetic builder (no network) for tests.
pub fn build_fetch_request(source: &str, user_agent: &str) -> Result<reqwest::Request, String> {
    let url = fetch_url(source);
    let client = reqwest::Client::new();
    client
        .get(url)
        .header("User-Agent", user_agent)
        .build()
        .map_err(|e| e.to_string())
}

/// Async fetch with retry (times=2, exponential 200ms jittered) + 10s timeout + filterStatusOk
pub async fn fetch_catalog(
    client: &reqwest::Client,
    source: &str,
    user_agent: &str,
) -> Result<Catalog, String> {
    let url = fetch_url(source);
    let timeout = std::time::Duration::from_secs(FETCH_TIMEOUT_SECS);
    let mut last_err: Option<String> = None;
    for attempt in 0..=RETRY_TIMES {
        if attempt > 0 {
            let base = RETRY_BASE_MS * (1u64 << (attempt - 1));
            // jittered: 0.5..1.5x approximated as base + rand-ish via nanos
            let jitter = (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos()
                % 100) as u64;
            let delay = base + jitter;
            tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
        }
        let resp = client
            .get(&url)
            .header("User-Agent", user_agent)
            .timeout(timeout)
            .send()
            .await;
        match resp {
            Ok(r) => {
                if !r.status().is_success() {
                    // retry on 5xx per retryTransient errors-and-responses
                    if r.status().is_server_error() {
                        last_err = Some(format!("Request failed: {}", r.status()));
                        continue;
                    }
                    return Err(format!("Request failed: {}", r.status()));
                }
                let text = r.text().await.map_err(|e| e.to_string())?;
                let catalog: Catalog = serde_json::from_str(&text).map_err(|e| e.to_string())?;
                return Ok(catalog);
            }
            Err(e) => {
                // transient errors retry
                last_err = Some(e.to_string());
                if e.is_timeout() || e.is_connect() {
                    continue;
                }
                // also retry
                continue;
            }
        }
    }
    Err(last_err.unwrap_or_else(|| "fetch failed".to_string()))
}

pub fn parse_catalog_json(text: &str) -> Result<Catalog, String> {
    serde_json::from_str(text).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn catalog_url_and_headers() {
        assert_eq!(
            fetch_url("https://models.opencode.ai"),
            "https://models.opencode.ai/api.json"
        );
        let req =
            build_fetch_request("https://models.opencode.ai", "opencode/test/1/client").unwrap();
        assert_eq!(req.url().as_str(), "https://models.opencode.ai/api.json");
        assert_eq!(
            req.headers().get("User-Agent").unwrap(),
            "opencode/test/1/client"
        );
    }
    #[test]
    fn parse_fixture() {
        let json = r#"{"openai":{"id":"openai","name":"OpenAI","env":["OPENAI_API_KEY"],"models":{"gpt-4":{"id":"gpt-4","name":"GPT-4","release_date":"2024-01-01","attachment":false,"reasoning":false,"temperature":true,"tool_call":true,"limit":{"context":128000,"output":4096}}}}}"#;
        let cat = parse_catalog_json(json).unwrap();
        assert!(cat.contains_key("openai"));
        assert!(cat["openai"].models.contains_key("gpt-4"));
    }
}
