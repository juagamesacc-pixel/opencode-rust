//! Rust port of `packages/core/src/plugin/provider/github-copilot.ts`.

pub const ID: &str = "github-copilot";
pub const PACKAGE: &str = "@ai-sdk/github-copilot";

pub fn should_hide_gpt5_chat(model_id: &str) -> bool {
    model_id == "gpt-5-chat-latest"
}

pub fn select_endpoint(
    sdk_has_responses: bool,
    sdk_has_chat: bool,
    endpoint: Option<&str>,
    model_id: &str,
) -> &'static str {
    if !sdk_has_responses && !sdk_has_chat {
        return "languageModel";
    }
    if endpoint == Some("responses") && sdk_has_responses {
        return "responses";
    }
    if endpoint == Some("chat") && sdk_has_chat {
        return "chat";
    }
    // Copilot supports Responses for GPT-5 class models, except mini variants
    if let Some(caps) = model_id.strip_prefix("gpt-") {
        if let Some(num_str) = caps.split(&['-', '.'][..]).next() {
            if let Ok(n) = num_str.parse::<u32>() {
                if n >= 5 && !model_id.starts_with("gpt-5-mini") && sdk_has_responses {
                    return "responses";
                }
            }
        }
    }
    "chat"
}

pub fn build_headers(
    api_key: Option<&str>,
    extra: std::collections::HashMap<String, String>,
    version: &str,
) -> std::collections::HashMap<String, String> {
    let mut h = extra;
    if let Some(k) = api_key {
        h.insert("Authorization".to_string(), format!("Bearer {k}"));
    }
    // withUserAgentSuffix adds ai-sdk/openai-compatible/VERSION
    let ua_suffix = format!("ai-sdk/openai-compatible/{version}");
    // preserve existing User-Agent if any else default opencode
    h.entry("User-Agent".to_string())
        .or_insert_with(|| format!("opencode/{version} {ua_suffix}"));
    h
}
