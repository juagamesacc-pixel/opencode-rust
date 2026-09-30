//! Port of `packages/schema/src/mcp-event.ts`.
//!
//! Source exports: `ToolsChanged`, `BrowserOpenFailed`, `Definitions`
//! (`mcp.tools.changed` `{server}`, `mcp.browser.open.failed`
//! `{mcpName, url}`). Uses the `Event.define` namespace form in source.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// Payload of `mcp.tools.changed`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolsChangedPayload {
    pub server: String,
}

/// `mcp.tools.changed` definition marker.
pub struct ToolsChanged;
impl ToolsChanged {
    pub const TYPE: &'static str = "mcp.tools.changed";
}

/// Payload of `mcp.browser.open.failed`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrowserOpenFailedPayload {
    pub mcpName: String,
    pub url: String,
}

/// `mcp.browser.open.failed` definition marker.
pub struct BrowserOpenFailed;
impl BrowserOpenFailed {
    pub const TYPE: &'static str = "mcp.browser.open.failed";
}

/// Verbatim declaration order: `ToolsChanged`, `BrowserOpenFailed`.
pub const Definitions: &[&'static str] = &[ToolsChanged::TYPE, BrowserOpenFailed::TYPE];
