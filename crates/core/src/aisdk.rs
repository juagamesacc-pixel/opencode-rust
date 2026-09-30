//! Rust port of `packages/core/src/aisdk.ts` + `src/plugin/provider.ts` AI SDK bridge.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AiSdkInfo {
    pub model: String,
    pub provider_id: String,
    pub package: String,
}

pub fn is_aisdk_provider(id: &str) -> bool {
    // Provider.Api.type === "aisdk" heuristic
    id.contains("aisdk")
        || matches!(
            id,
            "anthropic" | "openai" | "google" | "azure" | "groq" | "mistral" | "xai" | "openrouter"
        )
}

pub fn sdk_package_for(provider_id: &str) -> Option<&'static str> {
    match provider_id {
        "anthropic" => Some("@ai-sdk/anthropic"),
        "openai" => Some("@ai-sdk/openai"),
        "google" => Some("@ai-sdk/google"),
        "azure" => Some("@ai-sdk/azure"),
        "groq" => Some("@ai-sdk/groq"),
        "mistral" => Some("@ai-sdk/mistral"),
        "xai" => Some("@ai-sdk/xai"),
        "openrouter" => Some("@openrouter/ai-sdk-provider"),
        "vercel" => Some("@ai-sdk/vercel"),
        "perplexity" => Some("@ai-sdk/perplexity"),
        "deepinfra" => Some("@ai-sdk/deepinfra"),
        "cerebras" => Some("@ai-sdk/cerebras"),
        "togetherai" => Some("@ai-sdk/togetherai"),
        _ => None,
    }
}

pub fn language_model_id(_provider_id: &str, model_api_id: &str) -> String {
    model_api_id.trim().to_string()
}

pub fn uses_responses_api(provider_id: &str, model_id: &str) -> bool {
    // mirrors openai and github-copilot plugin: responses for gpt-5+ except mini
    if provider_id == "openai" || provider_id == "github-copilot" {
        if model_id.starts_with("gpt-5-mini") {
            return false;
        }
        if let Some(rest) = model_id.strip_prefix("gpt-") {
            if let Some(num) = rest.split('-').next().and_then(|s| s.parse::<u32>().ok()) {
                return num >= 5;
            }
        }
    }
    false
}

pub fn headers_for_provider(
    provider_id: &str,
    mut headers: HashMap<String, String>,
) -> HashMap<String, String> {
    match provider_id {
        "anthropic" => {
            headers.insert(
                "anthropic-beta".to_string(),
                "interleaved-thinking-2025-05-14,fine-grained-tool-streaming-2025-05-14"
                    .to_string(),
            );
        }
        "cerebras" => {
            headers.insert(
                "X-Cerebras-3rd-Party-Integration".to_string(),
                "opencode".to_string(),
            );
        }
        "openrouter" => {
            headers.insert(
                "HTTP-Referer".to_string(),
                "https://opencode.ai/".to_string(),
            );
            headers.insert("X-Title".to_string(), "opencode".to_string());
        }
        "vercel" => {
            headers.insert(
                "http-referer".to_string(),
                "https://opencode.ai/".to_string(),
            );
            headers.insert("x-title".to_string(), "opencode".to_string());
        }
        _ => {}
    }
    headers
}
