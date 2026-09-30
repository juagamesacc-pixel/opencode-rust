// source: src/mcp/oauth-provider.ts — exports: McpOAuthConfig,
// McpOAuthCallbacks, McpOAuthProvider, OAUTH_CALLBACK_PORT,
// OAUTH_CALLBACK_PATH, parseRedirectUri (+ rest per source)
// PROVISIONAL pending MCP sdk auth: port/path, redirect rule, client_name
// "OpenCode", grant/response types, auth-method rule verbatim.

/// source: OAUTH_CALLBACK_PORT = 19876 — verbatim.
pub const OAUTH_CALLBACK_PORT: u16 = 19876;
/// source: OAUTH_CALLBACK_PATH — verbatim.
pub const OAUTH_CALLBACK_PATH: &str = "/mcp/oauth/callback";

/// source: redirectUrl() — redirectUri ?? 127.0.0.1:port+path. Verbatim.
pub fn redirect_url(redirect_uri: Option<&str>, callback_port: Option<u16>) -> String {
    if let Some(u) = redirect_uri {
        return u.to_string();
    }
    format!(
        "http://127.0.0.1:{}{}",
        callback_port.unwrap_or(OAUTH_CALLBACK_PORT),
        OAUTH_CALLBACK_PATH
    )
}

/// source: client_name "OpenCode", client_uri, grant/response types — verbatim.
pub const CLIENT_NAME: &str = "OpenCode";
pub const CLIENT_URI: &str = "https://opencode.ai";
pub const GRANT_TYPES: &[&str] = &["authorization_code", "refresh_token"];
pub const RESPONSE_TYPES: &[&str] = &["code"];

/// source: token_endpoint_auth_method — clientSecret ? post : none. Verbatim.
pub fn auth_method(has_secret: bool) -> &'static str {
    if has_secret {
        "client_secret_post"
    } else {
        "none"
    }
}
