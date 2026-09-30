// source: src/auth/index.ts — exports: OAUTH_DUMMY_KEY, Oauth, Api,
// WellKnown, Info, AuthError, Interface, Service, node, Auth
// PROVISIONAL pending crates/core (layer-node, schema NonNegativeInt, global,
// fs-util): file path Global.Path.data/auth.json; key normalization verbatim.

use serde::{Deserialize, Serialize};

/// source: OAUTH_DUMMY_KEY — verbatim.
pub const OAUTH_DUMMY_KEY: &str = "opencode-oauth-dummy-key";

/// source: auth.json filename — verbatim.
pub const AUTH_FILE: &str = "auth.json";

/// source: Oauth — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Oauth {
    #[serde(rename = "type")]
    pub auth_type: String,
    pub refresh: String,
    pub access: String,
    pub expires: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_url: Option<String>,
}

/// source: Api ("ApiAuth") — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Api {
    #[serde(rename = "type")]
    pub auth_type: String,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

/// source: WellKnown ("WellKnownAuth") — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellKnown {
    #[serde(rename = "type")]
    pub auth_type: String,
    pub key: String,
    pub token: String,
}

/// source: Info = Union([Oauth, Api, WellKnown]) discriminator "type" — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Info {
    #[serde(rename = "oauth")]
    Oauth(Oauth),
    #[serde(rename = "api")]
    Api(Api),
    #[serde(rename = "wellknown")]
    WellKnown(WellKnown),
}

/// source: AuthError — verbatim (message + optional cause).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthError: {}", self.message)
    }
}

impl std::error::Error for AuthError {}

/// source: fail("Failed to write auth data") — verbatim message.
pub const FAIL_WRITE_MESSAGE: &str = "Failed to write auth data";

/// source: OPENCODE_AUTH_CONTENT env override — verbatim key.
pub const AUTH_CONTENT_ENV: &str = "OPENCODE_AUTH_CONTENT";

/// source: normalize key — key.replace(/\/+$/, "") — verbatim.
pub fn normalize_key(key: &str) -> String {
    key.trim_end_matches('/').to_string()
}

/// source: Interface — get/all/set/remove, verbatim.
pub trait Interface {
    fn get(&self, provider_id: &str) -> Result<Option<Info>, AuthError>;
    fn all(&self) -> Result<std::collections::HashMap<String, Info>, AuthError>;
    fn set(&mut self, key: &str, info: Info) -> Result<(), AuthError>;
    fn remove(&mut self, key: &str) -> Result<(), AuthError>;
}

/// source: Service "@opencode/Auth" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Auth";

/// source: node deps [FSUtil.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &["@opencode-ai/core/fs-util.FSUtil"];
