// source: src/share/share-next.ts — exports: Api, Req, Share, Interface,
// Service, use, node, ShareNext
// PROVISIONAL pending crates/core (layer-node, database, share/sql,
// provider, model, event, service-use) + @opencode-ai/sdk/v2 + @/*:
// endpoint table, key() mapping, enterprise fallback, messages verbatim.

use serde::{Deserialize, Serialize};

/// source: OPENCODE_DISABLE_SHARE env values — verbatim ("true" | "1").
pub const DISABLE_ENV: &str = "OPENCODE_DISABLE_SHARE";

/// source: Api { create, sync(), remove(), data() } — verbatim paths.
#[derive(Debug, Clone)]
pub struct Api {
    pub create: String,
}

impl Api {
    /// source: api(resource) — verbatim path templates.
    pub fn new(resource: &str) -> Self {
        Self {
            create: format!("/api/{}", resource),
        }
    }
    pub fn sync(&self, share_id: &str) -> String {
        format!("{}/{}/sync", self.create, share_id)
    }
    pub fn remove(&self, share_id: &str) -> String {
        format!("{}/{}", self.create, share_id)
    }
    pub fn data(&self, share_id: &str) -> String {
        format!("{}/{}/data", self.create, share_id)
    }
}

/// source: legacyApi = api("share"), consoleApi = api("shares") — verbatim.
pub const LEGACY_RESOURCE: &str = "share";
pub const CONSOLE_RESOURCE: &str = "shares";

/// source: enterprise fallback baseUrl "https://opncd.ai" — verbatim.
pub const FALLBACK_BASE_URL: &str = "https://opncd.ai";

/// source: Share { id, url, secret } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Share {
    pub id: String,
    pub url: String,
    pub secret: String,
}

/// source: disabled share triple { id: "", url: "", secret: "" } — verbatim.
pub fn disabled_share() -> Share {
    Share {
        id: String::new(),
        url: String::new(),
        secret: String::new(),
    }
}

/// source: Data type keys — verbatim key() mapping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    Session,
    Message,
    Part,
    SessionDiff,
    Model,
}

/// source: key(item) — verbatim.
pub fn data_key(kind: &DataType, message_id: Option<&str>, part_id: Option<&str>) -> String {
    match kind {
        DataType::Session => "session".to_string(),
        DataType::Message => format!("message/{}", message_id.unwrap_or("")),
        DataType::Part => format!(
            "part/{}/{}",
            message_id.unwrap_or(""),
            part_id.unwrap_or("")
        ),
        DataType::SessionDiff => "session_diff".to_string(),
        DataType::Model => "model".to_string(),
    }
}

/// source: flush delay 1000 — verbatim.
pub const FLUSH_DELAY_MS: u64 = 1000;

/// source: "No active account token available for sharing" — verbatim.
pub const NO_TOKEN_MESSAGE: &str = "No active account token available for sharing";

/// source: "x-org-id" header — verbatim.
pub const ORG_HEADER: &str = "x-org-id";

/// source: log strings — verbatim.
pub const LOG_FLUSH_FAILED: &str = "share flush failed";
pub const LOG_SUBSCRIBER_FAILED: &str = "share subscriber failed";
pub const LOG_FULL_SYNC: &str = "full sync";
pub const LOG_CREATING: &str = "creating share";
pub const LOG_REMOVING: &str = "removing share";
pub const LOG_SYNC_FAILED: &str = "failed to sync share";
pub const LOG_FULL_FAILED: &str = "share full sync failed";

/// source: Interface — init/url/request/create/remove, verbatim.
pub trait Interface {
    fn init(&self);
    fn url(&self) -> Result<String, String>;
    fn create(&self, session_id: &str) -> Result<Share, String>;
    fn remove(&self, session_id: &str) -> Result<(), String>;
}

/// source: Service "@opencode/ShareNext" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/ShareNext";

/// source: node deps — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[
    "@/account/account.Account",
    "@/event-v2-bridge.EventV2Bridge",
    "@/config/config.Config",
    "@opencode-ai/core/database/database.Database",
    "@opencode-ai/core/effect/app-node-platform.httpClient",
    "@/provider/provider.Provider",
    "@/session/session.Session",
];
