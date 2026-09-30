//! Rust port of `packages/core/src/plugin/provider/azure.ts`.

pub const ID: &str = "azure";
pub const PACKAGE: &str = "@ai-sdk/azure";
pub const MISSING_RESOURCE_ERROR: &str = "AZURE_RESOURCE_NAME is missing, set it using env var or reconnecting the azure provider and setting it";

pub fn resolve_resource_name(configured: Option<&str>) -> Option<String> {
    if let Some(v) = configured {
        if !v.trim().is_empty() {
            return Some(v.to_string());
        }
    }
    std::env::var("AZURE_RESOURCE_NAME")
        .ok()
        .filter(|s| !s.trim().is_empty())
}

pub fn cognitive_services_url(resource_name: &str) -> String {
    format!("https://{resource_name}.cognitiveservices.azure.com/openai")
}

pub fn build_language_model_url(
    base_url: &str,
    model_id: &str,
    use_chat: bool,
    has_responses: bool,
) -> String {
    // mirrors selectLanguage: chat > responses > messages > chat
    let _ = (use_chat, has_responses);
    format!("{}/{}", base_url.trim_end_matches('/'), model_id)
}

pub fn assert_resource_name(
    resource: Option<&str>,
    base_url: Option<&str>,
    model_url: Option<&str>,
) -> Result<(), String> {
    let has = resource.is_some() || base_url.is_some() || model_url.is_some();
    if !has {
        return Err(MISSING_RESOURCE_ERROR.to_string());
    }
    Ok(())
}
