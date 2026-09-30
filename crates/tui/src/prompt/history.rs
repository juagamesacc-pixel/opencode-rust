// source: packages/tui/src/prompt/history.tsx (111 lines, v1.18.30)
// 1:1 port — JSONL history (tail-50 parse, duplicate reset, trim
// rewrite/append rules) and the index-walk `move` semantics verbatim
// (negative indices into the past, `0` = fresh input).

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

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

/// Entry cap verbatim.
pub const MAX_HISTORY_ENTRIES: usize = 50;
/// Filename verbatim.
pub const HISTORY_FILENAME: &str = "prompt-history.jsonl";

/// Mirrors `parsePromptHistory` — bad lines dropped, tail 50 kept.
pub fn parse_prompt_history(text: &str) -> Vec<PromptInfo> {
    let entries: Vec<PromptInfo> = text
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();
    entries
        .into_iter()
        .rev()
        .take(MAX_HISTORY_ENTRIES)
        .rev()
        .collect::<Vec<_>>()
}

/// Mirrors `isDuplicateEntry` (JSON comparison).
pub fn is_duplicate_entry(previous: Option<&PromptInfo>, next: &PromptInfo) -> bool {
    match previous {
        None => false,
        Some(previous) => serde_json::to_string(previous).ok() == serde_json::to_string(next).ok(),
    }
}

/// Prompt history store (mirrors the context value).
pub struct PromptHistory {
    file: PathBuf,
    index: i64,
    history: Vec<PromptInfo>,
}

impl PromptHistory {
    pub fn new(state_dir: &str) -> Self {
        Self {
            file: PathBuf::from(state_dir).join(HISTORY_FILENAME),
            index: 0,
            history: Vec::new(),
        }
    }

    /// Mount load + self-heal rewrite (mirrors `onMount`).
    pub async fn load(&mut self) {
        let text = tokio::fs::read_to_string(&self.file)
            .await
            .unwrap_or_default();
        let lines = parse_prompt_history(&text);
        self.history = lines;
        if !self.history.is_empty() {
            self.rewrite().await;
        }
    }

    fn serialize(&self) -> String {
        self.history
            .iter()
            .map(|line| serde_json::to_string(line).unwrap_or_default())
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    }

    async fn rewrite(&self) {
        let _ = crate::util::persistence::write_text(&self.file, &self.serialize()).await;
    }

    /// Mirrors `move` — walks into the past with negative indices.
    /// Returns `None` when there is nothing to show.
    pub fn move_history(&mut self, direction: i64, input: &str) -> Option<PromptInfo> {
        if self.history.is_empty() {
            return None;
        }
        let current = self.at(self.index)?;
        if current.input != input && !input.is_empty() {
            return None;
        }
        let next = self.index + direction;
        if next.unsigned_abs() as usize > self.history.len() {
            return None;
        }
        if next > 0 {
            return None;
        }
        self.index = next;
        if self.index == 0 {
            return Some(PromptInfo {
                input: String::new(),
                parts: Vec::new(),
                mode: None,
            });
        }
        self.at(self.index).cloned()
    }

    fn at(&self, index: i64) -> Option<&PromptInfo> {
        if index > 0 {
            return None;
        }
        let position = self.history.len() as i64 + index;
        if position < 0 {
            return None;
        }
        self.history.get(position as usize)
    }

    /// Mirrors `append` (duplicate resets the index; trim rewrites).
    pub async fn append(&mut self, item: PromptInfo) {
        if is_duplicate_entry(self.history.last(), &item) {
            self.index = 0;
            return;
        }
        self.history.push(item.clone());
        let mut trimmed = false;
        if self.history.len() > MAX_HISTORY_ENTRIES {
            self.history = self
                .history
                .split_off(self.history.len() - MAX_HISTORY_ENTRIES);
            trimmed = true;
        }
        self.index = 0;
        if trimmed {
            self.rewrite().await;
            return;
        }
        let _ = crate::util::persistence::append_text(
            &self.file,
            &(serde_json::to_string(&item).unwrap_or_default() + "\n"),
        )
        .await;
    }

    pub fn entries(&self) -> &[PromptInfo] {
        &self.history
    }
}
