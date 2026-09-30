//! Rust port of `packages/core/src/plugin/provider/cloudflare-ai-gateway.ts`.

pub const ID: &str = "cloudflare-ai-gateway";
pub const PACKAGE: &str = "ai-gateway-provider";

pub struct GatewayConfig {
    pub account_id: String,
    pub gateway_id: String,
    pub api_key: String,
}

pub fn gateway_config(
    options: &std::collections::HashMap<String, String>,
) -> Option<GatewayConfig> {
    let account_id = std::env::var("CLOUDFLARE_ACCOUNT_ID")
        .ok()
        .or_else(|| options.get("accountId").cloned())?;
    let gateway_id = std::env::var("CLOUDFLARE_GATEWAY_ID")
        .ok()
        .or_else(|| options.get("gatewayId").cloned())
        .or_else(|| options.get("gateway").cloned())?;
    let api_key = std::env::var("CLOUDFLARE_API_TOKEN")
        .ok()
        .or_else(|| std::env::var("CF_AIG_TOKEN").ok())
        .or_else(|| options.get("apiKey").cloned())?;
    if account_id.is_empty() || gateway_id.is_empty() || api_key.is_empty() {
        return None;
    }
    Some(GatewayConfig {
        account_id,
        gateway_id,
        api_key,
    })
}

pub fn is_workers_ai(model_id: &str) -> bool {
    model_id.starts_with("workers-ai/") || model_id.starts_with("@cf/")
}
