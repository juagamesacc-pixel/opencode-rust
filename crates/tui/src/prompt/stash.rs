// source: packages/tui/src/prompt/stash.tsx (89 lines, v1.18.30)
// 1:1 port — JSONL prompt stash verbatim (tail-50 parse, push trim,
// pop/remove rewrite rules, timestamping).

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

/// Entry cap verbatim.
pub const MAX_STASH_ENTRIES: usize = 50;
/// Filename verbatim.
pub const STASH_FILENAME: &str = "prompt-stash.jsonl";

/// Mirrors `StashEntry`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StashEntry {
    #[serde(default)]
    pub input: String,
    #[serde(default)]
    pub parts: Vec<Value>,
    #[serde(default)]
    pub timestamp: i64,
}

/// Mirrors `parsePromptStash` — bad lines dropped, tail 50 kept.
pub fn parse_prompt_stash(text: &str) -> Vec<StashEntry> {
    let entries: Vec<StashEntry> = text
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();
    entries
        .into_iter()
        .rev()
        .take(MAX_STASH_ENTRIES)
        .rev()
        .collect::<Vec<_>>()
}

/// Prompt stash store (mirrors the context value).
pub struct PromptStash {
    file: PathBuf,
    entries: Vec<StashEntry>,
}

impl PromptStash {
    pub fn new(state_dir: &str) -> Self {
        Self {
            file: PathBuf::from(state_dir).join(STASH_FILENAME),
            entries: Vec::new(),
        }
    }

    /// Mount load + compaction rewrite (mirrors `onMount`).
    pub async fn load(&mut self) {
        let text = tokio::fs::read_to_string(&self.file)
            .await
            .unwrap_or_default();
        let lines = parse_prompt_stash(&text);
        self.entries = lines;
        if !self.entries.is_empty() {
            self.rewrite().await;
        }
    }

    fn serialize(&self) -> String {
        if self.entries.is_empty() {
            return String::new();
        }
        self.entries
            .iter()
            .map(|line| serde_json::to_string(line).unwrap_or_default())
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    }

    async fn rewrite(&self) {
        let _ = crate::util::persistence::write_text(&self.file, &self.serialize()).await;
    }

    pub fn list(&self) -> &[StashEntry] {
        &self.entries
    }

    /// Mirrors `push` (timestamped; trim rewrites, else appends).
    pub async fn push(&mut self, input: String, parts: Vec<Value>, now_ms: i64) {
        let entry = StashEntry {
            input,
            parts,
            timestamp: now_ms,
        };
        self.entries.push(entry.clone());
        if self.entries.len() > MAX_STASH_ENTRIES {
            self.entries = self
                .entries
                .split_off(self.entries.len() - MAX_STASH_ENTRIES);
            self.rewrite().await;
            return;
        }
        let _ = crate::util::persistence::append_text(
            &self.file,
            &(serde_json::to_string(&entry).unwrap_or_default() + "\n"),
        )
        .await;
    }

    /// Mirrors `pop` (last entry out, file rewritten).
    pub async fn pop(&mut self) -> Option<StashEntry> {
        if self.entries.is_empty() {
            return None;
        }
        let entry = self.entries.pop();
        self.rewrite().await;
        entry
    }

    /// Mirrors `remove` (out-of-range is a no-op, file rewritten).
    pub async fn remove(&mut self, index: usize) {
        if index >= self.entries.len() {
            return;
        }
        self.entries.remove(index);
        self.rewrite().await;
    }
}
