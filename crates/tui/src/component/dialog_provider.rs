// source: packages/tui/src/component/dialog-provider.tsx (469 lines, v1.18.30)
// 1:1 port — provider priority table + custom-id validation verbatim;
// option building (Popular/Providers categories, console-managed footer,
// connected gutter); auth-method selection; oauth (auto/code) and api
// flows with the verbatim toasts/descriptions; prompts-method `when`
// gating; all renders as data + state machines (no JSX).

#![allow(dead_code)]

use serde_json::Value;
use std::sync::Arc;

use crate::context::sdk::SdkClient;
use crate::ui::dialog::DialogStack;
use crate::ui::dialog_alert::show_alert;
use crate::ui::dialog_prompt::{show_prompt, PromptProps};
use crate::ui::dialog_select::{SelectOption, SelectState};
use crate::ui::toast::{ToastInput, ToastState};
use crate::util::error::error_message_value;
use crate::util::provider_origin::is_console_managed_provider;
use crate::util::selection::ToastVariant;

/// Priority table verbatim.
pub const PROVIDER_PRIORITY: &[(&str, u32)] = &[
    ("opencode", 0),
    ("opencode-go", 1),
    ("openai", 2),
    ("github-copilot", 3),
    ("anthropic", 4),
    ("google", 5),
];

/// Custom option value + id pattern verbatim.
pub const CUSTOM_PROVIDER_OPTION_VALUE: &str = "__opencode_custom_provider__";

/// Mirrors `ProviderOption` (provider vs custom).
#[derive(Debug, Clone)]
pub enum ProviderOption {
    Provider {
        title: String,
        value: String,
        provider_id: String,
        description: Option<String>,
        category: String,
    },
    Custom {
        title: String,
        value: String,
        description: Option<String>,
        category: String,
    },
}

fn provider_description(id: &str) -> Option<String> {
    match id {
        "opencode" => Some("(Recommended)".to_string()),
        "anthropic" => Some("(API key)".to_string()),
        "openai" => Some("(ChatGPT Plus/Pro or API key)".to_string()),
        "opencode-go" => Some("Low cost subscription for everyone".to_string()),
        _ => None,
    }
}

/// Mirrors `providerOptions` (priority sort + trailing custom entry).
pub fn provider_options(list: &[(String, String)]) -> Vec<ProviderOption> {
    let mut sorted = list.to_vec();
    sorted.sort_by(|a, b| {
        let priority = |id: &str| {
            PROVIDER_PRIORITY
                .iter()
                .find(|(known, _)| *known == id)
                .map(|(_, rank)| *rank)
                .unwrap_or(99)
        };
        priority(&a.0)
            .cmp(&priority(&b.0))
            .then_with(|| a.1.to_lowercase().cmp(&b.1.to_lowercase()))
            .then_with(|| a.0.cmp(&b.0))
    });
    let mut out: Vec<ProviderOption> = sorted
        .into_iter()
        .map(|(id, name)| ProviderOption::Provider {
            title: name,
            value: id.clone(),
            provider_id: id.clone(),
            description: provider_description(&id),
            category: if PROVIDER_PRIORITY.iter().any(|(known, _)| *known == id) {
                "Popular".to_string()
            } else {
                "Providers".to_string()
            },
        })
        .collect();
    out.push(ProviderOption::Custom {
        title: "Other".to_string(),
        value: CUSTOM_PROVIDER_OPTION_VALUE.to_string(),
        description: Some("Custom provider".to_string()),
        category: "Providers".to_string(),
    });
    out
}

/// Mirrors `normalizeCustomProviderID` (trim, strip `@ai-sdk/`, charset).
pub fn normalize_custom_provider_id(value: &str) -> Option<String> {
    let id = value
        .trim()
        .strip_prefix("@ai-sdk/")
        .unwrap_or(value.trim())
        .to_string();
    if id.is_empty() {
        return None;
    }
    let mut chars = id.chars();
    let first = chars.next()?;
    if !(first.is_ascii_lowercase() || first.is_ascii_digit()) {
        return None;
    }
    if !id
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '_')
    {
        return None;
    }
    Some(id)
}

/// Provider-auth method shape (subset read here).
#[derive(Debug, Clone)]
pub struct AuthMethod {
    pub method_type: String,
    pub label: String,
    pub prompts: Vec<Value>,
}

/// Default method when the server lists none (verbatim).
pub fn default_auth_method() -> AuthMethod {
    AuthMethod {
        method_type: "api".to_string(),
        label: "API key".to_string(),
        prompts: Vec::new(),
    }
}

/// Custom-id prompt description verbatim.
pub const CUSTOM_PROVIDER_DESCRIPTION: &str =
    "This only stores a credential. Configure the provider in opencode.json to use it.";
/// Custom-id error toast verbatim.
pub const CUSTOM_PROVIDER_ID_ERROR: &str =
    "Provider ids must start with a lowercase letter or number and only use lowercase letters, numbers, hyphens, and underscores";
/// Custom-provider saved toast (prefix verbatim).
pub fn custom_provider_saved_message(provider_id: &str) -> String {
    format!("Saved credential for {provider_id}. Configure it in opencode.json to use it.")
}

/// API-method descriptions for the ids that have one (verbatim copy).
pub fn api_method_description(provider_id: &str) -> Option<(&'static str, &'static str)> {
    match provider_id {
        "opencode" => Some((
            "OpenCode Zen gives you access to all the best coding models at the cheapest prices with a single API key.",
            "https://opencode.ai/zen",
        )),
        "opencode-go" => Some((
            "OpenCode Go is a $10 per month subscription that provides reliable access to popular open coding models with generous usage limits.",
            "https://opencode.ai/go",
        )),
        _ => None,
    }
}

/// OAuth code regex target (4 alnum `-` 4-5 alnum) + callback failure
/// message verbatim.
pub const OAUTH_CALLBACK_FAILED_MESSAGE: &str = "OAuth authorization failed. Try /connect again.";

/// Extract the clipboard code (`XXXX-XXXX[X]` else the URL, verbatim).
pub fn oauth_clipboard_code(instructions: &str, url: &str) -> String {
    let bytes = instructions.as_bytes();
    for window in 0..bytes.len() {
        for end in [window + 9, window + 10] {
            if end > bytes.len() {
                continue;
            }
            let candidate = &instructions[window..end];
            let mut chars = candidate.chars();
            let head: String = chars.by_ref().take(4).collect();
            if head.len() != 4
                || !head
                    .chars()
                    .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
            {
                continue;
            }
            if chars.next() != Some('-') {
                continue;
            }
            let tail: String = chars.collect();
            if (4..=5).contains(&tail.len())
                && tail
                    .chars()
                    .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
            {
                return candidate.to_string();
            }
        }
    }
    url.to_string()
}

/// Build the provider select state (connected gutter via `✓`, console
/// footer via the active org name).
pub fn provider_state(
    options: Vec<ProviderOption>,
    console_managed: &[String],
    active_org_name: Option<&str>,
    connected: &[String],
    onboarded: bool,
) -> SelectState {
    let select_options = options
        .into_iter()
        .map(|option| match option {
            ProviderOption::Custom {
                title,
                value,
                description,
                category,
            } => SelectOption {
                title,
                description,
                category: Some(category),
                value: Value::String(value),
                ..SelectOption::default()
            },
            ProviderOption::Provider {
                title,
                value,
                provider_id,
                description,
                category,
            } => {
                let managed = is_console_managed_provider(console_managed, &provider_id);
                let is_connected = connected.iter().any(|id| id == &provider_id);
                SelectOption {
                    title,
                    description,
                    category: Some(category),
                    footer: if managed {
                        active_org_name.map(str::to_string)
                    } else {
                        None
                    },
                    gutter: if is_connected && onboarded {
                        Some("connected".to_string())
                    } else {
                        None
                    },
                    value: Value::String(value),
                    ..SelectOption::default()
                }
            }
        })
        .collect();
    SelectState::new("Connect a provider", select_options)
}

/// Auth-method select state (title `Select auth method`, verbatim).
pub fn auth_method_state(methods: &[AuthMethod]) -> SelectState {
    SelectState::new(
        "Select auth method",
        methods
            .iter()
            .enumerate()
            .map(|(index, method)| SelectOption {
                title: method.label.clone(),
                value: Value::Number(index.into()),
                ..SelectOption::default()
            })
            .collect(),
    )
}

/// Prompts-method `when` gate (mirrors the eq/neq check).
pub fn prompt_applies(
    when: Option<&Value>,
    inputs: &std::collections::HashMap<String, String>,
) -> bool {
    let Some(when) = when else { return true };
    let Some(key) = when.get("key").and_then(|v| v.as_str()) else {
        return true;
    };
    let Some(value) = inputs.get(key) else {
        return false;
    };
    let expected = when.get("value").and_then(|v| v.as_str()).unwrap_or("");
    if when.get("op").and_then(|v| v.as_str()) == Some("eq") {
        value == expected
    } else {
        value != expected
    }
}

/// Custom provider id prompt props (title `Other`, verbatim).
pub fn custom_provider_prompt_props() -> PromptProps {
    PromptProps {
        title: "Other".to_string(),
        description: vec![CUSTOM_PROVIDER_DESCRIPTION.to_string()],
        placeholder: Some("Provider id".to_string()),
        value: None,
        busy: false,
        busy_text: None,
        submit_hint: None,
    }
}

/// API key prompt props (placeholder verbatim + per-id description).
pub fn api_key_prompt_props(provider_id: &str) -> PromptProps {
    let description = api_method_description(provider_id)
        .map(|(body, link)| vec![body.to_string(), format!("Go to {link}")])
        .unwrap_or_default();
    PromptProps {
        title: "API key".to_string(),
        description,
        placeholder: Some("API key".to_string()),
        value: None,
        busy: false,
        busy_text: None,
        submit_hint: None,
    }
}

/// Auth-code prompt props (placeholder verbatim).
pub fn auth_code_prompt_props(instructions: &str, url: &str) -> PromptProps {
    PromptProps {
        title: "Authorization code".to_string(),
        description: vec![instructions.to_string(), url.to_string()],
        placeholder: Some("Authorization code".to_string()),
        value: None,
        busy: false,
        busy_text: None,
        submit_hint: None,
    }
}

/// OAuth auto-method view lines (link + instructions + waiting line).
pub fn auto_method_lines(url: &str, instructions: &str) -> Vec<String> {
    vec![
        url.to_string(),
        instructions.to_string(),
        "Waiting for authorization…".to_string(),
    ]
}

/// Provider flow driver (method dispatch + oauth/api calls + follow-ups).
/// Returns the next dialog step for the app to mount.
pub enum ProviderNext {
    /// Show the model dialog for this provider.
    ShowModel { provider_id: String },
    /// Show an informational toast + clear.
    SavedCustom { provider_id: String },
    /// Stay (prompt loop continues via `show_*` receivers).
    Stay,
}

pub struct ProviderFlow<'a> {
    pub stack: &'a mut DialogStack,
    pub client: Arc<dyn SdkClient>,
    pub toast: &'a mut ToastState,
}

impl<'a> ProviderFlow<'a> {
    /// Custom provider id loop (mirrors `promptCustomProviderID`).
    pub async fn prompt_custom_provider_id(&mut self) -> Option<String> {
        loop {
            let value = show_prompt(self.stack, "Other", custom_provider_prompt_props())
                .await
                .ok()
                .flatten()?;
            if let Some(id) = normalize_custom_provider_id(&value) {
                return Some(id);
            }
            self.toast.show(ToastInput {
                title: None,
                message: CUSTOM_PROVIDER_ID_ERROR.to_string(),
                variant: Some(ToastVariant::Error),
                duration_ms: None,
            });
        }
    }

    /// API-key confirm (mirrors the `ApiMethod` onConfirm).
    pub async fn confirm_api_key(
        &mut self,
        provider_id: &str,
        metadata: Option<std::collections::HashMap<String, String>>,
        custom: bool,
        all_provider_ids: &[String],
        key: &str,
    ) -> ProviderNext {
        if key.is_empty() {
            return ProviderNext::Stay;
        }
        let mut auth = serde_json::json!({ "type": "api", "key": key });
        if let Some(metadata) = metadata {
            auth["metadata"] = serde_json::json!(metadata);
        }
        let _ = self
            .client
            .call(
                "auth.set",
                serde_json::json!({ "providerID": provider_id, "auth": auth }),
            )
            .await;
        let _ = self
            .client
            .call("instance.dispose", serde_json::json!({}))
            .await;
        // Bootstrap refresh happens in the app after dispose.
        if custom && !all_provider_ids.iter().any(|id| id == provider_id) {
            self.toast.show(ToastInput {
                title: None,
                message: custom_provider_saved_message(provider_id),
                variant: Some(ToastVariant::Info),
                duration_ms: None,
            });
            self.stack.clear();
            return ProviderNext::SavedCustom {
                provider_id: provider_id.to_string(),
            };
        }
        ProviderNext::ShowModel {
            provider_id: provider_id.to_string(),
        }
    }

    /// OAuth callback confirm for the code method (mirrors `CodeMethod` —
    /// `Err` carries the message so the caller can show `Invalid code`).
    pub async fn confirm_oauth_code(
        &mut self,
        provider_id: &str,
        method: usize,
        code: &str,
    ) -> Result<ProviderNext, String> {
        let result = self
            .client
            .call(
                "provider.oauth.callback",
                serde_json::json!({ "providerID": provider_id, "method": method, "code": code }),
            )
            .await;
        match result {
            Ok(_) => {
                let _ = self
                    .client
                    .call("instance.dispose", serde_json::json!({}))
                    .await;
                Ok(ProviderNext::ShowModel {
                    provider_id: provider_id.to_string(),
                })
            }
            Err(err) => Err(err),
        }
    }

    /// OAuth auto-method mount (mirrors `AutoMethod` onMount).
    pub async fn mount_oauth_auto(&mut self, provider_id: &str, method: usize) -> ProviderNext {
        let result = self
            .client
            .call(
                "provider.oauth.callback",
                serde_json::json!({ "providerID": provider_id, "method": method }),
            )
            .await;
        match result {
            Ok(_) => {
                let _ = self
                    .client
                    .call("instance.dispose", serde_json::json!({}))
                    .await;
                ProviderNext::ShowModel {
                    provider_id: provider_id.to_string(),
                }
            }
            Err(err) => {
                let message = if err.contains("ProviderAuthOauthCallbackFailed") {
                    OAUTH_CALLBACK_FAILED_MESSAGE.to_string()
                } else {
                    err
                };
                self.toast.show(ToastInput {
                    title: None,
                    message,
                    variant: Some(ToastVariant::Error),
                    duration_ms: None,
                });
                self.stack.clear();
                ProviderNext::Stay
            }
        }
    }

    /// OAuth authorize call (mirrors the authorize + method dispatch).
    pub async fn oauth_authorize(
        &mut self,
        provider_id: &str,
        method: usize,
        inputs: Option<std::collections::HashMap<String, String>>,
    ) -> OAuthAuthorizeNext {
        let result = self
            .client
            .call(
                "provider.oauth.authorize",
                serde_json::json!({ "providerID": provider_id, "method": method, "inputs": inputs }),
            )
            .await;
        match result {
            Err(err) => {
                self.toast.show(ToastInput {
                    title: None,
                    message: error_message_value(&Value::String(err)),
                    variant: Some(ToastVariant::Error),
                    duration_ms: None,
                });
                self.stack.clear();
                OAuthAuthorizeNext::Done
            }
            Ok(response) => {
                let data = response.get("data").cloned().unwrap_or(Value::Null);
                match data.get("method").and_then(|v| v.as_str()) {
                    Some("code") => OAuthAuthorizeNext::Code {
                        authorization: data,
                    },
                    Some("auto") => OAuthAuthorizeNext::Auto {
                        authorization: data,
                    },
                    _ => {
                        self.stack.clear();
                        OAuthAuthorizeNext::Done
                    }
                }
            }
        }
    }
}

/// Next step after `oauth.authorize`.
pub enum OAuthAuthorizeNext {
    Code { authorization: Value },
    Auto { authorization: Value },
    Done,
}

/// Copy-code helper for the auto view (mirrors the `c` binding).
pub fn auto_copy_code(instructions: &str, url: &str) -> String {
    oauth_clipboard_code(instructions, url)
}

pub async fn alert_unable_to_warp(stack: &mut DialogStack) {
    let _ = show_alert(
        stack,
        "Unable to Warp Session",
        "Unable to apply file changes to this workspace. It has existing changes that conflict or is based off a different branch. Session has not been warped.",
    )
    .await;
}
