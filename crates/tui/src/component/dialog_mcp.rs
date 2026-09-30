// source: packages/tui/src/component/dialog-mcp.tsx (85 lines, v1.18.30)
// 1:1 port — name-sorted MCP options with failed/status descriptions and
// `✓ Enabled` / `○ Disabled` / `⋯ Loading` footers; selection never
// closes; toggle guarded by the in-flight loader.

#![allow(dead_code)]

use crate::ui::dialog_select::{SelectOption, SelectState};

/// Footer glyphs verbatim.
pub const MCP_ENABLED_FOOTER: &str = "✓ Enabled";
pub const MCP_DISABLED_FOOTER: &str = "○ Disabled";
pub const MCP_LOADING_FOOTER: &str = "⋯ Loading";

/// One MCP row input.
#[derive(Debug, Clone)]
pub struct McpRow {
    pub name: String,
    pub status: String,
    pub enabled: bool,
}

/// Build MCP options (name-sorted, verbatim descriptions/footers).
pub fn mcp_options(rows: &[McpRow], loading: Option<&str>) -> Vec<SelectOption> {
    let mut sorted = rows.to_vec();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));
    sorted
        .into_iter()
        .map(|row| {
            let footer = if Some(row.name.as_str()) == loading {
                MCP_LOADING_FOOTER.to_string()
            } else if row.enabled {
                MCP_ENABLED_FOOTER.to_string()
            } else {
                MCP_DISABLED_FOOTER.to_string()
            };
            SelectOption {
                title: row.name.clone(),
                description: Some(if row.status == "failed" {
                    "failed".to_string()
                } else {
                    row.status.clone()
                }),
                footer: Some(footer),
                value: serde_json::Value::String(row.name),
                ..SelectOption::default()
            }
        })
        .collect()
}

/// Build the MCP select state (title `MCPs`, verbatim).
pub fn mcp_state(rows: &[McpRow], loading: Option<&str>) -> SelectState {
    SelectState::new("MCPs", mcp_options(rows, loading))
}
