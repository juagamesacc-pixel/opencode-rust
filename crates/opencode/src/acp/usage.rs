// source: src/acp/usage.ts — exports: AssistantTokenCost,
// AssistantMessage, SessionMessage, MessagesInput, SDK, MessageLoaderInterface,
// ContextLimitLoaderInterface, UsageConnection, Interface, MessageLoader,
// ContextLimitLoader, Service, messageLoaderFromSDK, messageLoaderLayer,
// contextTokens, buildUsage, latestAssistantMessage, totalSessionCost,
// findContextLimit, messageLoaderNode, contextLimitLoaderNode, node, UsageService
// PROVISIONAL pending ACP sdk + sdk/v2 + core (provider, model, app-node,
// layer-node) + @/*: usage arithmetic (thought/cached conditional spread,
// total, cost sum, contextTokens), cache key join \u0000, currency USD,
// sessionUpdate "usage_update", log strings verbatim.

use serde::{Deserialize, Serialize};

/// source: Tokens { input, output, reasoning, cache{read,write} } — verbatim shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tokens {
    pub input: i64,
    pub output: i64,
    pub reasoning: i64,
    pub cache: CacheTokens,
}

/// source: cache { read, write } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheTokens {
    pub read: i64,
    pub write: i64,
}

/// source: contextTokens() — input + cache.read + cache.write. Verbatim.
pub fn context_tokens(input: i64, read: i64, write: i64) -> i64 {
    input + read + write
}

/// source: buildUsage() — totals + conditional thought/cached spreads. Verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub total_tokens: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thought_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_read_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_write_tokens: Option<i64>,
}

/// source: buildUsage() — verbatim arithmetic.
pub fn build_usage(input: i64, output: i64, thought: i64, read: i64, write: i64) -> Usage {
    Usage {
        input_tokens: input,
        output_tokens: output,
        total_tokens: input + output + thought + read + write,
        thought_tokens: if thought > 0 { Some(thought) } else { None },
        cached_read_tokens: if read > 0 { Some(read) } else { None },
        cached_write_tokens: if write > 0 { Some(write) } else { None },
    }
}

/// source: totalSessionCost() — sum of assistant costs. Verbatim.
pub fn total_session_cost(costs: &[f64]) -> f64 {
    costs.iter().sum()
}

/// source: cache key `${directory}\0${providerID}\0${modelID}` — verbatim.
pub fn limit_cache_key(directory: &str, provider_id: &str, model_id: &str) -> String {
    format!("{}\0{}\0{}", directory, provider_id, model_id)
}

/// source: cost currency "USD" — verbatim.
pub const COST_CURRENCY: &str = "USD";
/// source: sessionUpdate "usage_update" — verbatim.
pub const USAGE_UPDATE: &str = "usage_update";

/// source: log strings — verbatim.
pub const LOG_LIMIT_FAILED: &str = "failed to get providers for usage context limit";
pub const LOG_MESSAGES_FAILED: &str = "failed to fetch messages for usage update";

/// source: Service "@opencode/ACPUsage" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/ACPUsage";
/// source: MessageLoader "@opencode/ACPUsageMessageLoader" — verbatim.
pub const MESSAGE_LOADER_ID: &str = "@opencode/ACPUsageMessageLoader";
/// source: ContextLimitLoader "@opencode/ACPUsageContextLimitLoader" — verbatim.
pub const LIMIT_LOADER_ID: &str = "@opencode/ACPUsageContextLimitLoader";
