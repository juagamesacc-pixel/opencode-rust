//! Rust port of `packages/function/src/api.ts` (opencode v1.18.30).
//!
//! Source 388 lines. Exports: `SyncServer` (DurableObject), default `Hono` app.
//!
//! 1:1 notes:
//! - `Env = { SYNC_SERVER: DurableObjectNamespace<SyncServer>, Bucket: R2Bucket, WEB_DOMAIN: string }`.
//! - `SyncServer extends DurableObject<Env>` with methods `fetch`, `webSocketMessage`,
//!   `webSocketClose`, `publish`, `share`, `getData`, `assertSecret`, `getSecret`,
//!   `getSessionID`, `clear`, `shortName` — names/ordering verbatim.
//! - `publish` validates `key` prefixes `session/info/{sessionID}`, `session/message/{sessionID}/`,
//!   `session/part/{sessionID}/` else `400 "Error: Invalid key"`.
//! - `share` uses `randomUUID()` else cached secret; `assertSecret` throws `"Invalid secret"`.
//! - `clear` deletes `session/message/{sessionID}/` prefix limit 1000 + `session/info/{sessionID}` + `deleteAll()`.
//! - `shortName(id)=id.substring(id.length-8)`.
//! - Hono routes: `GET /` → `"Hello, world!"`; `POST /share_create` → `{sessionID}` → `{secret, url:"https://{WEB_DOMAIN}/s/{short}"}`;
//!   `POST /share_delete` → `{sessionID, secret}` → assert+clear → `{}`;
//!   `POST /share_delete_admin` → `{sessionShortName, adminSecret}` → `"Invalid admin secret"` if `!== Resource.ADMIN_SECRET.value`;
//!   `POST /share_sync` → `{sessionID, secret, key, content}` → assert+publish → `{}`;
//!   `GET /share_poll` → Upgrade `"websocket"` else `426 "Error: Upgrade header is required"`, `id` query else `400 "Error: Share ID is required"`;
//!   `GET /share_data` → `id` query else `400 "Error: Share ID is required"` → `{info, messages}` with `session/info`/`message`/`part` partitioning;
//!   `POST /feishu` → challenge passthrough, `content` JSON `text` branching, `@_user_` strip, `aiden` → `<@759257817772851260>`, `[threadId]` suffix, Discord forward `https://discord.com/api/v10/channels/{DISCORD_SUPPORT_CHANNEL_ID}/messages` Bot header, `502 "Discord bot message failed"`;
//!   `POST /exchange_github_app_token` → audience `"opencode-github-action"`, issuer `"https://token.actions.githubusercontent.com"`, `JWKS_URL` `/.well-known/jwks`, `401 "Authorization header is required"`, `403 "Invalid or expired token"`, `502 "Failed to exchange GitHub App token for {owner}/{repo}"`;
//!   `POST /exchange_github_app_token_with_pat` → `owner/repo` + Bearer, `throw "Authorization header is required"`, permissions `admin/push/maintain` else `"User does not have write permissions"`, `401` with `e.message`;
//!   `GET /get_github_app_installation` → `owner/repo` query, `Not Found` swallow else throw, → `{installation}`;
//!   `all "*" → "Not Found"`.
//!
//! PROVISIONAL: `cloudflare:workers` DurableObject/R2/WebSocketPair, `hono`, `sst Resource`,
//! `jose` JWT verify, `@octokit/*` are Workers/Bun surfaces — represented as descriptor
//! constants. No runtime reinterpretation.

// ---------------------------------------------------------------------------
// Env / DurableObject constants
// ---------------------------------------------------------------------------

/// `Env.WEB_DOMAIN` key verbatim.
pub const ENV_WEB_DOMAIN: &str = "WEB_DOMAIN";
/// `Env.SYNC_SERVER` binding key verbatim.
pub const ENV_SYNC_SERVER: &str = "SYNC_SERVER";
/// `Env.Bucket` binding key verbatim.
pub const ENV_BUCKET: &str = "Bucket";

/// DurableObject storage keys verbatim.
pub const STORAGE_SECRET: &str = "secret";
pub const STORAGE_SESSION_ID: &str = "sessionID";

/// WebSocket close reason verbatim: `"Durable Object is closing WebSocket"`.
pub const WS_CLOSE_REASON: &str = "Durable Object is closing WebSocket";
/// Subscribe log verbatim: `"SyncServer subscribe"`.
pub const LOG_SUBSCRIBE: &str = "SyncServer subscribe";
/// Publish log prefix: `"SyncServer publish"` (followed by key + subscriber count).
pub const LOG_PUBLISH: &str = "SyncServer publish";

// ---------------------------------------------------------------------------
// SyncServer service / key logic
// ---------------------------------------------------------------------------

/// Port of `SyncServer.shortName(id)` — last 8 chars verbatim.
pub fn short_name(id: &str) -> String {
    if id.len() <= 8 {
        id.to_string()
    } else {
        id[id.len() - 8..].to_string()
    }
}

/// Port of `SyncServer.publish` key-validation — returns `Err(400, "Error: Invalid key")` if invalid.
pub fn is_valid_publish_key(key: &str, session_id: &str) -> bool {
    key.starts_with(&format!("session/info/{}", session_id))
        || key.starts_with(&format!("session/message/{}/", session_id))
        || key.starts_with(&format!("session/part/{}/", session_id))
}

/// Error string verbatim for invalid publish key: `"Error: Invalid key"` (400).
pub const ERR_INVALID_KEY: &str = "Error: Invalid key";
pub const ERR_INVALID_KEY_STATUS: u16 = 400;

/// Error string verbatim for invalid secret: `"Invalid secret"`.
pub const ERR_INVALID_SECRET: &str = "Invalid secret";
/// Error string verbatim for invalid admin secret: `"Invalid admin secret"`.
pub const ERR_INVALID_ADMIN_SECRET: &str = "Invalid admin secret";

// ---------------------------------------------------------------------------
// R2 Bucket write keys
// ---------------------------------------------------------------------------

/// R2 put key prefix verbatim: `"share/{key}.json"`.
pub fn r2_share_key(key: &str) -> String {
    format!("share/{}.json", key)
}
/// R2 clear prefix: `"session/message/{sessionID}/"`.
pub fn r2_message_prefix(session_id: &str) -> String {
    format!("session/message/{}/", session_id)
}
/// R2 info key: `"session/info/{sessionID}"`.
pub fn r2_info_key(session_id: &str) -> String {
    format!("session/info/{}", session_id)
}
pub const R2_LIST_LIMIT: u32 = 1000;
pub const R2_CONTENT_TYPE: &str = "application/json";

// ---------------------------------------------------------------------------
// Hono routes / status / headers verbatim
// ---------------------------------------------------------------------------

pub const ROUTE_ROOT: &str = "/";
pub const ROUTE_SHARE_CREATE: &str = "/share_create";
pub const ROUTE_SHARE_DELETE: &str = "/share_delete";
pub const ROUTE_SHARE_DELETE_ADMIN: &str = "/share_delete_admin";
pub const ROUTE_SHARE_SYNC: &str = "/share_sync";
pub const ROUTE_SHARE_POLL: &str = "/share_poll";
pub const ROUTE_SHARE_DATA: &str = "/share_data";
pub const ROUTE_FEISHU: &str = "/feishu";
pub const ROUTE_EXCHANGE_GITHUB_APP_TOKEN: &str = "/exchange_github_app_token";
pub const ROUTE_EXCHANGE_GITHUB_APP_TOKEN_WITH_PAT: &str = "/exchange_github_app_token_with_pat";
pub const ROUTE_GET_GITHUB_APP_INSTALLATION: &str = "/get_github_app_installation";
pub const ROUTE_FALLBACK: &str = "*";

pub const HELLO_WORLD: &str = "Hello, world!";
pub const NOT_FOUND_TEXT: &str = "Not Found";

/// `"Error: Upgrade header is required"` status 426.
pub const ERR_UPGRADE_REQUIRED: &str = "Error: Upgrade header is required";
pub const STATUS_UPGRADE_REQUIRED: u16 = 426;
/// Upgrade header key verbatim: `"Upgrade"`.
pub const HEADER_UPGRADE: &str = "Upgrade";
pub const UPGRADE_WEBSOCKET: &str = "websocket";
/// `"Error: Share ID is required"` status 400.
pub const ERR_SHARE_ID_REQUIRED: &str = "Error: Share ID is required";
pub const QUERY_ID: &str = "id";

/// Share URL template: `"https://{WEB_DOMAIN}/s/{short}"`.
pub fn share_url(web_domain: &str, short: &str) -> String {
    format!("https://{}/s/{}", web_domain, short)
}

// ---------------------------------------------------------------------------
// share_data partitioning
// ---------------------------------------------------------------------------

/// Port of `share_data` partitioning — `data: Vec<{key, content}>` → `{info, messages}`.
///
/// Mirrors JS:
/// ```js
/// data.forEach((d) => {
///   const [root, type] = d.key.split("/");
///   if (root !== "session") return;
///   if (type === "info") { info = d.content; return; }
///   if (type === "message") { messages[d.content.id] = { parts:[], ...d.content }; }
///   if (type === "part") { messages[d.content.messageID].parts.push(d.content); }
/// });
/// ```
pub fn partition_share_data(
    data: Vec<(String, serde_json::Value)>,
) -> (
    Option<serde_json::Value>,
    serde_json::Map<String, serde_json::Value>,
) {
    let mut info: Option<serde_json::Value> = None;
    let mut messages: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    for (key, content) in data {
        let mut iter = key.split('/');
        let root = iter.next().unwrap_or("");
        let typ = iter.next().unwrap_or("");
        if root != "session" {
            continue;
        }
        if typ == "info" {
            info = Some(content);
            continue;
        }
        if typ == "message" {
            let id = content
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let mut obj = serde_json::Map::new();
            obj.insert("parts".to_string(), serde_json::Value::Array(vec![]));
            if let serde_json::Value::Object(map) = content.clone() {
                for (k, v) in map {
                    obj.insert(k, v);
                }
            }
            messages.insert(id, serde_json::Value::Object(obj));
        }
        if typ == "part" {
            let message_id = content
                .get("messageID")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            if let Some(entry) = messages.get_mut(&message_id) {
                if let Some(parts) = entry
                    .as_object_mut()
                    .and_then(|o| o.get_mut("parts"))
                    .and_then(|v| v.as_array_mut())
                {
                    parts.push(content);
                }
            }
        }
    }
    (info, messages)
}

// ---------------------------------------------------------------------------
// Feishu → Discord bridging constants
// ---------------------------------------------------------------------------

pub const FEISHU_MENTION_PREFIX_RE: &str = r"^@_user_\d+\s*";
pub const FEISHU_AIDEN_RE: &str = r"^aiden,?\s*";
pub const FEISHU_AIDEN_REPLACEMENT: &str = "<@759257817772851260> ";
pub const FEISHU_OK: &str = "ok";
pub const DISCORD_API_BASE: &str = "https://discord.com/api/v10/channels";
pub const DISCORD_AUTH_PREFIX: &str = "Bot ";
pub const DISCORD_ERROR: &str = "Discord bot message failed";
pub const DISCORD_ERROR_STATUS: u16 = 502;

// ---------------------------------------------------------------------------
// GitHub App token exchange constants
// ---------------------------------------------------------------------------

pub const EXPECTED_AUDIENCE: &str = "opencode-github-action";
pub const GITHUB_ISSUER: &str = "https://token.actions.githubusercontent.com";
pub const JWKS_SUFFIX: &str = "/.well-known/jwks";
pub fn jwks_url() -> String {
    format!("{}{}", GITHUB_ISSUER, JWKS_SUFFIX)
}
pub const ERR_AUTH_HEADER_REQUIRED: &str = "Authorization header is required";
pub const STATUS_UNAUTHORIZED: u16 = 401;
pub const ERR_INVALID_OR_EXPIRED_TOKEN: &str = "Invalid or expired token";
pub const STATUS_FORBIDDEN: u16 = 403;
pub const ERR_USER_NO_WRITE_PERMISSIONS: &str = "User does not have write permissions";
pub fn err_failed_exchange(owner: &str, repo: &str) -> String {
    format!("Failed to exchange GitHub App token for {}/{}", owner, repo)
}
pub const STATUS_BAD_GATEWAY: u16 = 502;

// ---------------------------------------------------------------------------
// Service descriptor (mirrors Hono app + DurableObject)
// ---------------------------------------------------------------------------

/// Mirrors `export class SyncServer extends DurableObject<Env>`.
#[derive(Clone, Debug, PartialEq)]
pub struct SyncServer;

impl SyncServer {
    pub const SERVICE_NAME: &'static str = "SyncServer";
    pub const DURABLE_OBJECT_NAME: &'static str = "SyncServer";
}

/// Mirrors default `Hono<{ Bindings: Env }>` app — route table in source order.
#[derive(Clone, Debug, PartialEq)]
pub struct Api;

impl Api {
    pub const ROUTES: &'static [&'static str] = &[
        ROUTE_ROOT,
        ROUTE_SHARE_CREATE,
        ROUTE_SHARE_DELETE,
        ROUTE_SHARE_DELETE_ADMIN,
        ROUTE_SHARE_SYNC,
        ROUTE_SHARE_POLL,
        ROUTE_SHARE_DATA,
        ROUTE_FEISHU,
        ROUTE_EXCHANGE_GITHUB_APP_TOKEN,
        ROUTE_EXCHANGE_GITHUB_APP_TOKEN_WITH_PAT,
        ROUTE_GET_GITHUB_APP_INSTALLATION,
        ROUTE_FALLBACK,
    ];
}

// ---------------------------------------------------------------------------
// PROVISIONAL stubs inventory (flagged pending runtime crates)
// ---------------------------------------------------------------------------

/// PROVISIONAL: `cloudflare:workers` `DurableObject`/`DurableObjectNamespace`/`R2Bucket`/`WebSocketPair`.
pub mod workers_provisional {
    pub const DURABLE_OBJECT_MODULE: &str = "cloudflare:workers";
    pub const R2_BUCKET_TYPE: &str = "R2Bucket";
}

/// PROVISIONAL: `hono` `Hono` app.
pub mod hono_provisional {
    pub const PACKAGE: &str = "hono";
}

/// PROVISIONAL: `sst` `Resource.ADMIN_SECRET` / `Resource.DISCORD_SUPPORT_CHANNEL_ID` / `Resource.DISCORD_SUPPORT_BOT_TOKEN` / `Resource.GITHUB_APP_ID` / `Resource.GITHUB_APP_PRIVATE_KEY`.
pub mod sst_provisional {
    pub const RESOURCE_ADMIN_SECRET: &str = "Resource.ADMIN_SECRET";
    pub const RESOURCE_DISCORD_CHANNEL: &str = "Resource.DISCORD_SUPPORT_CHANNEL_ID";
    pub const RESOURCE_DISCORD_TOKEN: &str = "Resource.DISCORD_SUPPORT_BOT_TOKEN";
    pub const RESOURCE_GITHUB_APP_ID: &str = "Resource.GITHUB_APP_ID";
    pub const RESOURCE_GITHUB_APP_PRIVATE_KEY: &str = "Resource.GITHUB_APP_PRIVATE_KEY";
}

/// PROVISIONAL: `jose` `jwtVerify`/`createRemoteJWKSet`.
pub mod jose_provisional {
    pub const PACKAGE: &str = "jose";
    pub const JWKS_FN: &str = "createRemoteJWKSet";
    pub const VERIFY_FN: &str = "jwtVerify";
}

/// PROVISIONAL: `@octokit/*` `createAppAuth`/`Octokit`.
pub mod octokit_provisional {
    pub const AUTH_APP_PACKAGE: &str = "@octokit/auth-app";
    pub const REST_PACKAGE: &str = "@octokit/rest";
}
