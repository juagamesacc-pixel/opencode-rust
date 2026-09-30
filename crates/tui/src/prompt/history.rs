// source: packages/tui/src/prompt/history.tsx (PromptInfo shape only —
// full module lands in the prompt batch; this seeds the shared type so
// context/route links today).

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Mirrors the `mode` field of `PromptInfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PromptMode {
    Normal,
    Shell,
}

/// Mirrors `PromptInfo` — parts stay `Value` (file/agent/text shapes).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PromptInfo {
    #[serde(default)]
    pub input: String,
    #[serde(default)]
    pub mode: Option<PromptMode>,
    #[serde(default)]
    pub parts: Vec<Value>,
}
