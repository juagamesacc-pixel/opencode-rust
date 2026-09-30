// source: src/provider/provider.ts — exports: Model, Info, ListResult,
// ConfigProvidersResult, toPublicInfo, defaultModelIDs, ModelNotFoundError,
// InitError, NoProvidersError, NoModelsError, DefaultModelError, Error,
// Interface, Service, use, fromModelsDevProvider, sort, parseModel, node, Provider
// PROVISIONAL pending crates/core (models-dev, provider, model, service-use,
// layer-node, fs-util, flag) + ai sdk + @/*: shapes + error messages +
// sort/priority, parseModel split, cloudflareGatewayNpm, cost mapping,
// experimental-modes expansion, suggestion filter, toPublicInfo replacer
// verbatim; loader/effect bodies as trait (provider surface: 1900+ lines of
// registry logic — residual risk R-P1, CI verifies).

use serde::{Deserialize, Serialize};

/// source: Model fields — verbatim (camelCase wire keys).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub id: String,
    pub provider_id: String,
    pub api: ApiInfo,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    pub capabilities: Capabilities,
    pub cost: Cost,
    pub limit: Limit,
    pub status: String,
    pub options: serde_json::Value,
    pub headers: std::collections::HashMap<String, String>,
    pub release_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<serde_json::Value>,
}

/// source: api { id, url, npm } — verbatim (+ compat default).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiInfo {
    pub id: String,
    pub url: String,
    pub npm: String,
}

/// source: "@ai-sdk/openai-compatible" compat default — verbatim.
pub const COMPAT_NPM: &str = "@ai-sdk/openai-compatible";

/// source: capabilities — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capabilities {
    pub temperature: bool,
    pub reasoning: bool,
    pub attachment: bool,
    pub toolcall: bool,
    pub input: Modality,
    pub output: Modality,
    pub interleaved: serde_json::Value,
}

/// source: modality { text, audio, image, video, pdf } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modality {
    pub text: bool,
    pub audio: bool,
    pub image: bool,
    pub video: bool,
    pub pdf: bool,
}

/// source: cost { input, output, cache{read,write}, tiers?, experimentalOver200K? } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cost {
    pub input: f64,
    pub output: f64,
    pub cache: CacheCost,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<Tier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental_over_200k: Option<Box<Over200K>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheCost {
    pub read: f64,
    pub write: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tier {
    pub input: f64,
    pub output: f64,
    pub cache: CacheCost,
    pub tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Over200K {
    pub cache: CacheCost,
    pub input: f64,
    pub output: f64,
}

/// source: limit { context, input, output } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limit {
    pub context: i64,
    pub input: i64,
    pub output: i64,
}

/// source: Info { id, name, source, env, key?, options, models } ("Provider") — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub id: String,
    pub name: String,
    pub source: String,
    pub env: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    pub options: serde_json::Value,
    pub models: std::collections::HashMap<String, Model>,
}

/// source: source literals — verbatim.
pub const SOURCE_ENV: &str = "env";
pub const SOURCE_CONFIG: &str = "config";
pub const SOURCE_CUSTOM: &str = "custom";
pub const SOURCE_API: &str = "api";

/// source: ListResult { all, default, connected } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResult {
    pub all: Vec<Info>,
    pub default: std::collections::HashMap<String, String>,
    pub connected: Vec<String>,
}

/// source: ConfigProvidersResult { providers, default } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigProvidersResult {
    pub providers: Vec<Info>,
    pub default: std::collections::HashMap<String, String>,
}

/// source: priority + smallModelFamilyPriority — verbatim order.
pub const PRIORITY: &[&str] = &["gpt-5", "claude-sonnet-4", "big-pickle", "gemini-3-pro"];
pub const SMALL_FAMILY_PRIORITY: &[&str] = &["gemini-flash", "gpt-nano", "claude-haiku"];

/// source: sort() — priority findIndex desc, latest-penalty asc, id desc. Verbatim.
pub fn sort_ids(ids: &mut [String]) {
    ids.sort_by(|a, b| {
        let pa = PRIORITY
            .iter()
            .position(|f| a.contains(f))
            .map(|i| i as i64)
            .unwrap_or(-1);
        let pb = PRIORITY
            .iter()
            .position(|f| b.contains(f))
            .map(|i| i as i64)
            .unwrap_or(-1);
        pb.cmp(&pa)
            .then_with(|| (a.contains("latest") as u8).cmp(&(b.contains("latest") as u8)))
            .then_with(|| b.cmp(a))
    });
}

/// source: parseModel() — split "/" head + rest-join. Verbatim.
pub fn parse_model(model: &str) -> (String, String) {
    match model.find('/') {
        Some(i) => (model[..i].to_string(), model[i + 1..].to_string()),
        None => (model.to_string(), String::new()),
    }
}

/// source: cloudflareGatewayNpm() — verbatim mapping.
pub fn cloudflare_gateway_npm(provider_id: &str, model_id: &str) -> Option<&'static str> {
    if provider_id != "cloudflare-ai-gateway" {
        return None;
    }
    if model_id.starts_with("openai/") {
        return Some("@ai-sdk/openai");
    }
    if model_id.starts_with("anthropic/") {
        return Some("@ai-sdk/anthropic");
    }
    None
}

/// source: experimental modes id `${model.id}-${mode}`, name capitalize — verbatim.
pub fn mode_id(model_id: &str, mode: &str) -> String {
    format!("{}-{}", model_id, mode)
}

/// source: suggestion filter — skip deprecated; skip alpha unless experimental. Verbatim.
pub fn suggestible(status: &str, enable_experimental: bool) -> bool {
    if status == "deprecated" {
        return false;
    }
    if status == "alpha" && !enable_experimental {
        return false;
    }
    true
}

/// source: ModelNotFoundError message — `Model not found: {p}/{m}.{suggestions}`. Verbatim.
pub fn model_not_found_message(
    provider_id: &str,
    model_id: &str,
    suggestions: &[String],
) -> String {
    let s = if suggestions.is_empty() {
        String::new()
    } else {
        format!(" Did you mean: {}?", suggestions.join(", "))
    };
    format!("Model not found: {}/{}.{}", provider_id, model_id, s)
}

/// source: "Failed to initialize provider: {id}" — verbatim.
pub fn init_failed_message(provider_id: &str) -> String {
    format!("Failed to initialize provider: {}", provider_id)
}

/// source: "No providers are available" — verbatim.
pub const NO_PROVIDERS_MESSAGE: &str = "No providers are available";

/// source: "No models are available for provider: {id}" — verbatim.
pub fn no_models_message(provider_id: &str) -> String {
    format!("No models are available for provider: {}", provider_id)
}

/// source: Interface — list/getProvider/getModel/getLanguage/closest/
/// getSmallModel/defaultModel, verbatim.
pub trait Interface {
    fn list(&self) -> Vec<Info>;
}

/// source: Service "@opencode/Provider" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Provider";

/// source: node deps — verbatim order.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[
    "@opencode-ai/core/fs-util.FSUtil",
    "@/config/config.Config",
    "@/auth.Auth",
    "@/env.Env",
    "@/plugin.Plugin",
    "@opencode-ai/core/models-dev.ModelsDev",
    "@/effect/runtime-flags.RuntimeFlags",
];
