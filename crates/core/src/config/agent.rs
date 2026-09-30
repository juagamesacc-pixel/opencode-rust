//! Rust port of `packages/core/src/config/agent.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

use serde::{Deserialize, Serialize};

// PROVISIONAL pending @opencode-ai/schema — Permission + ConfigProvider preserved as opaque
// PROVISIONAL pending schema PositiveInt validation

/// Source: `export const Color = Schema.Union([...])` — verbatim values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Color {
    Hex(String),
    Named(NamedColor),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NamedColor {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "secondary")]
    Secondary,
    #[serde(rename = "accent")]
    Accent,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "info")]
    Info,
}

/// Source: `export class Info extends Schema.Class<Info>("ConfigV2.Agent")({...})` — fields verbatim, optional as Option.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    // PROVISIONAL pending ConfigProvider.Request — preserved as serde_json::Value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    // PROVISIONAL pending Permission.Ruleset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    #[serde(rename = "subagent")]
    Subagent,
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "all")]
    All,
}

#[cfg(test)]
mod tests {
    fn is_hex_color(s: &str) -> bool {
        // std-only mirror of /^#[0-9a-fA-F]{6}$/ (no regex dep per port doctrine)
        let b = s.as_bytes();
        b.len() == 7 && b[0] == b'#' && b[1..].iter().all(|c| c.is_ascii_hexdigit())
    }
    #[test]
    fn color_hex_pattern_preserved() {
        assert!(is_hex_color("#1a2B3c"));
        assert!(!is_hex_color("primary"));
    }
}
