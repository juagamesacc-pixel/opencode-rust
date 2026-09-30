// source: packages/plugin/src/index.ts
#![allow(dead_code)]
#![allow(non_snake_case)]

//! Rust port of `packages/plugin/src/index.ts` (opencode v1.18.30).
//!
//! Source 335 lines. Re-exports `tool.js` plus Plugin/Hooks types.
//!
//! 1:1 notes:
//! - All type names/signatures/behavior preserved verbatim.
//! - `ProviderContext.source` values `"env"|"config"|"custom"|"api"` verbatim.
//! - `WorkspaceTarget` local/remote union verbatim.
//! - `Hooks` keys in source order verbatim.
//!
//! PROVISIONAL: `@opencode-ai/sdk` (`Project`, `Model`, `Provider`, `Permission`,
//! `UserMessage`, `Message`, `Part`, `Config`, `Event`), `@opencode-ai/sdk/v2`
//! (`Provider as ProviderV2`, `Model as ModelV2`, `Auth`), `BunShell` are pending
//! `crates/sdk` + `crates/plugin` shell — faithful stub types via `serde_json::Value`
//! with verbatim field names. No behavior reinterpretation.

pub mod example;
pub mod example_workspace;
pub mod shell;
pub mod tool;
pub mod tui;
pub mod v2;

pub use tool::{tool, ToolContext, ToolDefinition, ToolResult};

// ---------------------------------------------------------------------------
// Re-exported SDK types (PROVISIONAL — pending crates/sdk)
// ---------------------------------------------------------------------------

/// Mirrors `ProviderContext.source` literal union verbatim.
pub const PROVIDER_CONTEXT_SOURCES: &[&str] = &["env", "config", "custom", "api"];

/// Mirrors `ProviderContext` from `src/index.ts`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProviderContext {
    pub source: String,
    pub info: serde_json::Value,
    pub options: serde_json::Value,
}

/// Mirrors `WorkspaceInfo`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WorkspaceInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub name: String,
    pub branch: Option<String>,
    pub directory: Option<String>,
    pub extra: Option<serde_json::Value>,
    #[serde(rename = "projectID")]
    pub project_id: String,
}

/// Mirrors `WorkspaceTarget` union.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum WorkspaceTarget {
    #[serde(rename = "local")]
    Local { directory: String },
    #[serde(rename = "remote")]
    Remote {
        url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<serde_json::Value>,
    },
}

/// Mirrors `WorkspaceAdapter`.
#[derive(Clone, Debug, PartialEq)]
pub struct WorkspaceAdapter {
    pub name: String,
    pub description: String,
}

/// Mirrors `PluginInput`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginInput {
    pub client: serde_json::Value,
    pub project: serde_json::Value,
    pub directory: String,
    pub worktree: String,
    pub server_url: String,
}

/// Mirrors `PluginOptions`.
pub type PluginOptions = serde_json::Value;

/// Mirrors `Config`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub rest: serde_json::Value,
}

/// Mirrors `Plugin` fn signature descriptor.
pub type Plugin = serde_json::Value;

/// Mirrors `PluginModule`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginModule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub server: serde_json::Value,
}

/// Mirrors `Rule`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Rule {
    pub key: String,
    pub op: String,
    pub value: String,
}

/// Mirrors `AuthHook` method prompts union.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthPromptText {
    #[serde(rename = "type")]
    pub r#type: String,
    pub key: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<Rule>,
}

/// Mirrors `AuthHook`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthHook {
    pub provider: String,
    pub methods: Vec<serde_json::Value>,
}

/// Mirrors `AuthOAuthResult`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthOAuthResult {
    pub url: String,
    pub instructions: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<serde_json::Value>,
}

/// Mirrors `ProviderHookContext`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProviderHookContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Value>,
}

/// Mirrors `ProviderHook`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProviderHook {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<serde_json::Value>,
}

/// Mirrors `AuthOuathResult` deprecated alias.
pub type AuthOuathResult = AuthOAuthResult;

/// Mirrors `Hooks` interface keys in source order.
pub const HOOKS_KEYS: &[&str] = &[
    "dispose",
    "event",
    "config",
    "tool",
    "auth",
    "provider",
    "chat.message",
    "chat.params",
    "chat.headers",
    "permission.ask",
    "command.execute.before",
    "tool.execute.before",
    "shell.env",
    "tool.execute.after",
    "experimental.chat.messages.transform",
    "experimental.chat.system.transform",
    "experimental.provider.small_model",
    "experimental.session.compacting",
    "experimental.compaction.autocontinue",
    "experimental.text.complete",
    "tool.definition",
];

/// Mirrors `Hooks` descriptor with verbatim hook names.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Hooks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispose: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<serde_json::Value>,
    #[serde(rename = "chat.message", skip_serializing_if = "Option::is_none")]
    pub chat_message: Option<serde_json::Value>,
    #[serde(rename = "chat.params", skip_serializing_if = "Option::is_none")]
    pub chat_params: Option<serde_json::Value>,
    #[serde(rename = "chat.headers", skip_serializing_if = "Option::is_none")]
    pub chat_headers: Option<serde_json::Value>,
    #[serde(rename = "permission.ask", skip_serializing_if = "Option::is_none")]
    pub permission_ask: Option<serde_json::Value>,
    #[serde(
        rename = "command.execute.before",
        skip_serializing_if = "Option::is_none"
    )]
    pub command_execute_before: Option<serde_json::Value>,
    #[serde(
        rename = "tool.execute.before",
        skip_serializing_if = "Option::is_none"
    )]
    pub tool_execute_before: Option<serde_json::Value>,
    #[serde(rename = "shell.env", skip_serializing_if = "Option::is_none")]
    pub shell_env: Option<serde_json::Value>,
    #[serde(rename = "tool.execute.after", skip_serializing_if = "Option::is_none")]
    pub tool_execute_after: Option<serde_json::Value>,
    #[serde(
        rename = "experimental.chat.messages.transform",
        skip_serializing_if = "Option::is_none"
    )]
    pub experimental_chat_messages_transform: Option<serde_json::Value>,
    #[serde(
        rename = "experimental.chat.system.transform",
        skip_serializing_if = "Option::is_none"
    )]
    pub experimental_chat_system_transform: Option<serde_json::Value>,
    #[serde(
        rename = "experimental.provider.small_model",
        skip_serializing_if = "Option::is_none"
    )]
    pub experimental_provider_small_model: Option<serde_json::Value>,
    #[serde(
        rename = "experimental.session.compacting",
        skip_serializing_if = "Option::is_none"
    )]
    pub experimental_session_compacting: Option<serde_json::Value>,
    #[serde(
        rename = "experimental.compaction.autocontinue",
        skip_serializing_if = "Option::is_none"
    )]
    pub experimental_compaction_autocontinue: Option<serde_json::Value>,
    #[serde(
        rename = "experimental.text.complete",
        skip_serializing_if = "Option::is_none"
    )]
    pub experimental_text_complete: Option<serde_json::Value>,
    #[serde(rename = "tool.definition", skip_serializing_if = "Option::is_none")]
    pub tool_definition: Option<serde_json::Value>,
}

/// PROVISIONAL: `@opencode-ai/sdk` pending `crates/sdk`.
pub mod sdk_provisional {
    pub const PACKAGE: &str = "@opencode-ai/sdk";
    pub const V2_PACKAGE: &str = "@opencode-ai/sdk/v2";
    pub const PENDING_CRATE: &str = "crates/sdk";
}

/// PROVISIONAL: `BunShell` pending bun runtime.
pub mod shell_provisional {
    pub const MODULE: &str = "./shell.js";
    pub const PENDING_CRATE: &str = "bun";
}
