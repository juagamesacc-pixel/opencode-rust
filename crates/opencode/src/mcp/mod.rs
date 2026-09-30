// source: src/mcp/index.ts — exports: Resource, ToolsChanged,
// BrowserOpenFailed, Failed, NotFoundError, Status, ServerInstructions,
// McpTool, Interface, Service, use, AuthStatus, node, MCP
// PROVISIONAL (1004-line service): Resource shape, Status discriminated
// union tags, "MCPFailed", "MCP.NotFoundError", client name "opencode",
// 18-method interface table, AuthStatus literals, service id verbatim.

use serde::{Deserialize, Serialize};

/// source: Resource ("McpResource") — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub name: String,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    pub client: String,
}

/// source: Status tags — verbatim discriminators.
pub const STATUS_CONNECTED: &str = "connected";
pub const STATUS_DISABLED: &str = "disabled";
pub const STATUS_FAILED: &str = "failed";
pub const STATUS_NEEDS_AUTH: &str = "needs_auth";
pub const STATUS_NEEDS_REGISTRATION: &str = "needs_client_registration";

/// source: "MCPFailed" { name } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Failed {
    pub name: String,
}

/// source: NotFoundError ("MCP.NotFoundError" { name }) — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotFoundError {
    pub name: String,
}

/// source: client name "opencode" — verbatim.
pub const CLIENT_NAME: &str = "opencode";

/// source: Interface methods — verbatim names/order (18).
pub const MCP_METHODS: &[&str] = &[
    "status",
    "clients",
    "instructions",
    "tools",
    "prompts",
    "resources",
    "resourceTemplates",
    "add",
    "connect",
    "disconnect",
    "getPrompt",
    "readResource",
    "startAuth",
    "authenticate",
    "finishAuth",
    "removeAuth",
    "supportsOAuth",
    "hasStoredTokens",
    "getAuthStatus",
];

/// source: AuthStatus literals — verbatim.
pub const AUTH_STATUSES: &[&str] = &["authenticated", "expired", "not_authenticated"];

/// source: Service "@opencode/MCP" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/MCP";
