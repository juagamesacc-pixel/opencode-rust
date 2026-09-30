// source: src/mcp/oauth-callback.ts — exports: ensureRunning,
// waitForCallback, cancelPending, isPortInUse, stop, isRunning, McpOAuthCallback
// PROVISIONAL: node http/net server as descriptors; host/port/path,
// timeout, CSRF/state messages, page provider "MCP" verbatim.

/// source: OAUTH_CALLBACK_HOST — verbatim.
pub const OAUTH_CALLBACK_HOST: &str = "127.0.0.1";

/// source: CALLBACK_TIMEOUT_MS = 5*60*1000 — verbatim.
pub const CALLBACK_TIMEOUT_MS: u64 = 5 * 60 * 1000;

/// source: "Not found" 404 — verbatim.
pub const NOT_FOUND_BODY: &str = "Not found";

/// source: CSRF/state messages — verbatim.
pub const MISSING_STATE_MESSAGE: &str = "Missing required state parameter - potential CSRF attack";
pub const INVALID_STATE_MESSAGE: &str =
    "Invalid or expired state parameter - potential CSRF attack";
pub const NO_CODE_MESSAGE: &str = "No authorization code provided";

/// source: timeout/cancel/stop messages — verbatim.
pub const TIMEOUT_MESSAGE: &str = "OAuth callback timeout - authorization took too long";
pub const CANCELLED_MESSAGE: &str = "Authorization cancelled";
pub const STOPPED_MESSAGE: &str = "OAuth callback server stopped";

/// source: page provider "MCP" — verbatim.
pub const PAGE_PROVIDER: &str = "MCP";
