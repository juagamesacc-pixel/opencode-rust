// source: packages/tui/src/routes/session/sidebar.tsx (103 lines, v1.18.30)
// 1:1 port — 42-wide session panel descriptor: title, channel-gated
// session id, workspace label (unknown/error fallback), share URL,
// and the footer brand line (`• OpenCode {version}`).

#![allow(dead_code)]

use serde_json::Value;

/// Sidebar width verbatim.
pub const SIDEBAR_WIDTH: u16 = 42;

/// Sidebar title block descriptor.
#[derive(Debug, Clone)]
pub struct SidebarTitle {
    pub title: String,
    pub show_session_id: bool,
    pub session_id: String,
    pub workspace: Option<SidebarWorkspace>,
    pub share_url: Option<String>,
}

/// Workspace label descriptor (mirrors the Show/fallback chain).
#[derive(Debug, Clone)]
pub struct SidebarWorkspace {
    pub label_type: String,
    pub name: String,
    pub status: String,
}

/// Build the title block (channel gate + workspace fallback verbatim).
pub fn sidebar_title(
    session: &Value,
    session_id: &str,
    installation_channel: &str,
    workspace: Option<&Value>,
    workspace_status: Option<&str>,
) -> SidebarTitle {
    let workspace = session
        .get("workspaceID")
        .and_then(|v| v.as_str())
        .map(|id| match workspace {
            Some(workspace) => SidebarWorkspace {
                label_type: workspace
                    .get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                name: workspace
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                status: workspace_status.unwrap_or("error").to_string(),
            },
            None => SidebarWorkspace {
                label_type: "unknown".to_string(),
                name: id.to_string(),
                status: "error".to_string(),
            },
        });
    SidebarTitle {
        title: session
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        show_session_id: installation_channel != "latest",
        session_id: session_id.to_string(),
        workspace,
        share_url: session
            .get("share")
            .and_then(|s| s.get("url"))
            .and_then(|v| v.as_str())
            .map(str::to_string),
    }
}

/// Footer brand segments (`•`, `Open`, `Code`, version).
#[derive(Debug, Clone)]
pub struct SidebarFooter {
    pub version: String,
}

pub fn sidebar_footer(version: &str) -> SidebarFooter {
    SidebarFooter {
        version: version.to_string(),
    }
}
