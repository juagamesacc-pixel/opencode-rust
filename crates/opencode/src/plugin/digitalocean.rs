// source: src/plugin/digitalocean.ts — exports: DigitalOceanAuthPlugin
// PROVISIONAL: OAuth endpoints/ports/paths/scopes, refresh interval, URL
// builders verbatim; browser/server pensions as trait.

/// source: endpoints — verbatim.
pub const DO_AUTHORIZE_URL: &str = "https://cloud.digitalocean.com/v1/oauth/authorize";
pub const DO_API_BASE: &str = "https://api.digitalocean.com";
pub const DO_GENAI_API: &str = "https://api.digitalocean.com/v2/gen-ai";
pub const DO_INFERENCE_BASE: &str = "https://inference.do-ai.run/v1";

/// source: OAUTH_PORT = 1456 — verbatim.
pub const OAUTH_PORT: u16 = 1456;
/// source: OAUTH_REDIRECT_PATH — verbatim.
pub const OAUTH_REDIRECT_PATH: &str = "/auth/callback";
/// source: OAUTH_TOKEN_PATH — verbatim.
pub const OAUTH_TOKEN_PATH: &str = "/auth/token";
/// source: ROUTER_REFRESH_INTERVAL_MS — verbatim.
pub const ROUTER_REFRESH_INTERVAL_MS: u64 = 5 * 60 * 1000;
/// source: OAUTH_SCOPES — verbatim.
pub const OAUTH_SCOPES: &str = "genai:read inference:query";

/// source: redirectUri() — verbatim template.
pub fn redirect_uri() -> String {
    format!("http://localhost:{}{}", OAUTH_PORT, OAUTH_REDIRECT_PATH)
}

/// source: response_type "token" — verbatim.
pub const RESPONSE_TYPE: &str = "token";
