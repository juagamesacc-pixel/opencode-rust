//! Rust port of `packages/core/src/provider.ts` + aggregated provider request builders.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Provider catalog entry — mirrors Provider.Info shape from schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub api: ProviderApi,
    #[serde(default)]
    pub request: ProviderRequestConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderApi {
    #[serde(rename = "type")]
    pub api_type: String,
    pub package: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderRequestConfig {
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub body: HashMap<String, serde_json::Value>,
}

pub fn provider_id_is_valid(id: &str) -> bool {
    !id.is_empty()
}

/// Known provider IDs — mirrors Provider.ID
pub mod id {
    pub const ANTHROPIC: &str = "anthropic";
    pub const OPENAI: &str = "openai";
    pub const GOOGLE: &str = "google";
    pub const AZURE: &str = "azure";
    pub const AMAZON_BEDROCK: &str = "amazon-bedrock";
    pub const GROQ: &str = "groq";
    pub const MISTRAL: &str = "mistral";
    pub const COHERE: &str = "cohere";
    pub const PERPLEXITY: &str = "perplexity";
    pub const DEEPINFRA: &str = "deepinfra";
    pub const CEREBRAS: &str = "cerebras";
    pub const TOGETHERAI: &str = "togetherai";
    pub const XAI: &str = "xai";
    pub const OPENROUTER: &str = "openrouter";
    pub const VERCEL: &str = "vercel";
    pub const GITHUB_COPILOT: &str = "github-copilot";
    pub const GOOGLE_VERTEX: &str = "google-vertex";
    pub const CLOUDFLARE_WORKERS_AI: &str = "cloudflare-workers-ai";
}

/// Default base URLs for OpenAI-compatible providers
pub fn default_base_url(provider_id: &str) -> Option<&'static str> {
    match provider_id {
        "openai" => Some("https://api.openai.com/v1"),
        "anthropic" => Some("https://api.anthropic.com/v1"),
        "groq" => Some("https://api.groq.com/openai/v1"),
        "mistral" => Some("https://api.mistral.ai/v1"),
        "xai" => Some("https://api.x.ai/v1"),
        "openrouter" => Some("https://openrouter.ai/api/v1"),
        "deepinfra" => Some("https://api.deepinfra.com/v1/openai"),
        "cerebras" => Some("https://api.cerebras.ai/v1"),
        "togetherai" => Some("https://api.together.xyz/v1"),
        "perplexity" => Some("https://api.perplexity.ai"),
        "cohere" => Some("https://api.cohere.ai/compatibility/v1"),
        "github-copilot" => Some("https://api.githubcopilot.com"),
        _ => None,
    }
}

/// Build auth headers verbatim per provider: Authorization Bearer for OpenAI-family
pub fn auth_headers(
    provider_id: &str,
    api_key: Option<&str>,
    extra: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut h = extra.clone();
    if let Some(k) = api_key {
        match provider_id {
            "anthropic" => {
                h.insert("x-api-key".to_string(), k.to_string());
                h.insert("anthropic-version".to_string(), "2023-06-01".to_string());
            }
            _ => {
                h.insert("Authorization".to_string(), format!("Bearer {k}"));
            }
        }
    }
    h
}

/// Build chat completions request — mirrors `postJsonToApi` with JSON body and SSE headers.
/// Hermetic builder for tests.
#[allow(clippy::too_many_arguments)] // 1:1 source signature (postJsonToApi params), do not split
pub fn build_chat_request(
    provider_id: &str,
    model_id: &str,
    base_url: &str,
    api_key: Option<&str>,
    messages: serde_json::Value,
    stream: bool,
    extra_headers: HashMap<String, String>,
    extra_body: HashMap<String, serde_json::Value>,
) -> Result<reqwest::Request, String> {
    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
    let mut body = serde_json::json!({
        "model": model_id,
        "messages": messages,
        "stream": stream
    });
    if let serde_json::Value::Object(map) = &mut body {
        for (k, v) in extra_body {
            map.insert(k, v);
        }
    }
    let headers = auth_headers(provider_id, api_key, &extra_headers);
    let client = reqwest::Client::new();
    let mut req = client
        .post(url)
        .json(&body)
        .header("Content-Type", "application/json");
    if stream {
        req = req.header("Accept", "text/event-stream");
    } else {
        req = req.header("Accept", "application/json");
    }
    for (k, v) in headers {
        req = req.header(k, v);
    }
    req.build().map_err(|e| e.to_string())
}

/// SSE line parsing — mirrors `createEventSourceResponseHandler` + `createJsonResponseHandler`
/// Parses `data: {...}` lines, handles `[DONE]`, returns JSON values.
pub fn parse_sse_body(body: &str) -> Vec<serde_json::Value> {
    let mut out = Vec::new();
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "data: [DONE]" {
            break;
        }
        let data = if let Some(rest) = trimmed.strip_prefix("data: ") {
            rest
        } else {
            continue;
        };
        if data == "[DONE]" {
            break;
        }
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(data) {
            // skip error envelope with `error` key? keep for caller to detect
            out.push(v);
        }
    }
    out
}

/// Extract text deltas from OpenAI chat stream chunks
pub fn extract_chat_text_deltas(chunks: &[serde_json::Value]) -> String {
    let mut s = String::new();
    for v in chunks {
        if let Some(choice) = v
            .get("choices")
            .and_then(|c| c.as_array())
            .and_then(|a| a.first())
        {
            if let Some(delta) = choice
                .get("delta")
                .and_then(|d| d.get("content"))
                .and_then(|c| c.as_str())
            {
                s.push_str(delta);
            }
        }
    }
    s
}

/// Parse non-stream JSON response body into content + finish reason (hermetic)
pub fn parse_chat_response(body: &str) -> Option<(String, Option<String>)> {
    let v: serde_json::Value = serde_json::from_str(body).ok()?;
    let choice = v.get("choices")?.as_array()?.first()?;
    let text = choice.get("message")?.get("content")?.as_str()?.to_string();
    let finish = choice
        .get("finish_reason")
        .and_then(|f| f.as_str())
        .map(|s| s.to_string());
    Some((text, finish))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sse_parses_data_lines() {
        let body = "data: {\"choices\":[{\"delta\":{\"content\":\"hello\"}}]}\n\ndata: {\"choices\":[{\"delta\":{\"content\":\" world\"}}]}\n\ndata: [DONE]\n";
        let chunks = parse_sse_body(body);
        assert_eq!(chunks.len(), 2);
        assert_eq!(extract_chat_text_deltas(&chunks), "hello world");
    }
    #[test]
    fn chat_request_headers() {
        let req = build_chat_request(
            "openai",
            "gpt-4",
            "https://api.openai.com/v1",
            Some("sk-123"),
            serde_json::json!([{"role":"user","content":"hi"}]),
            true,
            Default::default(),
            Default::default(),
        )
        .unwrap();
        assert_eq!(
            req.url().as_str(),
            "https://api.openai.com/v1/chat/completions"
        );
        assert_eq!(req.headers().get("Authorization").unwrap(), "Bearer sk-123");
    }
}
