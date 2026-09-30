//! Port of packages/app/src/components/titlebar-session-events.ts
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub const SESSION_TABS_REMOVED_EVENT: &str = "opencode:session-tabs-removed";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionTabsRemovedDetail {
    pub server: Option<String>,
    pub directory: String,
    pub session_ids: Vec<String>,
}

pub fn read_session_tabs_removed_detail(
    detail: &serde_json::Value,
) -> Option<SessionTabsRemovedDetail> {
    let obj = detail.as_object()?;
    let directory = obj.get("directory")?.as_str()?.to_string();
    let session_ids = obj
        .get("sessionIDs")?
        .as_array()?
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect::<Vec<_>>();
    if session_ids.is_empty() {
        return None;
    }
    let server = obj
        .get("server")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    Some(SessionTabsRemovedDetail {
        server,
        directory,
        session_ids,
    })
}
