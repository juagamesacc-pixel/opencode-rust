// source: src/mcp/auth.ts — exports: Tokens, ClientInfo, Entry,
// Interface, Service, use, node, McpAuth
// PROVISIONAL pending core (layer-node, global, fs-util, effect-flock,
// service-use): file "mcp-auth.json", lock key, mode 0o600, getForUrl
// serverUrl-match rule verbatim.

use serde::{Deserialize, Serialize};

/// source: Tokens { accessToken, refreshToken?, expiresAt?, scope? } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tokens {
    pub access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

/// source: ClientInfo — verbatim camelCase fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id_issued_at: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret_expires_at: Option<f64>,
}

/// source: Entry { tokens?, clientInfo?, codeVerifier?, oauthState?, serverUrl? } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Tokens>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_info: Option<ClientInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
}

/// source: file "mcp-auth.json" — verbatim.
pub const AUTH_FILE: &str = "mcp-auth.json";
/// source: lockKey `mcp-auth:${filepath}` — verbatim.
pub fn lock_key(filepath: &str) -> String {
    format!("mcp-auth:{}", filepath)
}
/// source: write mode 0o600 — verbatim.
pub const WRITE_MODE: u32 = 0o600;

/// source: getForUrl() — entry + serverUrl match required. Verbatim rule.
pub fn matches_url(entry: Option<&Entry>, server_url: &str) -> bool {
    match entry.and_then(|e| e.server_url.as_deref()) {
        Some(u) => u == server_url,
        None => false,
    }
}

/// source: Interface methods — verbatim names/order.
pub const AUTH_METHODS: &[&str] = &[
    "all",
    "get",
    "getForUrl",
    "set",
    "remove",
    "updateTokens",
    "updateClientInfo",
    "updateCodeVerifier",
    "clearCodeVerifier",
    "updateOAuthState",
    "getOAuthState",
    "clearOAuthState",
];

/// source: Service "@opencode/McpAuth" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/McpAuth";

/// source: node deps [FSUtil.node, EffectFlock.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[
    "@opencode-ai/core/fs-util.FSUtil",
    "@opencode-ai/core/util/effect-flock.EffectFlock",
];
