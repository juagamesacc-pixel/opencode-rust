// source: src/plugin/openai/ws-pool.ts — exports: TITLE_HEADER,
// CreateWebSocketFetchOptions, createWebSocketFetch,
// withoutInternalHeaders, OpenAIWebSocketPool
// PROVISIONAL: pool as descriptors; header/timeout consts verbatim.

/// source: TITLE_HEADER — verbatim.
pub const TITLE_HEADER: &str = "x-opencode-title";

/// source: DEFAULT_CONNECT_TIMEOUT = 15_000 — verbatim.
pub const DEFAULT_CONNECT_TIMEOUT: u64 = 15_000;
/// source: DEFAULT_IDLE_TIMEOUT = 5*60*1000 — verbatim.
pub const DEFAULT_IDLE_TIMEOUT: u64 = 5 * 60 * 1000;
/// source: DEFAULT_MAX_CONNECTION_AGE = 55*60*1000 — verbatim.
pub const DEFAULT_MAX_CONNECTION_AGE: u64 = 55 * 60 * 1000;
/// source: CONNECTION_LIMIT_REACHED_CODE — verbatim.
pub const CONNECTION_LIMIT_REACHED_CODE: &str = "websocket_connection_limit_reached";
