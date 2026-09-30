// source: src/plugin/cloudflare.ts — exports: CloudflareWorkersAuthPlugin,
// CloudflareAIGatewayAuthPlugin (env-gated prompts verbatim).
// PROVISIONAL pending plugin Hooks.

/// source: providers — verbatim.
pub const WORKERS_PROVIDER: &str = "cloudflare-workers-ai";
pub const GATEWAY_PROVIDER: &str = "cloudflare-ai-gateway";

/// source: env keys — verbatim.
pub const ACCOUNT_ENV: &str = "CLOUDFLARE_ACCOUNT_ID";
pub const GATEWAY_ENV: &str = "CLOUDFLARE_GATEWAY_ID";

/// source: account prompt — verbatim strings.
pub const ACCOUNT_KEY: &str = "accountId";
pub const ACCOUNT_MESSAGE: &str = "Enter your Cloudflare Account ID";
pub const ACCOUNT_PLACEHOLDER: &str = "e.g. 1234567890abcdef1234567890abcdef";

/// source: gateway prompt + label — verbatim strings.
pub const GATEWAY_KEY: &str = "gatewayId";
pub const GATEWAY_MESSAGE: &str = "Enter your Cloudflare AI Gateway ID";
pub const GATEWAY_PLACEHOLDER: &str = "e.g. my-gateway";
pub const GATEWAY_LABEL: &str = "Gateway API token";
pub const API_LABEL: &str = "API key";
