// source: src/acp/service.ts — exports: AuthMethodID, Error, Interface,
// Service, make (+ ACP service wiring)
// PROVISIONAL pending ACP sdk + sdk/v2 + core + @/* (1105-line service):
// AuthMethodID, auth-method envelope (description/name/terminal-auth meta),
// agentCapabilities table (protocolVersion 1, loadSession, mcp http/sse,
// embeddedContext/image, close/fork/list/resume), 13-method interface table,
// service id verbatim; session/directory/usage/event wiring as trait.

/// source: AuthMethodID = "opencode-login" — verbatim.
pub const AUTH_METHOD_ID: &str = "opencode-login";

/// source: auth method envelope — verbatim strings.
pub const AUTH_DESCRIPTION: &str = "Run `opencode auth login` in the terminal";
pub const AUTH_NAME: &str = "Login with opencode";
pub const AUTH_COMMAND: &str = "opencode";
pub const AUTH_LABEL: &str = "OpenCode Login";

/// source: protocolVersion 1 — verbatim.
pub const PROTOCOL_VERSION: u32 = 1;

/// source: agentCapabilities — verbatim flags.
pub const CAP_LOAD_SESSION: bool = true;
pub const CAP_MCP_HTTP: bool = true;
pub const CAP_MCP_SSE: bool = true;
pub const CAP_EMBEDDED_CONTEXT: bool = true;
pub const CAP_IMAGE: bool = true;

/// source: Interface methods — verbatim names/order (13).
pub const SERVICE_METHODS: &[&str] = &[
    "initialize",
    "authenticate",
    "newSession",
    "loadSession",
    "listSessions",
    "resumeSession",
    "closeSession",
    "forkSession",
    "setSessionConfigOption",
    "setSessionMode",
    "setSessionModel",
    "prompt",
    "cancel",
];

/// source: Service "@opencode/ACP/Service" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/ACP/Service";
