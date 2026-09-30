//! Port of packages/app/src/components/status-popover-indicator.ts
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HasAttentionInput {
    pub mcp: Vec<String>,
}

pub fn has_service_needing_attention(input: &HasAttentionInput) -> bool {
    input
        .mcp
        .iter()
        .any(|s| s == "needs_auth" || s == "needs_client_registration")
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HasIssueInput {
    pub mcp: Vec<String>,
    pub lsp: Vec<String>,
}

pub fn has_non_blocking_service_issue(input: &HasIssueInput) -> bool {
    input
        .mcp
        .iter()
        .any(|s| s != "connected" && s != "pending" && s != "disabled")
        || input.lsp.iter().any(|s| s == "error")
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DotClassInput {
    pub ready: bool,
    pub server_health: Option<bool>,
    pub attention: Option<bool>,
    pub issue: bool,
}

pub fn server_status_dot_class(input: &DotClassInput) -> &'static str {
    if input.server_health == Some(false) {
        return "bg-icon-critical-base";
    }
    if !input.ready || input.server_health.is_none() {
        return "bg-border-weak-base";
    }
    if input.attention == Some(true) {
        return "bg-v2-background-bg-accent";
    }
    if input.issue {
        return "bg-icon-warning-base";
    }
    if input.server_health == Some(true) {
        return "bg-icon-success-base";
    }
    "bg-border-weak-base"
}
