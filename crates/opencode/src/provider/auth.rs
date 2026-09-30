// source: src/provider/auth.ts — exports: Method, Methods,
// Authorization, AuthorizeInput, CallbackInput, OauthMissing,
// OauthCodeMissing, OauthCallbackFailed, ValidationFailed, Error,
// Interface, Service, use, node, ProviderAuth
// PROVISIONAL pending core (layer-node, provider, service-use,
// schema optional) + @/*: prompt shapes, input descriptions, method-type
// branches verbatim.

use serde::{Deserialize, Serialize};

/// source: When { key, op: eq|neq, value } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct When {
    pub key: String,
    pub op: String,
    pub value: String,
}

/// source: TextPrompt — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPrompt {
    pub key: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<When>,
}

/// source: SelectOption { label, value, hint? } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

/// source: SelectPrompt — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectPrompt {
    pub key: String,
    pub message: String,
    pub options: Vec<SelectOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<When>,
}

/// source: Prompt union — verbatim tags.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Prompt {
    #[serde(rename = "text")]
    Text(TextPrompt),
    #[serde(rename = "select")]
    Select(SelectPrompt),
}

/// source: Method ("ProviderAuthMethod") { type: oauth|api, label, prompts? } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    #[serde(rename = "type")]
    pub method_type: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<Vec<Prompt>>,
}

/// source: Authorization ("ProviderAuthAuthorization") — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authorization {
    pub url: String,
    pub method: String,
    pub instructions: String,
}

/// source: AuthorizeInput { method ("Auth method index"), inputs? ("Prompt inputs") } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeInput {
    pub method: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<std::collections::HashMap<String, String>>,
}

/// source: CallbackInput { method, code? ("OAuth authorization code") } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackInput {
    pub method: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// source: OauthMissing / OauthCodeMissing { providerID } — verbatim tags.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OauthMissing {
    pub provider_id: String,
}

/// source: OauthCallbackFailed {} — verbatim tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OauthCallbackFailed {}

/// source: ValidationFailed { field, message } — verbatim tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationFailed {
    pub field: String,
    pub message: String,
}

/// source: Interface — methods/authorize/callback, verbatim.
pub trait Interface {
    fn authorize(&self, provider_id: &str, input: &AuthorizeInput) -> Option<Authorization>;
}

/// source: Service "@opencode/ProviderAuth" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/ProviderAuth";

/// source: node deps [Auth.node, Plugin.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &["@/auth.Auth", "@/plugin.Plugin"];
